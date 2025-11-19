use crate::controllers::error::handle_client_error;
use crate::controllers::movies::pagination::Pagination;
use crate::models::movies::WrapperMovieDetail;
use crate::{
    controllers::movies::client_config,
    models::movies::{WrapperMovie, WrapperNewMovie},
};
use actix_web::{delete, get, post, put, web};
use movies_client::apis::{configuration::Configuration, movies_api};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

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
pub async fn create_movie(
    new_movie: web::Json<WrapperNewMovie>,
) -> Result<web::Json<WrapperMovie>, actix_web::Error> {
    let config = client_config();
    let new_movie = new_movie.into_inner();

    match movies_api::create_movie(&config, new_movie.into()).await {
        Ok(movie) => Ok(web::Json(movie.into())),
        Err(err) => Err(handle_client_error(err, "Creating movie")),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to retrieve")
    ),
    responses(
        (status = 200, description = "Get movie by ID", body = WrapperMovie),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies/{movie_id}")]
pub async fn get_movie_by_id(
    movie_id: web::Path<Uuid>,
) -> Result<web::Json<WrapperMovie>, actix_web::Error> {
    let config = client_config();

    match movies_api::get_movie_by_id(&config, &movie_id.clone().to_string()).await {
        Ok(movie) => Ok(web::Json(movie.into())),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Fetching movie {}", movie_id),
        )),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to delete")
    ),
    responses(
        (status = 200, description = "Delete movie by ID", body = usize),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/movies/{movie_id}")]
pub async fn delete_movie(movie_id: web::Path<Uuid>) -> Result<web::Json<i32>, actix_web::Error> {
    let config = client_config();

    match movies_api::delete_movie(&config, &movie_id.clone().to_string()).await {
        Ok(deleted) => Ok(web::Json(deleted)),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Deleting movie {}", movie_id),
        )),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to update")
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
    movie_id: web::Path<Uuid>,
    updated_movie: web::Json<WrapperNewMovie>,
) -> Result<web::Json<WrapperMovie>, actix_web::Error> {
    let config = Configuration::default();
    let updated_movie = updated_movie.into_inner();

    match movies_api::update_movie(&config, &movie_id.clone().to_string(), updated_movie.into())
        .await
    {
        Ok(movie) => Ok(web::Json(movie.into())),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Updating movie {}", movie_id),
        )),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to retrieve details for")
    ),
    responses(
        (status = 200, description = "Get movie details by ID", body = WrapperMovieDetail),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies/{movie_id}/details")]
pub async fn get_movie_details_by_id(
    movie_id: web::Path<Uuid>,
) -> Result<web::Json<WrapperMovieDetail>, actix_web::Error> {
    let config = client_config();
    let movie_id = movie_id.into_inner().to_string();

    match movies_api::get_movie_details_by_id(&config, &movie_id).await {
        Ok(movie) => Ok(web::Json(movie.into())),
        Err(err) => Err(handle_client_error(err, "Searching movies by title")),
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
        (status = 200, description = "Search movies by title", body = [WrapperMovie]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/movies/title")]
pub async fn search_movies_by_title(
    query: web::Query<TitleSearchQuery>,
) -> Result<web::Json<Vec<WrapperMovie>>, actix_web::Error> {
    let config = client_config();

    match movies_api::search_movies_by_title(
        &config,
        &query.title_name,
        query.limit.unwrap_or(100),
        query.offset.unwrap_or(0),
    )
    .await
    {
        Ok(movies) => Ok(web::Json(
            movies.into_iter().map(|e| WrapperMovie::from(e)).collect(),
        )),
        Err(err) => Err(handle_client_error(err, "Searching movies by title")),
    }
}

#[derive(serde::Deserialize)]
pub struct ActorSearchQuery {
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
        (status = 200, description = "Search movies by actor", body = [WrapperMovie]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/movies/person")]
pub async fn search_movies_by_actor(
    person_name: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<WrapperMovie>>, actix_web::Error> {
    let config = client_config();

    match movies_api::search_movies_by_actor(
        &config,
        &person_name.clone(),
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(movies) => Ok(web::Json(
            movies.into_iter().map(|e| WrapperMovie::from(e)).collect(),
        )),
        Err(err) => Err(handle_client_error(err, "Searching movies by title")),
    }
}

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of movies to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Get all movies", body = [WrapperMovie]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/movies")]
pub async fn get_all_movies(
    pagination: web::Query<Pagination>,
) -> anyhow::Result<web::Json<Vec<WrapperMovie>>, actix_web::Error> {
    let config = client_config();

    match movies_api::get_all_movies(
        &config,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(movies) => Ok(web::Json(movies.into_iter().map(WrapperMovie::from).collect())),
        Err(err) => Err(handle_client_error(err, "Searching movies by title")),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    create_movie,
    get_movie_by_id,
    delete_movie,
    update_movie,
    search_movies_by_title,
    search_movies_by_actor,
    get_movie_details_by_id,
    get_all_movies,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(create_movie);
    cfg.service(get_movie_by_id);
    cfg.service(delete_movie);
    cfg.service(update_movie);
    cfg.service(search_movies_by_title);
    cfg.service(search_movies_by_actor);
    cfg.service(get_movie_details_by_id);
    cfg.service(get_all_movies);
}
