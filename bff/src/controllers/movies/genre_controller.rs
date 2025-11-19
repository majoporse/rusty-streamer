use crate::{
    controllers::{
        error::{self, handle_client_error},
        movies::{client_config, pagination::Pagination},
    },
    models::movies::{WrapperGenre, WrapperNewGenre},
};

use actix_web::{delete, get, post, put, web};
use movies_client::apis::genres_api;
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "Genres";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of genres to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all genres", body = [WrapperGenre]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/genres")]
pub async fn list_genres(
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<WrapperGenre>>, actix_web::Error> {
    let config = client_config();

    match genres_api::list_genres(
        &config,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(genres) => Ok(web::Json(genres.into_iter().map(|e| e.into()).collect())),
        Err(err) => Err(handle_client_error(err, "Creating movie")),
    }
}

#[utoipa::path(
    params(
        ("genre_id" = Uuid, Path, description = "ID of the genre to retrieve")
    ),
    responses(
        (status = 200, description = "Get genre by ID", body = WrapperGenre),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/genres/{genre_id}")]
pub async fn get_genre_by_id(
    genre_id: web::Path<Uuid>,
) -> Result<web::Json<WrapperGenre>, actix_web::Error> {
    let config = client_config();

    match genres_api::get_genre_by_id(&config, &genre_id.to_string()).await {
        Ok(g) => Ok(web::Json(g.into())),
        Err(err) => Err(handle_client_error(err, "get_genre_by_id")),
    }
}

#[utoipa::path(
    request_body = WrapperNewGenre,
    responses(
        (status = 200, description = "Create a new genre", body = WrapperGenre),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/genres")]
pub async fn create_genre(
    new_genre: web::Json<WrapperNewGenre>,
) -> Result<web::Json<WrapperGenre>, actix_web::Error> {
    let config = client_config();

    match genres_api::create_genre(&config, new_genre.into_inner().into()).await {
        Ok(g) => Ok(web::Json(g.into())),
        Err(err) => Err(handle_client_error(err, "create_genre")),
    }
}

#[utoipa::path(
    params(
        ("genre_id" = Uuid, Path, description = "ID of the genre to delete")
    ),
    responses(
        (status = 200, description = "Delete genre by ID", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/genres/{genre_id}")]
pub async fn delete_genre(genre_id: web::Path<Uuid>) -> Result<web::Json<i32>, actix_web::Error> {
    let config = client_config();

    match genres_api::delete_genre(&config, &genre_id.to_string()).await {
        Ok(count) => Ok(web::Json(count)),
        Err(err) => Err(handle_client_error(err, "delete_genre")),
    }
}

#[utoipa::path(
    params(
        ("genre_id" = Uuid, Path, description = "ID of the genre to update")
    ),
    request_body = WrapperNewGenre,
    responses(
        (status = 200, description = "Update genre by ID", body = WrapperGenre),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/genres/{genre_id}")]
pub async fn update_genre(
    genre_id: web::Path<Uuid>,
    updated_genre: web::Json<WrapperNewGenre>,
) -> Result<web::Json<WrapperGenre>, actix_web::Error> {
    let config = client_config();

    match genres_api::update_genre(&config, &genre_id.to_string(), updated_genre.into_inner().into()).await {
        Ok(g) => Ok(web::Json(g.into())),
        Err(err) => Err(handle_client_error(err, "update_genre")),
    }
}

#[derive(OpenApi)]
#[openapi(paths(list_genres, get_genre_by_id, create_genre, delete_genre, update_genre,))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list_genres);
    cfg.service(get_genre_by_id);
    cfg.service(create_genre);
    cfg.service(delete_genre);
    cfg.service(update_genre);
}
