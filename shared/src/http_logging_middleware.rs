use http::Extensions;
use opentelemetry::{KeyValue, global, trace::{Span, Tracer}};
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let tracer = global::tracer("otlp_metrics_logger");

        let method = req.method().clone();
        let path = req.url().path().to_string();

        let mut span = tracer.start(format!("HTTP {} {}", method.to_string(), path));

        span.set_attribute(KeyValue::new("method", method.to_string()));
        span.set_attribute(KeyValue::new("url", req.url().to_string()));

        let res = next.run(req, extensions).await;

        span.set_attribute(KeyValue::new(
            "status_code",
            res.as_ref()
                .map(|r| r.status().as_u16().to_string())
                .unwrap_or_else(|e| format!("error: {}", e)),
        ));

        println!("Result: {:?}", res);
        res
    }
}
