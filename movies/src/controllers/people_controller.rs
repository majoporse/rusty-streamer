use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::controllers::error;
use crate::controllers::pagination::Pagination;
use crate::services::people_service::PeopleService;
use actix_web::{delete, get, post, put, web};
use utoipa::{OpenApi, ToSchema};
use crate::controllers::models::people::person::Person;
use crate::controllers::models::people::new_person::NewPerson;

static TAG: &str = "People";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of people to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all people", body = [Person]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/people")]
pub async fn get_all_people(
    people_service: web::Data<PeopleService>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<Person>>, actix_web::Error> {

    match people_service.list_people(
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    ).await {
        Ok(persons) => Ok(web::Json(persons.into_iter().map(|p| Person::from(p)).collect())),
        Err(e) => Err(error::handle_db_error(&e, "get_all_persons")),
    }
}

#[utoipa::path(
    request_body = NewPerson,
    responses(
        (status = 200, description = "Create a new person", body = Person),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/people")]
pub async fn create_person(
    people_service: web::Data<PeopleService>,
    new_person: web::Json<NewPerson>,
) -> Result<web::Json<Person>, actix_web::Error> {

    match people_service.create_person(new_person.into_inner().into()).await {
        Ok(person) => Ok(web::Json(Person::from(person))),
        Err(e) => Err(error::handle_db_error(&e, "create_person")),
    }
}

#[utoipa::path(
    params(
        ("person_id" = Uuid, Path, description = "ID of the person to retrieve")
        
    ),
    responses(
        (status = 200, description = "Get person by ID", body = Person),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/people/{person_id}")]
pub async fn get_person_by_id(
    people_service: web::Data<PeopleService>,
    person_id: web::Path<Uuid>,
) -> Result<web::Json<Person>, actix_web::Error> {

    match people_service.get_by_id( *person_id).await {
        Ok(person) => Ok(web::Json(Person::from(person))),
        Err(e) => Err(error::handle_db_error(&e, "get_person_by_id")),
    }
}

#[utoipa::path(
    params(
        ("person_id" = Uuid, Path, description = "ID of the person to delete")
    ),
    responses(
        (status = 200, description = "Delete person by ID", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/people/{person_id}")]
pub async fn delete_person(
    people_service: web::Data<PeopleService>,
    person_id: web::Path<Uuid>,
) -> Result<web::Json<usize>, actix_web::Error> {

    match people_service.delete_person( *person_id).await {
        Ok(deleted_rows) => Ok(web::Json(deleted_rows)),
        Err(e) => Err(error::handle_db_error(&e, "delete_person")),
    }
}

#[utoipa::path(
    params(
        ("person_id" = Uuid, Path, description = "ID of the person to update")
    ),
    request_body = NewPerson,
    responses(
        (status = 200, description = "Update person by ID", body = Person),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/people/{person_id}")]
pub async fn update_person(
    people_service: web::Data<PeopleService>,
    person_id: web::Path<Uuid>,
    updated_person: web::Json<NewPerson>,
) -> Result<web::Json<Person>, actix_web::Error> {

    match people_service.update_person( *person_id, updated_person.into_inner().into()).await {
        Ok(person) => Ok(web::Json(Person::from(person))),
        Err(e) => Err(error::handle_db_error(&e, "update_person")),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to get persons for")
    ),
    responses(
        (status = 200, description = "Get persons by movie ID", body = [Person]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/people/movie/{movie_id}")]
pub async fn get_person_by_movie_id(
    people_service: web::Data<PeopleService>,
    movie_id: web::Path<Uuid>,
) -> Result<web::Json<Vec<Person>>, actix_web::Error> {

    match people_service.get_people_by_movie_id( *movie_id).await {
        Ok(persons) => Ok(web::Json(persons.into_iter().map(|p| Person::from(p)).collect())),
        Err(e) => Err(error::handle_db_error(&e, "get_person_by_movie_id")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PersonNameQuery {
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
        (status = 200, description = "Get persons by name", body = [Person]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/people/name")]
pub async fn get_person_by_name(
    people_service: web::Data<PeopleService>,
    params: web::Query<PersonNameQuery>,
) -> Result<web::Json<Vec<Person>>, actix_web::Error> {

    match people_service.get_people_by_name(&params.name, params.limit.unwrap_or(100), params.offset.unwrap_or(0)).await {
        Ok(persons) => Ok(web::Json(persons.into_iter().map(|p| Person::from(p)).collect())),
        Err(e) => Err(error::handle_db_error(&e, "get_person_by_name")),
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
