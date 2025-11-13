use std::sync::Arc;

use crate::controllers::error;
use crate::data::watch_list;
use crate::models::watch_list::{NewWatchlist, Watchlist};
use crate::models::DbConnection;

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
use uuid::Uuid;

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
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    user_id: web::Path<String>,
    web::Query(pagination): web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_user_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for user_id"),
    };

    let limit = pagination
        .get("limit")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(50);
    let offset = pagination
        .get("offset")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(0);

    match watch_list::get_watchlist_by_user(&mut db_conn, parsed_user_id, limit, offset) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => error::handle_db_error(&e, "get_watchlist_by_user"),
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
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_item: web::Json<NewWatchlist>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_list::create_watchlist_item(&mut db_conn, new_item.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => error::handle_db_error(&e, "create_watchlist_item"),
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
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");
    let (user_id_str, content_id_str) = path.into_inner();

    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for user_id"),
    };

    let content_id = match Uuid::parse_str(&content_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for content_id"),
    };

    match watch_list::delete_watchlist_item(&mut db_conn, user_id, content_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_watchlist_item"),
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
