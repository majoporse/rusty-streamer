use crate::controllers::error;
use crate::controllers::pagination::Pagination;
use crate::services::genre_service::GenreService;

use crate::controllers::models::genre::genre::Genre;
use crate::controllers::models::genre::new_genre::NewGenre;

use actix_web::{delete, get, post, put, web};
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "Genres";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of genres to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all genres", body = [Genre]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/genres")]
pub async fn list_genres(
    genre_service: web::Data<GenreService>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<Genre>>, actix_web::Error> {
    match genre_service
        .list_genres(
            pagination.limit.unwrap_or(100),
            pagination.offset.unwrap_or(0),
        )
        .await
    {
        Ok(genres) => Ok(web::Json(genres.into_iter().map(Genre::from).collect())   ),
        Err(e) => Err(error::handle_db_error(&e, "list_genres")),
    }
}

#[utoipa::path(
    params(
        ("genre_id" = Uuid, Path, description = "ID of the genre to retrieve")
    ),
    responses(
        (status = 200, description = "Get genre by ID", body = Genre),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/genres/{genre_id}")]
pub async fn get_genre_by_id(
    genre_service: web::Data<GenreService>,
    genre_id: web::Path<Uuid>,
) -> Result<web::Json<Genre>, actix_web::Error> {
    match genre_service.get_by_id(*genre_id).await {
        Ok(genre) => Ok(web::Json(genre.into())),
        Err(e) => Err(error::handle_db_error(&e, "get_genre_by_id")),
    }
}

#[utoipa::path(
    request_body = NewGenre,
    responses(
        (status = 200, description = "Create a new genre", body = Genre),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/genres")]
pub async fn create_genre(
    genre_service: web::Data<GenreService>,
    new_genre: web::Json<NewGenre>,
) -> Result<web::Json<Genre>, actix_web::Error> {
    match genre_service.create_genre(new_genre.into_inner().into()).await {
        Ok(genre) => Ok(web::Json(genre.into())),
        Err(e) => Err(error::handle_db_error(&e, "create_genre")),
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
pub async fn delete_genre(
    genre_service: web::Data<GenreService>,
    genre_id: web::Path<Uuid>,
) -> Result<web::Json<usize>, actix_web::Error> {
    match genre_service.delete_genre(*genre_id).await {
        Ok(count) => Ok(web::Json(count)),
        Err(e) => Err(error::handle_db_error(&e, "delete_genre")),
    }
}

#[utoipa::path(
    params(
        ("genre_id" = Uuid, Path, description = "ID of the genre to update")
    ),
    request_body = NewGenre,
    responses(
        (status = 200, description = "Update genre by ID", body = Genre),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/genres/{genre_id}")]
pub async fn update_genre(
    genre_service: web::Data<GenreService>,
    genre_id: web::Path<Uuid>,
    updated_genre: web::Json<NewGenre>,
) -> Result<web::Json<Genre>, actix_web::Error> {
    match genre_service
        .update_genre(*genre_id, updated_genre.into_inner().into())
        .await
    {
        Ok(genre) => Ok(web::Json(genre.into())),
        Err(e) => Err(error::handle_db_error(&e, "update_genre")),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    list_genres,
    get_genre_by_id,
    create_genre,
    delete_genre,
    update_genre,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list_genres);
    cfg.service(get_genre_by_id);
    cfg.service(create_genre);
    cfg.service(delete_genre);
    cfg.service(update_genre);
}
