use std::path::PathBuf;
use std::sync::Arc;

use actix_multipart::form::MultipartFormConfig;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
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

use crate::models::DbConnection;

pub mod controllers;
pub mod data;
pub mod models;
pub mod schema;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Video Server API",
        version = "1.0.0",
        description = "API documentation for my video server."
    )
)]
struct ApiDoc;

pub fn get_connection_pool() -> anyhow::Result<Pool<ConnectionManager<DbConnection>>> {
    log::info!("Setting up database connection pool...");
    let url = std::env::var("USERS_DB_STRING").expect("USERS_DB_STRING must be set");

    let manager = ConnectionManager::<DbConnection>::new(url);

    Ok(Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "debug,opentelemetry=debug,opentelemetry_otlp=debug",
    );
    dotenv().ok();
    env_logger::init();

    let port = std::env::var("USERS_PORT")
        .expect("USERS_PORT is not defined")
        .parse::<u16>()
        .expect("USERS_PORT is not defined");

    let mut apidoc = ApiDoc::openapi();

    apidoc.merge(controllers::users::ApiDoc::openapi());
    apidoc.merge(controllers::watch_history::ApiDoc::openapi());
    apidoc.merge(controllers::watch_list::ApiDoc::openapi());
    apidoc.merge(controllers::watch_room::ApiDoc::openapi());
    apidoc.merge(controllers::watch_room_participant::ApiDoc::openapi());
    apidoc.merge(controllers::watch_room_message::ApiDoc::openapi());

    let pool = Arc::new(get_connection_pool().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("DB Pool Error: {}", e))
    })?);


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
            .configure(controllers::users::scoped_config)
            .configure(controllers::watch_history::scoped_config)
            .configure(controllers::watch_list::scoped_config)
            .configure(controllers::watch_room_participant::scoped_config)
            .configure(controllers::watch_room::scoped_config)
            .configure(controllers::watch_room_message::scoped_config)
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
                .with_attributes([KeyValue::new("service.name", "users")])
                .build(),
        )
        .build();

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(
            Resource::builder_empty()
                .with_attributes([KeyValue::new("service.name", "users")])
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
