use std::sync::Arc;

use crate::controllers::error;
use crate::controllers::pagination::Pagination;
use crate::data::watch_history;
use crate::models::watch_history::{WatchHistory, NewWatchHistory, UpdateWatchHistory};
use crate::models::DbConnection;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "Watch History";

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user whose history to retrieve"),
        ("limit" = i64, Query, description = "Max number of items to return", example = 50),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List user watch history", body = [WatchHistory]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users/{user_id}/watch-history")]
pub async fn list_watch_history_by_user(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    user_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    match watch_history::list_watch_history_by_user(
        &mut db_conn,
        parsed_id,
        pagination.limit.unwrap_or(50),
        pagination.offset.unwrap_or(0),
    ) {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => error::handle_db_error(&e, "list_watch_history_by_user"),
    }
}

#[utoipa::path(
    params(
        ("id" = i64, Path, description = "ID of the watch history entry")
    ),
    responses(
        (status = 200, description = "Get watch history by ID", body = WatchHistory),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-history/{id}")]
pub async fn get_watch_history_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    id: web::Path<i64>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_history::get_watch_history_by_id(&mut db_conn, *id) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => error::handle_db_error(&e, "get_watch_history_by_id"),
    }
}

#[utoipa::path(
    request_body = NewWatchHistory,
    responses(
        (status = 200, description = "Create a new watch history entry", body = WatchHistory),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/watch-history")]
pub async fn create_watch_history(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_item: web::Json<NewWatchHistory>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_history::create_watch_history(&mut db_conn, new_item.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => error::handle_db_error(&e, "create_watch_history"),
    }
}

#[utoipa::path(
    params(
        ("id" = i64, Path, description = "ID of the watch history entry to update")
    ),
    request_body = UpdateWatchHistory,
    responses(
        (status = 200, description = "Update watch history entry", body = WatchHistory),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/watch-history/{id}")]
pub async fn update_watch_history(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    id: web::Path<i64>,
    update_data: web::Json<UpdateWatchHistory>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_history::update_watch_history(&mut db_conn, *id, update_data.into_inner()) {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => error::handle_db_error(&e, "update_watch_history"),
    }
}

#[utoipa::path(
    params(
        ("id" = i64, Path, description = "ID of the watch history entry to delete")
    ),
    responses(
        (status = 200, description = "Delete watch history entry", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-history/{id}")]
pub async fn delete_watch_history(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    id: web::Path<i64>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_history::delete_watch_history(&mut db_conn, *id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_watch_history"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    list_watch_history_by_user,
    get_watch_history_by_id,
    create_watch_history,
    update_watch_history,
    delete_watch_history,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list_watch_history_by_user);
    cfg.service(get_watch_history_by_id);
    cfg.service(create_watch_history);
    cfg.service(update_watch_history);
    cfg.service(delete_watch_history);
}
