use crate::models::wrappers::{WrapperMovie, WrapperNewMovie};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use movies_client::apis::{movies_api, configuration::Configuration};
use utoipa::OpenApi;
use crate::controllers::error::handle_client_error;

static TAG: &str = "Movies";

#[utoipa::path(
    responses(
        (status = 200, description = "Movie created successfully", body = WrapperMovie),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error"),
    ),
    tag = TAG
)]
#[post("/movies")]
pub async fn create_movie(new_movie: web::Json<WrapperNewMovie>) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();
    let new_movie = new_movie.into_inner();

    match movies_api::create_movie(&config, new_movie.into()).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => handle_client_error(err, "Creating movie"),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to retrieve")
    ),
    responses(
        (status = 200, description = "Get movie by ID", body = WrapperMovie),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies/{movie_id}")]
pub async fn get_movie_by_id(movie_id: web::Path<i32>) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();

    match movies_api::get_movie_by_id(&config, movie_id.clone()).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => handle_client_error(err, &format!("Fetching movie {}", movie_id)),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to delete")
    ),
    responses(
        (status = 200, description = "Delete movie by ID", body = usize),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/movies/{movie_id}")]
pub async fn delete_movie(movie_id: web::Path<i32>) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();

    match movies_api::delete_movie(&config, movie_id.clone()).await {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => handle_client_error(err, &format!("Deleting movie {}", movie_id)),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to update")
    ),
    request_body = WrapperNewMovie,
    responses(
        (status = 200, description = "Update movie by ID", body = WrapperMovie),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/movies/{movie_id}")]
pub async fn update_movie(
    movie_id: web::Path<i32>,
    updated_movie: web::Json<WrapperNewMovie>,
) -> impl Responder {
    let config = Configuration::default();
    let updated_movie = updated_movie.into_inner();

    match movies_api::update_movie(&config, movie_id.clone(), updated_movie.into()).await {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => handle_client_error(err, &format!("Updating movie {}", movie_id)),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    create_movie,
    get_movie_by_id,
    delete_movie,
    update_movie,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(create_movie);
    cfg.service(get_movie_by_id);
    cfg.service(delete_movie);
    cfg.service(update_movie);
}
