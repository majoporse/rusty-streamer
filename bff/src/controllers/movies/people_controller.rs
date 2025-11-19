use crate::controllers::error::handle_client_error;
use crate::{
    controllers::{movies::client_config, users::pagination::Pagination},
    models::movies::{WrapperPerson, WrapperNewPerson},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use movies_client::apis::people_api;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

static TAG: &str = "People";

#[utoipa::path(
    responses(
        (status = 200, description = "List all people", body = [WrapperPerson]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/people")]
pub async fn get_all_people(pagination: web::Query<Pagination>) -> Result<web::Json<Vec<WrapperPerson>>, actix_web::Error> {
    let config = client_config();

    match people_api::get_all_people(
        &config,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(people) => Ok(web::Json(people.into_iter().map(|p| p.into()).collect())),
        Err(err) => Err(handle_client_error(err, "Fetching all people")),
    }
}

#[utoipa::path(
    request_body = WrapperNewPerson,
    responses(
        (status = 200, description = "Create a new person", body = WrapperPerson),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/people")]
pub async fn create_person(new_person: web::Json<WrapperNewPerson>) -> Result<web::Json<WrapperPerson>, actix_web::Error> {
    let config = client_config();
    let new_person = new_person.into_inner();

    match people_api::create_person(&config, new_person.into()).await {
        Ok(person) => Ok(web::Json(person.into())),
        Err(err) => Err(handle_client_error(err, "Creating person")),
    }
}

#[utoipa::path(
    params(
        ("person_id" = Uuid, Path, description = "ID of the person to retrieve")
    ),
    responses(
        (status = 200, description = "Get person by ID", body = WrapperPerson),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/people/{person_id}")]
pub async fn get_person_by_id(person_id: web::Path<Uuid>) -> Result<web::Json<WrapperPerson>, actix_web::Error> {
    let config = client_config();
    let person_id = person_id.into_inner().to_string();

    match people_api::get_person_by_id(&config, &person_id).await {
        Ok(person) => Ok(web::Json(person.into())),
        Err(err) => Err(handle_client_error(err, &format!("Fetching person {}", person_id))),
    }
}

#[utoipa::path(
    params(
        ("person_id" = Uuid, Path, description = "ID of the person to delete")
    ),
    responses(
        (status = 200, description = "Delete person by ID", body = usize),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/people/{person_id}")]
pub async fn delete_person(person_id: web::Path<Uuid>) -> Result<web::Json<i32>, actix_web::Error> {
    let config = client_config();
    let person_id = person_id.into_inner().to_string();

    match people_api::delete_person(&config, &person_id).await {
        Ok(deleted) => Ok(web::Json(deleted)),
        Err(err) => Err(handle_client_error(err, &format!("Deleting person {}", person_id))),
    }
}

#[utoipa::path(
    params(
        ("person_id" = Uuid, Path, description = "ID of the person to update")
    ),
    request_body = WrapperNewPerson,
    responses(
        (status = 200, description = "Update person by ID", body = WrapperPerson),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/people/{person_id}")]
pub async fn update_person(
    person_id: web::Path<Uuid>,
    updated_person: web::Json<WrapperNewPerson>,
) -> Result<HttpResponse, actix_web::Error> {
    let config = client_config();
    let updated_person = updated_person.into_inner();
    let person_id = person_id.into_inner().to_string();

    match people_api::update_person(&config, &person_id, updated_person.into()).await {
        Ok(person) => Ok(HttpResponse::Ok().json(person)),
        Err(err) => Err(handle_client_error(err, &format!("Updating person {}", person_id))),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to get people for")
    ),
    responses(
        (status = 200, description = "Get people by movie ID", body = [WrapperPerson]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/people/movie/{movie_id}")]
pub async fn get_person_by_movie_id(movie_id: web::Path<Uuid>) -> Result<HttpResponse, actix_web::Error> {
    let config = client_config();
    let movie_id = movie_id.into_inner().to_string();

    match people_api::get_person_by_movie_id(&config, &movie_id).await {
        Ok(person) => Ok(HttpResponse::Ok().json(person)),
        Err(err) => Err(handle_client_error(err, &format!("Fetching people for movie {}", movie_id))),
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SearchPeopleByNameParams {
    pub name: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
#[utoipa::path(
    params(
        ("name" = String, Query, description = "Name of the person to search for"),
        ("limit" = i64, Query, description = "Max number of people to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Get persons by name", body = [WrapperPerson]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/people/name")]
pub async fn get_person_by_name(
    params: web::Query<SearchPeopleByNameParams>,
) -> Result<web::Json<Vec<WrapperPerson>>, actix_web::Error> {
    let config = client_config();
    let name = params.name.clone();
    match people_api::get_person_by_name(&config, &name, params.limit.unwrap_or(100), params.offset.unwrap_or(0)).await {
        Ok(person) => Ok(web::Json(person.into_iter().map(|p| p.into()).collect())),
        Err(err) => Err(handle_client_error(err, &format!("Fetching people by name {}", name))),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_all_people,
    get_person_by_id,
    create_person,
    delete_person,
    update_person,
    get_person_by_movie_id,
    get_person_by_name,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_all_people);
    cfg.service(get_person_by_id);
    cfg.service(create_person);
    cfg.service(delete_person);
    cfg.service(update_person);
    cfg.service(get_person_by_movie_id);
    cfg.service(get_person_by_name);
}
