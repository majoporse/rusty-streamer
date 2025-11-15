use crate::controllers::users::client_config;
use crate::controllers::users::error::handle_client_error;
use crate::controllers::users::pagination::Pagination;
use crate::models::users::{Watchlist, NewWatchlist};

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use utoipa::OpenApi;
use uuid::Uuid;

// Generated OpenAPI client
use users_client::apis::watchlist_api;

static TAG: &str = "Watchlist";

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user whose watchlist to retrieve"),
        ("limit" = i64, Query, description = "Max number of items to return", example = 50),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List user watchlist", body = [Watchlist]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users/{user_id}/watchlist")]
pub async fn get_watchlist_by_user(
    user_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let config = client_config();

    // Validate UUID
    let parsed_user_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for user_id"),
    };

    match watchlist_api::get_watchlist_by_user(
        &config,
        &parsed_user_id.to_string(),
        pagination.limit.unwrap_or(50),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => handle_client_error(err, "Fetching watchlist by user"),
    }
}

#[utoipa::path(
    request_body = NewWatchlist,
    responses(
        (status = 200, description = "Add a new item to the watchlist", body = Watchlist),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/watchlist")]
pub async fn create_watchlist_item(
    new_item: web::Json<NewWatchlist>,
) -> impl Responder {
    let config = client_config();

    match watchlist_api::create_watchlist_item(&config, new_item.into_inner().into()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => handle_client_error(err, "Creating watchlist item"),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user"),
        ("content_id" = String, Path, description = "UUID of the content item")
    ),
    responses(
        (status = 200, description = "Delete item from watchlist", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/users/{user_id}/watchlist/{content_id}")]
pub async fn delete_watchlist_item(
    path: web::Path<(String, String)>,
) -> impl Responder {
    let config = client_config();
    let (user_id_str, content_id_str) = path.into_inner();

    // Validate UUIDs
    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for user_id"),
    };

    let content_id = match Uuid::parse_str(&content_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for content_id"),
    };

    match watchlist_api::delete_watchlist_item(
        &config,
        &user_id.to_string(),
        &content_id.to_string(),
    )
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => handle_client_error(err, "Deleting watchlist item"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_watchlist_by_user,
    create_watchlist_item,
    delete_watchlist_item,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_watchlist_by_user);
    cfg.service(create_watchlist_item);
    cfg.service(delete_watchlist_item);
}
