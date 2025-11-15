use crate::{controllers::{movies::client_config, users::pagination::Pagination}, models::movies::{WrapperActor, WrapperNewActor}};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use movies_client::apis::actors_api;
use utoipa::OpenApi;
use crate::controllers::error::handle_client_error;

static TAG: &str = "Actors";

#[utoipa::path(
    responses(
        (status = 200, description = "List all actors", body = [WrapperActor]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/actors")]
pub async fn get_all_actors(
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let config = client_config();

    match actors_api::get_all_actors(&config, pagination.limit.unwrap_or(100), pagination.offset.unwrap_or(0)).await {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(err) => handle_client_error(err, "Fetching all actors"),
    }
}

#[utoipa::path(
    params(
        ("actor_id" = i32, Path, description = "ID of the actor to retrieve")
    ),
    responses(
        (status = 200, description = "Get actor by ID", body = WrapperActor),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/actors/{actor_id}")]
pub async fn get_actor_by_id(actor_id: web::Path<i32>) -> impl Responder {
    let config = client_config();


    match actors_api::get_actor_by_id(&config, actor_id.clone()).await {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(err) => handle_client_error(err, &format!("Fetching actor {}", actor_id)),
    }
}

#[utoipa::path(
    request_body = WrapperNewActor,
    responses(
        (status = 200, description = "Create a new actor", body = WrapperActor),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/actors")]
pub async fn create_actor(new_actor: web::Json<WrapperNewActor>) -> impl Responder {
    let config = client_config();
    let new_actor = new_actor.into_inner();

    match actors_api::create_actor(&config, new_actor.into()).await {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(err) => handle_client_error(err, "Creating actor"),
    }
}

#[utoipa::path(
    params(
        ("actor_id" = i32, Path, description = "ID of the actor to delete")
    ),
    responses(
        (status = 200, description = "Delete actor by ID", body = usize),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/actors/{actor_id}")]
pub async fn delete_actor(actor_id: web::Path<i32>) -> impl Responder {
    let config = client_config();


    match actors_api::delete_actor(&config, actor_id.clone()).await {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => handle_client_error(err, &format!("Deleting actor {}", actor_id)),
    }
}

#[utoipa::path(
    params(
        ("actor_id" = i32, Path, description = "ID of the actor to update")
    ),
    request_body = WrapperNewActor,
    responses(
        (status = 200, description = "Update actor by ID", body = WrapperActor),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/actors/{actor_id}")]
pub async fn update_actor(
    actor_id: web::Path<i32>,
    updated_actor: web::Json<WrapperNewActor>,
) -> impl Responder {
    let config = client_config();
    let updated_actor = updated_actor.into_inner();

    match actors_api::update_actor(&config, actor_id.clone(), updated_actor.into()).await {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(err) => handle_client_error(err, &format!("Updating actor {}", actor_id)),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_all_actors,
    get_actor_by_id,
    create_actor,
    delete_actor,
    update_actor,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_all_actors);
    cfg.service(get_actor_by_id);
    cfg.service(create_actor);
    cfg.service(delete_actor);
    cfg.service(update_actor);
}
