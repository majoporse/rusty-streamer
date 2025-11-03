use actix_multipart::form::MultipartFormConfig;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "movies API",
        version = "1.0.0",
        description = "API documentation for my movies server."
    ),
    paths(
    ),
    tags(
        (name = "movies server", description = "Movies Server API Documentation")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    dotenv().ok();

    env_logger::init();

    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::new("%a \"%r\" %s %b \"%{User-Agent}i\" %T")) // ✅ proper placement
            .app_data(
                MultipartFormConfig::default()
                    .total_limit(10 * 1024 * 1024 * 1024) // 10 GB
                    .memory_limit(10 * 1024 * 1024), // 10 MB
            )
            .into_utoipa_app()
            // services
            // .configure(|cfg| controllers::blob::scoped_config(cfg))
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
