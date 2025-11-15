use crate::controllers::users::client_config;
use crate::controllers::users::error::handle_client_error;
use crate::controllers::users::pagination::Pagination;
use crate::models::users::{NewWatchRoomParticipant, WatchRoomParticipant};

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use utoipa::OpenApi;
use uuid::Uuid;

// Generated OpenAPI client
use users_client::apis::watch_room_participants_api;

static TAG: &str = "WatchRoomParticipants";


#[utoipa::path(
    params(
        ("participant_id" = i64, Path, description = "ID of the participant to retrieve")
    ),
    responses(
        (status = 200, description = "Get a watch room participant by ID", body = WatchRoomParticipant),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-room-participants/{participant_id}")]
pub async fn get_watch_room_participant_by_id(
    participant_id: web::Path<i64>,
) -> impl Responder {
    let config = client_config();

    match watch_room_participants_api::get_watch_room_participant_by_id(
        &config,
        *participant_id,
    )
    .await
    {
        Ok(participant) => HttpResponse::Ok().json(participant),
        Err(err) => handle_client_error(err, "Fetching watch room participant by ID"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room whose participants to retrieve"),
        ("limit" = i64, Query, description = "Max number of participants", example = 50),
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
    room_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let config = client_config();

    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_participants_api::list_participants_by_room(
        &config,
        &parsed_room_id.to_string(),
        pagination.limit.unwrap_or(50),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(participants) => HttpResponse::Ok().json(participants),
        Err(err) => handle_client_error(err, "Listing participants by room"),
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
    new_participant: web::Json<NewWatchRoomParticipant>,
) -> impl Responder {
    let config = client_config();

    match watch_room_participants_api::create_watch_room_participant(
        &config,
        new_participant.into_inner().into(),
    )
    .await
    {
        Ok(participant) => HttpResponse::Ok().json(participant),
        Err(err) => handle_client_error(err, "Creating watch room participant"),
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
    path: web::Path<(String, String)>,
) -> impl Responder {
    let config = client_config();
    let (room_id_str, user_id_str) = path.into_inner();

    let room_id = match Uuid::parse_str(&room_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for user_id"),
    };

    match watch_room_participants_api::delete_watch_room_participant(
        &config,
        &room_id.to_string(),
        &user_id.to_string(),
    )
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => handle_client_error(err, "Deleting watch room participant"),
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
    room_id: web::Path<String>,
) -> impl Responder {
    let config = client_config();

    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_participants_api::delete_participants_by_room(&config, &parsed_room_id.to_string())
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => handle_client_error(err, "Deleting participants by room"),
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
