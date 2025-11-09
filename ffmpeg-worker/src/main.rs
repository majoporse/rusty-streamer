use std::path::PathBuf;

use actix_multipart::form::MultipartFormConfig;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use log::info;
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::Tracer as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{Protocol, WithExportConfig as _};
use opentelemetry_sdk::Resource;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

use shared::log_middleware::OtlpMetricsLogger;

mod controllers;
mod logic;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Video Server API",
        version = "1.0.0",
        description = "API documentation for my video server."
    ),
    paths(
        controllers::blob::upload,
    ),
    tags(
        (name = "video server", description = "Video Server API Documentation")
    )
)]
struct ApiDoc;
async fn index() -> &'static str {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "debug,opentelemetry=debug,opentelemetry_otlp=debug",
    );
    dotenv().ok();

    env_logger::init();

    save_openapi_spec().await?;

    setup_otel().await?;

    let aa = global::tracer("main_tracer");
    let mut span = aa.start("aaaaaa");
    let a = global::tracer("asdfas");
    a.in_span("aaaaaaaa", |_cx| {
        // Your application logic here...
        let b = global::tracer("bbbbbb");
        b.in_span("bbbbbbb", |_cx| {
            // Nested span logic...
        });
    });
    span.end();

    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")) // ✅ proper placement
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1024 * 1024), // 10 MB
            )
            .wrap(OtlpMetricsLogger::new())
            .service(web::resource("/").to(index))
            .into_utoipa_app()
            // services
            .configure(controllers::blob::scoped_config)
            // OpenAPI docs
            .openapi(ApiDoc::openapi())
            .openapi_service(|api| Redoc::with_url("/redoc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .openapi_service(|_| RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app();
        app
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

async fn setup_otel() -> std::io::Result<()> {
    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        // .with_http_client(reqwest::Client::new())
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        // .with_endpoint("http://localhost:4318/v1/metrics")
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", "ffmpeg-worker")])
                .build(),
        )
        .build();

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", "ffmpeg-worker")])
                .build(),
        )
        .build();

    global::set_meter_provider(meter_provider);
    global::set_tracer_provider(tracer_provider);



    Ok(())
}

async fn save_openapi_spec() -> std::io::Result<()> {
    // save the api doc to a file
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let location = PathBuf::from(root).join("api-docs").join("openapi.json");

    tokio::fs::write(&location, ApiDoc::openapi().to_pretty_json()?).await?;
    info!("Wrote OpenAPI doc to {:?}", location);
    Ok(())
}
