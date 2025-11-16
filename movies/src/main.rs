use std::path::PathBuf;
use std::sync::Arc;

use actix_multipart::form::MultipartFormConfig;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use dotenvy::dotenv;
use log::info;
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


pub mod controllers;
pub mod data;
pub mod dtos;
pub mod schema;
pub mod services;

#[derive(OpenApi)]
#[openapi(info(
    title = "Movies API",
    version = "1.0.0",
    description = "API documentation for my movies server."
))]
struct ApiDoc;

pub async fn get_connection_pool() -> Pool<AsyncPgConnection> {
    log::info!("Setting up database connection pool...");
    let url = std::env::var("MOVIES_DB_STRING").expect("MOVIES_DB_STRING must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(url);
    let pool = Pool::builder()
        .build(config)
        .await
        .expect("Could not build connection pool");
    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,diesel=debug");
    dotenv().ok();
    env_logger::init();

    let port = std::env::var("MOVIES_PORT")
        .expect("MOVIES_PORT is not defined")
        .parse::<u16>()
        .expect("MOVIES_PORT is not defined");

    let mut apidoc = ApiDoc::openapi();

    apidoc.merge(controllers::people_controller::ApiDoc::openapi());
    apidoc.merge(controllers::movies_controller::ApiDoc::openapi());
    apidoc.merge(controllers::reviews_controller::ApiDoc::openapi());

    let pool = Arc::new(get_connection_pool().await);

    save_openapi_spec(&apidoc).await?;
    log::info!("setup openapi");

    setup_otel().await?;
    log::info!("otel setup");

    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")) // ✅ proper placement
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1024 * 1024), // 10 MB
            )
            .wrap(OtlpMetricsLogger::new())
            .into_utoipa_app()
            .app_data(Data::new(pool.clone()))
            // services
            .app_data(Data::new(services::movie_service::MovieService::new(
                pool.clone(),
            )))
            .app_data(Data::new(services::review_service::ReviewService::new(
                pool.clone(),
            )))
            .app_data(Data::new(services::people_service::PeopleService::new(
                pool.clone(),
            )))
            // controllers
            .configure(controllers::people_controller::scoped_config)
            .configure(controllers::movies_controller::scoped_config)
            .configure(controllers::reviews_controller::scoped_config)
            // OpenAPI docs
            .openapi(apidoc.clone())
            .openapi_service(|api| Redoc::with_url("/redoc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .openapi_service(|_| RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app();
        app
    })
    .bind(("127.0.0.1", port))?
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
                .with_attributes([KeyValue::new("service.name", "movies")])
                .build(),
        )
        .build();

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", "movies")])
                .build(),
        )
        .build();

    global::set_meter_provider(meter_provider);
    global::set_tracer_provider(tracer_provider);

    Ok(())
}

async fn save_openapi_spec(apidoc: &utoipa::openapi::OpenApi) -> std::io::Result<()> {
    // save the api doc to a file
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let location = PathBuf::from(root).join("api-docs").join("openapi.json");
    std::fs::create_dir_all(&location.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "parent not found",
    ))?)?;

    tokio::fs::write(&location, apidoc.to_pretty_json()?).await?;
    info!("Wrote OpenAPI doc to {:?}", location);
    Ok(())
}
