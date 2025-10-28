use actix_multipart::form::MultipartFormConfig;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

mod controllers;
mod logic;


#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "video server", description = "Video Server API Documentation")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    dotenv().ok();

    env_logger::init(); 

    let state = web::Data::new(controllers::test::AppState {
        app_name: Mutex::new(String::from("Video Server")),
    });

    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")) // ✅ proper placement
            .app_data(state.clone())
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1024 * 1024) // 10 MB
            )
            .into_utoipa_app()
            // services
            .configure(|cfg| controllers::test::scoped_config(cfg))
            .configure(|cfg| controllers::blob::scoped_config(cfg))
            // OpenAPI docs
            .openapi(ApiDoc::openapi())
            .openapi_service(|api| Redoc::with_url("/redoc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .openapi_service(|api| RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app();
        app
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
