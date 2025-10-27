use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::Redoc;
use utoipa_redoc::Servable as _;

use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;
mod controllers;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "video server", description = "Video Server API Documentation")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    let state = web::Data::new(controllers::test::AppState {
        app_name: Mutex::new(String::from("Video Server")),
    });

    HttpServer::new(move || {
        App::new()
            .into_utoipa_app()
            .app_data(state.clone())
            .openapi(ApiDoc::openapi())
            .map(|app| app.wrap(Logger::default()))
            // services
            .configure(|cfg| controllers::test::scoped_config(cfg))
            // openapi spec
            .openapi_service(|api| Redoc::with_url("/redoc", api))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .openapi_service(|api| RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .openapi_service(|api| Scalar::with_url("/scalar", api))
            .into_app()
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
