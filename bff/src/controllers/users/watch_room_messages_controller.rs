use crate::controllers::users::client_config;
use crate::controllers::users::error::handle_client_error;
use crate::controllers::users::pagination::Pagination;
use crate::models::users::{NewWatchRoomMessage, WatchRoomMessage};

use actix_web::{delete, get, post, web, HttpResponse, Responder};
use utoipa::OpenApi;
use uuid::Uuid;

// Generated OpenAPI client
use users_client::apis::watch_room_messages_api;

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
pub async fn get_watch_room_message_by_id(message_id: web::Path<i64>) -> impl Responder {
    let config = client_config();

    match watch_room_messages_api::get_watch_room_message_by_id(&config, *message_id).await {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(err) => handle_client_error(err, "Fetching watch room message by ID"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room whose messages to retrieve"),
        ("limit" = i64, Query, description = "Max number of messages", example = 50),
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
    room_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let config = client_config();

    // Validate UUID
    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_messages_api::list_messages_by_room(
        &config,
        &parsed_room_id.to_string(),
        pagination.limit.unwrap_or(50),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(err) => handle_client_error(err, "Listing messages by room"),
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
    new_message: web::Json<NewWatchRoomMessage>,
) -> impl Responder {
    let config = client_config();

    match watch_room_messages_api::create_watch_room_message(
        &config,
        new_message.into_inner().into(),
    )
    .await
    {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(err) => handle_client_error(err, "Creating watch room message"),
    }
}

#[utoipa::path(
    params(
        ("message_id" = i64, Path, description = "ID of the message to delete")
    ),
    responses(
        (status = 200, description = "Delete a message", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-room-messages/{message_id}")]
pub async fn delete_watch_room_message(message_id: web::Path<i64>) -> impl Responder {
    let config = client_config();

    match watch_room_messages_api::delete_watch_room_message(&config, *message_id).await {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => handle_client_error(err, "Deleting watch room message"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room whose messages to delete")
    ),
    responses(
        (status = 200, description = "Delete all messages from a room", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/watch-rooms/{room_id}/messages")]
pub async fn delete_messages_by_room(room_id: web::Path<String>) -> impl Responder {
    let config = client_config();

    // Validate UUID
    let parsed_room_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_room_messages_api::delete_messages_by_room(&config, &parsed_room_id.to_string())
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => handle_client_error(err, "Deleting messages by room"),
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
