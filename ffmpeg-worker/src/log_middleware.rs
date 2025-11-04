use std::{
    future::{ready, Ready},
    time::Instant,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use opentelemetry::{
    global::{self, ObjectSafeSpan},
    metrics::{Counter, Histogram, Meter},
    trace::Tracer,
    KeyValue,
};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct OtlpMetricsLogger {
    request_counter: Counter<u64>,
    latency_histogram: Histogram<f64>,
}

impl OtlpMetricsLogger {
    pub fn new() -> Self {
        let meter: Meter = global::meter("my_service");

        let request_counter = meter
            .u64_counter("http_requests_total")
            .with_description("Total HTTP requests")
            .build();

        let latency_histogram = meter
            .f64_histogram("http_request_duration_seconds")
            .with_description("HTTP request duration in seconds")
            .build();

        Self {
            request_counter,
            latency_histogram,
        }
    }
}
// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for OtlpMetricsLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OtlpMetricsLoggerService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(OtlpMetricsLoggerService {
            service: service,
            request_counter: self.request_counter.clone(),
            latency_histogram: self.latency_histogram.clone(),
        }))
    }
}

pub struct OtlpMetricsLoggerService<S> {
    service: S,
    request_counter: Counter<u64>,
    latency_histogram: Histogram<f64>,
}

impl<S, B> Service<ServiceRequest> for OtlpMetricsLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let start = Instant::now();

        let counter = self.request_counter.clone();
        let histogram = self.latency_histogram.clone();

        let tracer = global::tracer("otlp_metrics_logger");
        let mut span = tracer.start(format!("HTTP {} {}", method, path));

        span.set_attribute(KeyValue::new("method", method.to_string()));
        span.set_attribute(KeyValue::new("path", path.clone()));

        log::debug!("Started {} {}", method, path);
        let fut = self.service.call(req);

        span.end();

        Box::pin(async move {
            let res = fut.await?;

            let duration = start.elapsed().as_secs_f64();

            // Log request info
            log::info!("{} {} -> {} ({:.3}s)", method, path, res.status(), duration);

            // Record metrics
            counter.add(
                1,
                &[
                    opentelemetry::KeyValue::new("method", method.to_string()),
                    opentelemetry::KeyValue::new("path", path.clone()),
                    opentelemetry::KeyValue::new("status", res.status().as_u16().to_string()),
                ],
            );

            histogram.record(
                duration,
                &[
                    opentelemetry::KeyValue::new("method", method.to_string()),
                    opentelemetry::KeyValue::new("path", path.clone()),
                    opentelemetry::KeyValue::new("status", res.status().as_u16().to_string()),
                ],
            );

            Ok(res)
        })
    }
}
