use std::sync::Arc;

use crate::controllers::error;
use crate::controllers::pagination::Pagination;
use crate::data::reviews;
use crate::models::{
    review::{NewReview, Review},
    DbConnection,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use utoipa::{OpenApi};

static TAG: &str = "Reviews";

#[utoipa::path(
    params(
        ("limit" = i64, Query, description = "Max number of reviews to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
	responses(
		(status = 200, description = "List all reviews", body = [Review]),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[get("/reviews")]
pub async fn get_all_reviews(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    pagination: web::Query<Pagination>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::list_reviews(&mut db_conn, pagination.limit.unwrap_or(100), pagination.offset.unwrap_or(0)) {
        Ok(reviews) => HttpResponse::Ok().json(reviews),
        Err(e) => error::handle_db_error(&e, "get_all_reviews"),
    }
}

#[utoipa::path(
	params(
		("review_id" = i32, Path, description = "ID of the review to retrieve")
	),
	responses(
		(status = 200, description = "Get review by ID", body = Review),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[get("/reviews/{review_id}")]
pub async fn get_review_by_id(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    review_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::get_review_by_id(&mut db_conn, *review_id) {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => error::handle_db_error(&e, "get_review_by_id"),
    }
}

#[utoipa::path(
	request_body = NewReview,
	responses(
		(status = 200, description = "Create a new review", body = Review),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[post("/reviews")]
pub async fn create_review(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    new_review: web::Json<NewReview>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::create_review(&mut db_conn, new_review.into_inner()) {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => error::handle_db_error(&e, "create_review"),
    }
}

#[utoipa::path(
	params(
		("review_id" = i32, Path, description = "ID of the review to delete")
	),
	responses(
		(status = 200, description = "Delete review by ID", body = usize),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[delete("/reviews/{review_id}")]
pub async fn delete_review(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    review_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::delete_review(&mut db_conn, *review_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(e) => error::handle_db_error(&e, "delete_review"),
    }
}

#[utoipa::path(
	params(
		("review_id" = i32, Path, description = "ID of the review to update")
	),
	request_body = NewReview,
	responses(
		(status = 200, description = "Update review by ID", body = Review),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[put("/reviews/{review_id}")]
pub async fn update_review(
    pool: web::Data<Arc<r2d2::Pool<ConnectionManager<DbConnection>>>>,
    review_id: web::Path<i32>,
    updated_review: web::Json<NewReview>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::update_review(&mut db_conn, *review_id, updated_review.into_inner()) {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(e) => error::handle_db_error(&e, "update_review"),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_all_reviews,
    get_review_by_id,
    create_review,
    delete_review,
    update_review,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_all_reviews);
    cfg.service(get_review_by_id);
    cfg.service(create_review);
    cfg.service(delete_review);
    cfg.service(update_review);
}
