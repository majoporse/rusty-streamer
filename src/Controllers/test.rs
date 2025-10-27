use actix_web::{get, post, put, web, HttpResponse};
use std::fs::Metadata;
use std::sync::Mutex;
use utoipa_actix_web::service_config::ServiceConfig;
use utoipa_redoc::Config;

#[utoipa::path(
    responses(
        (status = 200, description = "Index endpoint", body = String)
    )
)]
#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name.lock().unwrap(); // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[utoipa::path(
    request_body = String,
    responses(
        (status = 200, description = "Change app name")
    )
)]
#[post("/change_name")]
async fn change_name(data: web::Data<AppState>, new_name: String) -> HttpResponse {
    let mut app_name = data.app_name.lock().unwrap();
    *app_name = new_name;
    HttpResponse::Ok().body("App name changed")
}

pub struct AppState {
    pub app_name: Mutex<String>,
}

pub fn scoped_config(cfg: &mut ServiceConfig) {
    cfg.service(index).service(change_name);
}
