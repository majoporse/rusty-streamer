use crate::data::reviews;
use crate::models::{
    review::{NewReview, Review},
    DbConnection,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

#[utoipa::path(
	responses(
		(status = 200, description = "List all reviews", body = [Review]),
		(status = 500, description = "Internal Server Error")
	),
	tag = "Reviews"
)]
#[get("/reviews")]
pub async fn get_all_reviews(
    pool: web::Data<r2d2::Pool<ConnectionManager<DbConnection>>>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::list_reviews(&mut db_conn, 100, 0) {
        Ok(reviews) => HttpResponse::Ok().json(reviews),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
	tag = "Reviews"
)]
#[get("/reviews/{review_id}")]
pub async fn get_review_by_id(
    pool: web::Data<r2d2::Pool<ConnectionManager<DbConnection>>>,
    review_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::get_review_by_id(&mut db_conn, *review_id) {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct NewReviewRequest {
    pub movie_id: i32,
    pub user_name: String,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
}

#[utoipa::path(
	request_body = NewReviewRequest,
	responses(
		(status = 200, description = "Create a new review", body = Review),
		(status = 500, description = "Internal Server Error")
	),
	tag = "Reviews"
)]
#[post("/reviews")]
pub async fn create_review(
    pool: web::Data<r2d2::Pool<ConnectionManager<DbConnection>>>,
    new_review: web::Json<NewReviewRequest>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let req = new_review.into_inner();
    let insert = NewReview {
        movie_id: req.movie_id,
        user_name: &req.user_name,
        rating: req.rating,
        title: req.title.as_deref(),
        body: req.body.as_deref(),
    };

    match reviews::create_review(&mut db_conn, &insert) {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
	tag = "Reviews"
)]
#[delete("/reviews/{review_id}")]
pub async fn delete_review(
    pool: web::Data<r2d2::Pool<ConnectionManager<DbConnection>>>,
    review_id: web::Path<i32>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match reviews::delete_review(&mut db_conn, *review_id) {
        Ok(deleted_rows) => HttpResponse::Ok().json(deleted_rows),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
	params(
		("review_id" = i32, Path, description = "ID of the review to update")
	),
	request_body = NewReviewRequest,
	responses(
		(status = 200, description = "Update review by ID", body = Review),
		(status = 500, description = "Internal Server Error")
	),
	tag = "Reviews"
)]
#[put("/reviews/{review_id}")]
pub async fn update_review(
    pool: web::Data<r2d2::Pool<ConnectionManager<DbConnection>>>,
    review_id: web::Path<i32>,
    updated_review: web::Json<NewReviewRequest>,
) -> impl Responder {
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    let req = updated_review.into_inner();
    let update = NewReview {
        movie_id: req.movie_id,
        user_name: &req.user_name,
        rating: req.rating,
        title: req.title.as_deref(),
        body: req.body.as_deref(),
    };

    match reviews::update_review(&mut db_conn, *review_id, &update) {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(_) => HttpResponse::InternalServerError().finish(),
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
