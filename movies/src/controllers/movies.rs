use std::sync::Arc;

use crate::controllers::error;
use crate::data::movies;
use crate::models::{
    movie::{Movie, NewMovie},
    DbConnection,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;

static TAG: &str = "Movies";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of movies to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all movies", body = [Movie]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies")]
pub async fn get_all_movies(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    limit: web::Query<Option<i64>>,
    offset: web::Query<Option<i64>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match movies::list_movies(&mut db_conn, limit.into_inner().unwrap_or(100), offset.into_inner().unwrap_or(0)) {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(e) => return error::handle_db_error(&e, "get_all_movies"),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to retrieve")
    ),
    responses(
        (status = 200, description = "Get movie by ID", body = Movie),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies/{movie_id}")]
pub async fn get_movie_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    movie_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match movies::get_movie_by_id(&mut db_conn, *movie_id) {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(e) => error::handle_db_error(&e, "get_movie_by_id"),
    }
}

#[utoipa::path(
    request_body = NewMovie,
    responses(
        (status = 200, description = "Create a new movie", body = Movie),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/movies")]
pub async fn create_movie(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_movie: web::Json<NewMovie>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match movies::create_movie(&mut db_conn, new_movie.into_inner()) {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(e) =>  crate::controllers::error::handle_db_error(&e, "create_movie"),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to delete")
    ),
    responses(
        (status = 200, description = "Delete movie by ID", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/movies/{movie_id}")]
pub async fn delete_movie(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    movie_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match movies::delete_movie(&mut db_conn, *movie_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_movie"),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to update")
    ),
    request_body = NewMovie,
    responses(
        (status = 200, description = "Update movie by ID", body = Movie),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/movies/{movie_id}")]
pub async fn update_movie(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    movie_id: web::Path<i32>,
    updated_movie: web::Json<NewMovie>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match movies::update_movie(&mut db_conn, *movie_id, updated_movie.into_inner()) {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(e) => error::handle_db_error(&e, "update_movie"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_all_movies,
    get_movie_by_id,
    create_movie,
    delete_movie,
    update_movie,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_all_movies);
    cfg.service(get_movie_by_id);
    cfg.service(create_movie);
    cfg.service(delete_movie);
    cfg.service(update_movie);
}
