use std::sync::Arc;

use crate::controllers::error;
use crate::data::users; // <-- your Diesel CRUD functions module
use crate::models::user::{NewUser, UpdateUser, User};
use crate::models::DbConnection;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "Users";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of users to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all users", body = [User]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users")]
pub async fn get_all_users(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    limit: web::Query<Option<i64>>,
    offset: web::Query<Option<i64>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match users::list_users(
        &mut db_conn,
        limit.into_inner().unwrap_or(100),
        offset.into_inner().unwrap_or(0),
    ) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => error::handle_db_error(&e, "get_all_users"),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user to retrieve")
    ),
    responses(
        (status = 200, description = "Get user by ID", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    user_id: web::Path<String>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    match users::get_user_by_id(&mut db_conn, parsed_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => error::handle_db_error(&e, "get_user_by_id"),
    }
}

#[utoipa::path(
    request_body = NewUser,
    responses(
        (status = 200, description = "Create a new user", body = User),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/users")]
pub async fn create_user(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match users::create_user(&mut db_conn, new_user.into_inner()) {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(e) => error::handle_db_error(&e, "create_user"),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user to update")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Update user by ID", body = User),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/users/{user_id}")]
pub async fn update_user(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    user_id: web::Path<String>,
    updated_user: web::Json<UpdateUser>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    match users::update_user(&mut db_conn, parsed_id, updated_user.into_inner()) {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(e) => error::handle_db_error(&e, "update_user"),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user to delete")
    ),
    responses(
        (status = 200, description = "Delete user by ID", body = usize),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/users/{user_id}")]
pub async fn delete_user(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    user_id: web::Path<String>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let parsed_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    match users::delete_user(&mut db_conn, parsed_id) {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(e) => error::handle_db_error(&e, "delete_user"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(get_all_users, get_user_by_id, create_user, update_user, delete_user,))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_user_by_id);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
