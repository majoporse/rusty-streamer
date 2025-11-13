use std::sync::Arc;

use crate::controllers::error;
use crate::data::actors;
use crate::models::{
    actor::{Actor, NewActor},
    DbConnection,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
static TAG: &str = "Actors";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of actors to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all actors", body = [Actor]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/actors")]
pub async fn get_all_actors(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    limit: web::Query<Option<i64>>,
    offset: web::Query<Option<i64>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match actors::list_actors(&mut db_conn, limit.into_inner().unwrap_or(100), offset.into_inner().unwrap_or(0)) {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(e) => error::handle_db_error(&e, "get_all_actors"),
    }
}

#[utoipa::path(
    params(
        ("actor_id" = i32, Path, description = "ID of the actor to retrieve")
    ),
    responses(
        (status = 200, description = "Get actor by ID", body = Actor),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/actors/{actor_id}")]
pub async fn get_actor_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    actor_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match actors::get_actor_by_id(&mut db_conn, *actor_id) {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(e) => error::handle_db_error(&e, "get_actor_by_id"),
    }
}

#[utoipa::path(
    request_body = NewActor,
    responses(
        (status = 200, description = "Create a new actor", body = Actor),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/actors")]
pub async fn create_actor(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_actor: web::Json<NewActor>,
) -> impl Responder {
    log::info!("Creating actor: {:?}", new_actor);
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");
    log::info!("Got DB connection");

    match actors::create_actor(&mut db_conn, new_actor.into_inner()) {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(e) =>  error::handle_db_error(&e, "create_actor"),
    }
}

#[utoipa::path(
    params(
        ("actor_id" = i32, Path, description = "ID of the actor to delete")
    ),
    responses(
        (status = 200, description = "Delete actor by ID", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/actors/{actor_id}")]
pub async fn delete_actor(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    actor_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match actors::delete_actor(&mut db_conn, *actor_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) =>  error::handle_db_error(&e, "delete_actor"),
    }
}

#[utoipa::path(
    params(
        ("actor_id" = i32, Path, description = "ID of the actor to update")
    ),
    request_body = NewActor,
    responses(
        (status = 200, description = "Update actor by ID", body = Actor),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/actors/{actor_id}")]
pub async fn update_actor(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    actor_id: web::Path<i32>,
    updated_actor: web::Json<NewActor>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match actors::update_actor(&mut db_conn, *actor_id, updated_actor.into_inner()) {
        Ok(actor) => HttpResponse::Ok().json(actor),
        Err(e) =>  error::handle_db_error(&e, "update_actor"),
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
