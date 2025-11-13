use std::sync::Arc;

use crate::controllers::error;
use crate::data::watch_room;
use crate::models::watch_room::{NewWatchRoom, UpdateWatchRoom, WatchRoom};
use crate::models::DbConnection;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "WatchRooms";

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the watch room to retrieve")
    ),
    responses(
        (status = 200, description = "Get watch room by ID", body = WatchRoom),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-rooms/{room_id}")]
pub async fn get_watch_room_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room::get_watch_room_by_id(&mut db_conn, parsed_id) {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(e) => error::handle_db_error(&e, "get_watch_room_by_id"),
    }
}

#[utoipa::path(
    params(
        ("host_user_id" = String, Path, description = "UUID of the user hosting the rooms"),
        ("limit" = i64, Query, description = "Max number of rooms to return", example = 50),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List rooms hosted by user", body = [WatchRoom]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users/{host_user_id}/watch-rooms")]
pub async fn list_rooms_by_host(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    host_user_id: web::Path<String>,
    limit: web::Query<Option<i64>>,
    offset: web::Query<Option<i64>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_user_id = match Uuid::parse_str(&host_user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for host_user_id"),
    };

    match watch_room::list_rooms_by_host(
        &mut db_conn,
        parsed_user_id,
        limit.into_inner().unwrap_or(50),
        offset.into_inner().unwrap_or(0),
    ) {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(e) => error::handle_db_error(&e, "list_rooms_by_host"),
    }
}

#[utoipa::path(
    request_body = NewWatchRoom,
    responses(
        (status = 200, description = "Create a new watch room", body = WatchRoom),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/watch-rooms")]
pub async fn create_watch_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_room: web::Json<NewWatchRoom>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_room::create_watch_room(&mut db_conn, new_room.into_inner()) {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(e) => error::handle_db_error(&e, "create_watch_room"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room to update")
    ),
    request_body = UpdateWatchRoom,
    responses(
        (status = 200, description = "Update an existing watch room", body = WatchRoom),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/watch-rooms/{room_id}")]
pub async fn update_watch_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
    update_data: web::Json<UpdateWatchRoom>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room::update_watch_room(&mut db_conn, parsed_id, update_data.into_inner()) {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(e) => error::handle_db_error(&e, "update_watch_room"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room to delete")
    ),
    responses(
        (status = 200, description = "Delete a watch room", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-rooms/{room_id}")]
pub async fn delete_watch_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room::delete_watch_room(&mut db_conn, parsed_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_watch_room"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_watch_room_by_id,
    list_rooms_by_host,
    create_watch_room,
    update_watch_room,
    delete_watch_room,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_watch_room_by_id);
    cfg.service(list_rooms_by_host);
    cfg.service(create_watch_room);
    cfg.service(update_watch_room);
    cfg.service(delete_watch_room);
}
