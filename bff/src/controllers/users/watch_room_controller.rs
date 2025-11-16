use crate::controllers::users::error::handle_client_error;
use crate::controllers::users::pagination::Pagination;
use crate::controllers::users::{client_config, redis_conn};
use crate::models::users::{NewWatchRoom, UpdateWatchRoom, WatchRoom};

use actix_web::{delete, get, post, put, rt, web, HttpRequest, HttpResponse, Responder};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use redis::AsyncCommands;
use utoipa::OpenApi;
use uuid::Uuid;

use users_client::apis::watch_rooms_api;

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
pub async fn get_watch_room_by_id(room_id: web::Path<String>) -> impl Responder {
    let config = client_config();

    let parsed_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_rooms_api::get_watch_room_by_id(&config, &parsed_id.to_string()).await {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(err) => handle_client_error(err, "Fetching watch room by ID"),
    }
}

#[utoipa::path(
    params(
        ("room_id" = String, Path, description = "UUID of the room to connect to")
    ),
    responses(
        (status = 200, description = "Connect to a watch room"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/watch-rooms/{room_id}/connect")]
pub async fn connect_room(
    room_id: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    let client = redis_conn();

    let room_channel_name = format!("room_{}", room_id.clone());

    let mut sender = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    sender
        .subscribe(&room_channel_name)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut reciever = client
        .get_async_pubsub()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    reciever
        .subscribe(&room_channel_name)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let room_channel_listener = room_channel_name.clone();
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message

                    match sender
                        .publish::<&str, String, ()>(&room_channel_listener, text.to_string())
                        .await
                    {
                        Ok(_) => {}
                        Err(e) => {
                            log::debug!("Redis publish error: {:?}", e);
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    });

    rt::spawn(async move {
        loop {
            let recieved = match reciever.on_message().next().await {
                Some(msg) => msg,
                None => {
                    log::debug!("Redis stream closed");
                    break;
                }
            };

            log::debug!("Received {:?}", recieved);
            let data = recieved
                .get_payload::<String>()
                .expect("unable to parse to string");

            log::debug!("Message: {}", data);

            match session.text(data.clone()).await {
                Ok(_) => {}
                Err(e) => {
                    log::debug!("WebSocket send error: {:?}", e);
                    break;
                }
            }
            if data == "close" {
                break;
            }
        }
    });

    Ok(res)
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
    host_user_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let config = client_config();

    let parsed_user_id = match Uuid::parse_str(&host_user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for host_user_id"),
    };

    match watch_rooms_api::list_rooms_by_host(
        &config,
        &parsed_user_id.to_string(),
        pagination.limit.unwrap_or(50),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(rooms) => HttpResponse::Ok().json(rooms),
        Err(err) => handle_client_error(err, "Listing rooms by host"),
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
pub async fn create_watch_room(new_room: web::Json<NewWatchRoom>) -> impl Responder {
    let config = client_config();

    match watch_rooms_api::create_watch_room(&config, new_room.into_inner().into()).await {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(err) => handle_client_error(err, "Creating watch room"),
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
    room_id: web::Path<String>,
    update_data: web::Json<UpdateWatchRoom>,
) -> impl Responder {
    let config = client_config();

    let parsed_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_rooms_api::update_watch_room(
        &config,
        &parsed_id.to_string(),
        update_data.into_inner().into(),
    )
    .await
    {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(err) => handle_client_error(err, "Updating watch room"),
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
pub async fn delete_watch_room(room_id: web::Path<String>) -> impl Responder {
    let config = client_config();

    let parsed_id = match Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format for room_id"),
    };

    match watch_rooms_api::delete_watch_room(&config, &parsed_id.to_string()).await {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(err) => handle_client_error(err, "Deleting watch room"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_watch_room_by_id,
    list_rooms_by_host,
    create_watch_room,
    update_watch_room,
    delete_watch_room,
    connect_room,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_watch_room_by_id);
    cfg.service(list_rooms_by_host);
    cfg.service(create_watch_room);
    cfg.service(update_watch_room);
    cfg.service(delete_watch_room);
    cfg.service(connect_room);
}
