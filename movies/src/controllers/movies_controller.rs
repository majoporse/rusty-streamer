use crate::controllers::models::movies::movie::Movie;
use crate::controllers::models::movies::new_movie::NewMovie;
use crate::controllers::pagination::Pagination;
use crate::{controllers::error, services::movie_service::MovieService};
use actix_web::{delete, get, post, put, web, Result};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

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
    movie_service: web::Data<MovieService>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<Movie>>, actix_web::Error> {
    match movie_service.list_movies(
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    ).await {
        Ok(movies) => Ok(web::Json(
            movies.into_iter().map(|m| Movie::from(m)).collect(),
        )),
        Err(e) => Err(error::handle_db_error(&e, "get_all_movies")),
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
    movie_service: web::Data<MovieService>,
    new_movie: web::Json<NewMovie>,
) -> Result<web::Json<Movie>, actix_web::Error> {
    match movie_service.create_movie(new_movie.into_inner().into()).await {
        Ok(movie) => Ok(web::Json(Movie::from(movie))),
        Err(e) => Err(crate::controllers::error::handle_db_error(
            &e,
            "create_movie",
        )),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to retrieve")
    ),
    responses(
        (status = 200, description = "Get movie by ID", body = Movie),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies/{movie_id}")]
pub async fn get_movie_by_id(
    movie_service: web::Data<MovieService>,
    movie_id: web::Path<Uuid>,
) -> Result<web::Json<Movie>, actix_web::Error> {
    match movie_service.get_by_id(movie_id.into_inner()).await {
        Ok(movie) => Ok(web::Json(Movie::from(movie))),
        Err(e) => Err(error::handle_db_error(&e, "get_movie_by_id")),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to delete")
    ),
    responses(
        (status = 200, description = "Delete movie by ID", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/movies/{movie_id}")]
pub async fn delete_movie(
    movie_service: web::Data<MovieService>,
    movie_id: web::Path<Uuid>,
) -> Result<web::Json<usize>, actix_web::Error> {
    match movie_service.delete_movie(*movie_id).await {
        Ok(deleted_rows) => Ok(web::Json(deleted_rows)),
        Err(e) => Err(error::handle_db_error(&e, "delete_movie")),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to update")
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
    movie_service: web::Data<MovieService>,
    movie_id: web::Path<Uuid>,
    updated_movie: web::Json<NewMovie>,
) -> Result<web::Json<Movie>, actix_web::Error> {
    match movie_service.update_movie(*movie_id, updated_movie.into_inner().into()).await {
        Ok(movie) => Ok(web::Json(Movie::from(movie))),
        Err(e) => Err(error::handle_db_error(&e, "update_movie")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct ActorSearchQuery {
    pub actor_name: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
#[utoipa::path(
    params(
        ("actor_name" = String, Query, description = "Actor name to search for movies", example = "Leonardo DiCaprio"),
        ("limit" = i64, Query, description = "Max number of movies to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Search movies by actor", body = [Movie]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/movies/people")]
pub async fn search_movies_by_actor(
    movie_service: web::Data<MovieService>,
    query: web::Query<ActorSearchQuery>,
) -> Result<web::Json<Vec<Movie>>, actix_web::Error> {
    match movie_service.search_by_actor(
        &query.actor_name,
        query.limit.unwrap_or(100),
        query.offset.unwrap_or(0),
    ).await {
        Ok(movies) => Ok(web::Json(
            movies.into_iter().map(|m| Movie::from(m)).collect(),
        )),
        Err(e) => Err(error::handle_db_error(&e, "search_movies_by_actor")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct TitleSearchQuery {
    pub title_name: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[utoipa::path(
    params(
        ("title_name" = String, Query, description = "Title query string to search for movies", example = "Inception"),
        ("limit" = i64, Query, description = "Max number of movies to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Search movies by title", body = [Movie]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/movies/title")]
pub async fn search_movies_by_title(
    movie_service: web::Data<MovieService>,
    query: web::Query<TitleSearchQuery>,
) -> Result<web::Json<Vec<Movie>>, actix_web::Error> {
    match movie_service.search_by_title(
        &query.title_name,
        query.limit.unwrap_or(100),
        query.offset.unwrap_or(0),
    ).await {
        Ok(movies) => Ok(web::Json(
            movies.into_iter().map(|m| Movie::from(m)).collect(),
        )),
        Err(e) => Err(error::handle_db_error(&e, "search_movies_by_title")),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to retrieve details for")
    ),
    responses(
        (status = 200, description = "Get movie details by ID", body = crate::controllers::models::movies::movie_detail::MovieDetail),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies/{movie_id}/details")]
pub async fn get_movie_details_by_id(
    movie_service: web::Data<MovieService>,
    movie_id: web::Path<Uuid>,
) -> Result<web::Json<crate::controllers::models::movies::movie_detail::MovieDetail>, actix_web::Error> {
    match movie_service.get_movie_details_by_id(movie_id.into_inner()).await {
        Ok(movie_details) => Ok(web::Json(movie_details.into())),
        Err(e) => Err(error::handle_db_error(&e, "get_movie_details_by_id")),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_all_movies,
    get_movie_by_id,
    create_movie,
    delete_movie,
    update_movie,
    search_movies_by_title,
    search_movies_by_actor,
    get_movie_details_by_id,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig){
    cfg.service(get_all_movies);
    cfg.service(get_movie_by_id);
    cfg.service(create_movie);
    cfg.service(delete_movie);
    cfg.service(update_movie);
    cfg.service(search_movies_by_title);
    cfg.service(search_movies_by_actor);
    cfg.service(get_movie_details_by_id);
}
