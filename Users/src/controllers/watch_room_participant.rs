use std::sync::Arc;

use crate::controllers::error;
use crate::data::watch_room_participant;
use crate::models::watch_room_participant::{NewWatchRoomParticipant, WatchRoomParticipant};
use crate::models::DbConnection;

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "WatchRoomParticipants";

#[utoipa::path(
    params(
        ("participant_id" = i64, Path, description = "ID of the participant to retrieve")
    ),
    responses(
        (status = 200, description = "Get a participant by ID", body = WatchRoomParticipant),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-room-participants/{participant_id}")]
pub async fn get_watch_room_participant_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    participant_id: web::Path<i64>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_room_participant::get_watch_room_participant_by_id(&mut db_conn, *participant_id) {
        Ok(participant) => HttpResponse::Ok().json(participant),
        Err(e) => error::handle_db_error(&e, "get_watch_room_participant_by_id"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room to list participants for"),
        ("limit" = i64, Query, description = "Max number of participants to return", example = 50),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List participants by room", body = [WatchRoomParticipant]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-rooms/{room_id}/participants")]
pub async fn list_participants_by_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
    limit: web::Query<Option<i64>>,
    offset: web::Query<Option<i64>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_participant::list_participants_by_room(
        &mut db_conn,
        parsed_room_id,
        limit.into_inner().unwrap_or(50),
        offset.into_inner().unwrap_or(0),
    ) {
        Ok(participants) => HttpResponse::Ok().json(participants),
        Err(e) => error::handle_db_error(&e, "list_participants_by_room"),
    }
}

#[utoipa::path(
    request_body = NewWatchRoomParticipant,
    responses(
        (status = 200, description = "Add a new participant to a watch room", body = WatchRoomParticipant),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/watch-room-participants")]
pub async fn create_watch_room_participant(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_participant: web::Json<NewWatchRoomParticipant>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match watch_room_participant::create_watch_room_participant(
        &mut db_conn,
        new_participant.into_inner(),
    ) {
        Ok(participant) => HttpResponse::Ok().json(participant),
        Err(e) => error::handle_db_error(&e, "create_watch_room_participant"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room"),
        ("user_id" = String, Path, description = "UUID of the participant to remove")
    ),
    responses(
        (status = 200, description = "Delete a participant from a room", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-rooms/{room_id}/participants/{user_id}")]
pub async fn delete_watch_room_participant(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let (room_id_str, user_id_str) = path.into_inner();

    let room_id = match Uuid::parse_str(&room_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for user_id"),
    };

    match watch_room_participant::delete_watch_room_participant(&mut db_conn, room_id, user_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_watch_room_participant"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room whose participants to delete")
    ),
    responses(
        (status = 200, description = "Delete all participants from a watch room", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-rooms/{room_id}/participants")]
pub async fn delete_participants_by_room(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    room_id: web::Path<String>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_participant::delete_participants_by_room(&mut db_conn, parsed_room_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_participants_by_room"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_watch_room_participant_by_id,
    list_participants_by_room,
    create_watch_room_participant,
    delete_watch_room_participant,
    delete_participants_by_room,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_watch_room_participant_by_id);
    cfg.service(list_participants_by_room);
    cfg.service(create_watch_room_participant);
    cfg.service(delete_watch_room_participant);
    cfg.service(delete_participants_by_room);
}
