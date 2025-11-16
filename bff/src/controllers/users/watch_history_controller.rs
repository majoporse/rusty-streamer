use crate::controllers::users::client_config;
use crate::controllers::users::error::handle_client_error;
use crate::controllers::users::pagination::Pagination;
use crate::models::users::{WatchHistory, NewWatchHistory, UpdateWatchHistory};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use utoipa::OpenApi;
use uuid::Uuid;

// The generated OpenAPI client
use users_client::apis::watch_history_api;

static TAG: &str = "Watch History";


#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user whose history to retrieve"),
        ("limit" = i64, Query, description = "Max number of items to return"),
        ("offset" = i64, Query, description = "Pagination offset")
    ),
    responses(
        (status = 200, description = "List user watch history", body = [WatchHistory]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users/{user_id}/watch-history")]
pub async fn list_watch_history_by_user(
    user_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let config = client_config();

    let parsed_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    match watch_history_api::list_watch_history_by_user(
        &config,
        &parsed_id.to_string(),
        pagination.limit.unwrap_or(50),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => handle_client_error(err, "Listing user watch history"),
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
    id: web::Path<i64>,
) -> impl Responder {
    let config = client_config();

    match watch_history_api::get_watch_history_by_id(&config, *id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => handle_client_error(err, &format!("Fetching watch history ID {}", id)),
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
    new_item: web::Json<NewWatchHistory>,
) -> impl Responder {
    let config = client_config();

    match watch_history_api::create_watch_history(&config, new_item.into_inner().into()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => handle_client_error(err, "Creating watch history entry"),
    }
}

#[utoipa::path(
    params(
        ("id" = i64, Path, description = "ID of the watch history entry to update")
    ),
    request_body = UpdateWatchHistory,
    responses(
        (status = 200, description = "Update watch history entry", body = WatchHistory),
        (status = 404, description = "Not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/watch-history/{id}")]
pub async fn update_watch_history(
    id: web::Path<i64>,
    update_data: web::Json<UpdateWatchHistory>,
) -> impl Responder {
    let config = client_config();

    match watch_history_api::update_watch_history(
        &config,
        *id,
        update_data.into_inner().into(),
    )
    .await
    {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => handle_client_error(err, &format!("Updating watch history ID {}", id)),
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
pub async fn delete_watch_history(id: web::Path<i64>) -> impl Responder {
    let config = client_config();

    match watch_history_api::delete_watch_history(&config, *id).await {
        Ok(count) => HttpResponse::Ok().json(count),
        Err(err) => handle_client_error(err, &format!("Deleting watch history ID {}", id)),
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
