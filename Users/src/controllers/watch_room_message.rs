
use std::sync::Arc;

use crate::controllers::error;
use crate::data::watch_room_message;
use crate::models::watch_room_message::{NewWatchRoomMessage, WatchRoomMessage};
use crate::models::DbConnection;

use actix_web::{get, post, delete, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "WatchRoomMessages";

#[utoipa::path(
    params(
        ("message_id" = i64, Path, description = "ID of the message to retrieve")
    ),
    responses(
        (status = 200, description = "Get a watch room message by ID", body = WatchRoomMessage),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-room-messages/{message_id}")]
pub async fn get_watch_room_message_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    message_id: web::Path<i64>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_room_message::get_watch_room_message_by_id(&mut db_conn, *message_id) {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(e) => error::handle_db_error(&e, "get_watch_room_message_by_id"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room whose messages to retrieve"),
        ("limit" = i64, Query, description = "Max number of messages to return", example = 50),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List messages by room", body = [WatchRoomMessage]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-rooms/{room_id}/messages")]
pub async fn list_messages_by_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
    web::Query(pagination): web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    let limit = pagination
        .get("limit")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(50);
    let offset = pagination
        .get("offset")
        .and_then(|v| v.parse::<i64>().ok())
        .unwrap_or(0);

    match watch_room_message::list_messages_by_room(&mut db_conn, parsed_room_id, limit, offset) {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => error::handle_db_error(&e, "list_messages_by_room"),
    }
}

#[utoipa::path(
    request_body = NewWatchRoomMessage,
    responses(
        (status = 200, description = "Create a new message in a watch room", body = WatchRoomMessage),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/watch-room-messages")]
pub async fn create_watch_room_message(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_message: web::Json<NewWatchRoomMessage>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_room_message::create_watch_room_message(&mut db_conn, new_message.into_inner()) {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(e) => error::handle_db_error(&e, "create_watch_room_message"),
    }
}

#[utoipa::path(
    params(
        ("message_id" = i64, Path, description = "ID of the message to delete")
    ),
    responses(
        (status = 200, description = "Delete a message from a watch room", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-room-messages/{message_id}")]
pub async fn delete_watch_room_message(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    message_id: web::Path<i64>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_room_message::delete_watch_room_message(&mut db_conn, *message_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_watch_room_message"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room whose messages to delete")
    ),
    responses(
        (status = 200, description = "Delete all messages from a watch room", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-rooms/{room_id}/messages")]
pub async fn delete_messages_by_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_message::delete_messages_by_room(&mut db_conn, parsed_room_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_messages_by_room"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_watch_room_message_by_id,
    list_messages_by_room,
    create_watch_room_message,
    delete_watch_room_message,
    delete_messages_by_room,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_watch_room_message_by_id);
    cfg.service(list_messages_by_room);
    cfg.service(create_watch_room_message);
    cfg.service(delete_watch_room_message);
    cfg.service(delete_messages_by_room);
}
