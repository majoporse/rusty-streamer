use crate::UserAuth;
use crate::controllers::users::auth::CustomClaims;
use crate::controllers::users::client_config;
use crate::controllers::users::error::handle_client_error;
use crate::controllers::users::pagination::Pagination;
use crate::models::users::{NewUser, UpdateUser, User};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use utoipa::OpenApi;

use users_client::apis::users_api;
use utoipa::openapi::info;
use uuid::Uuid;

static TAG: &str = "Users";

#[utoipa::path(
    responses(
        (status = 200, description = "List all users", body = [User]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/users")]
pub async fn get_all_users(claims: CustomClaims, pagination: web::Query<Pagination>) -> impl Responder {
    let config = client_config();

    match users_api::get_all_users(
        &config,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => handle_client_error(err, "Fetching all users"),
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
pub async fn get_user_by_id(user_id: web::Path<String>) -> impl Responder {
    let config = client_config();

    let parsed_id = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    match users_api::get_user_by_id(&config, &*parsed_id.to_string()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => handle_client_error(err, &format!("Fetching user {}", user_id)),
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
pub async fn create_user( new_user: web::Json<NewUser>) -> impl Responder {
    let config = client_config();

    let new_user = new_user.into_inner();

    match users_api::create_user(&config, new_user.into()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => handle_client_error(err, "Creating user"),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user to update")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Update user by ID", body = User),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/users/{user_id}")]
pub async fn update_user(
    user_id: web::Path<String>,
    updated_user: web::Json<UpdateUser>,
) -> impl Responder {
    let config = client_config();

    if let Err(_) = Uuid::parse_str(&user_id) {
        return HttpResponse::BadRequest().body("Invalid UUID format");
    };

    match users_api::update_user(
        &config,
        &*user_id,
        updated_user.into_inner().into(),
    )
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => handle_client_error(err, &format!("Updating user {}", user_id)),
    }
}

#[utoipa::path(
    params(
        ("user_id" = String, Path, description = "UUID of the user to delete")
    ),
    responses(
        (status = 200, description = "Delete user by ID", body = usize),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/users/{user_id}")]
pub async fn delete_user(user_id: web::Path<String>) -> impl Responder {
    let config = client_config();

    if let Err(_) = Uuid::parse_str(&user_id) {
        return HttpResponse::BadRequest().body("Invalid UUID format");
    };

    match users_api::delete_user(&config, &*user_id).await {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => handle_client_error(err, &format!("Deleting user {}", user_id)),
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
