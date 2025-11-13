use crate::models::movies::{WrapperReview, WrapperNewReview};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use movies_client::apis::{reviews_api, configuration::Configuration};
use utoipa::OpenApi;
use crate::controllers::error::handle_client_error;

static TAG: &str = "Reviews";

#[utoipa::path(
    responses(
        (status = 200, description = "List all reviews", body = [WrapperReview]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/reviews")]
pub async fn get_all_reviews() -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();

    match reviews_api::get_all_reviews(&config).await {
        Ok(reviews) => HttpResponse::Ok().json(reviews),
        Err(err) => handle_client_error(err, "Fetching all reviews"),
    }
}

#[utoipa::path(
    params(
        ("review_id" = i32, Path, description = "ID of the review to retrieve")
    ),
    responses(
        (status = 200, description = "Get review by ID", body = WrapperReview),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/reviews/{review_id}")]
pub async fn get_review_by_id(review_id: web::Path<i32>) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();

    match reviews_api::get_review_by_id(&config, review_id.clone()).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(err) => handle_client_error(err, &format!("Fetching review {}", review_id)),
    }
}

#[utoipa::path(
    request_body = WrapperNewReview,
    responses(
        (status = 200, description = "Create a new review", body = WrapperReview),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/reviews")]
pub async fn create_review(new_review: web::Json<WrapperNewReview>) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();
    let new_review = new_review.into_inner();

    match reviews_api::create_review(&config, new_review.into()).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(err) => handle_client_error(err, "Creating review"),
    }
}

#[utoipa::path(
    params(
        ("review_id" = i32, Path, description = "ID of the review to delete")
    ),
    responses(
        (status = 200, description = "Delete review by ID", body = usize),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[delete("/reviews/{review_id}")]
pub async fn delete_review(review_id: web::Path<i32>) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();

    match reviews_api::delete_review(&config, review_id.clone()).await {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        Err(err) => handle_client_error(err, &format!("Deleting review {}", review_id)),
    }
}

#[utoipa::path(
    params(
        ("review_id" = i32, Path, description = "ID of the review to update")
    ),
    request_body = WrapperNewReview,
    responses(
        (status = 200, description = "Update review by ID", body = WrapperReview),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[put("/reviews/{review_id}")]
pub async fn update_review(
    review_id: web::Path<i32>,
    updated_review: web::Json<WrapperNewReview>,
) -> impl Responder {
    let mut config = Configuration::default();
    config.base_path = "http://127.0.0.1:8081".to_string();
    let updated_review = updated_review.into_inner();

    match reviews_api::update_review(&config, review_id.clone(), updated_review.into()).await {
        Ok(review) => HttpResponse::Ok().json(review),
        Err(err) => handle_client_error(err, &format!("Updating review {}", review_id)),
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
