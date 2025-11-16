use crate::controllers::models::reviews::new_review::NewReview;
use crate::controllers::models::reviews::review::Review;
use crate::controllers::pagination::Pagination;
use crate::services::review_service::ReviewService;
use crate::controllers::error;
use actix_web::{delete, get, post, put, web};
use utoipa::OpenApi;
use uuid::Uuid;

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
    review_service: web::Data<ReviewService>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<Review>>, actix_web::Error> {
    match review_service.list_reviews(
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    ).await {
        Ok(reviews) => Ok(web::Json(
            reviews.into_iter().map(|r| Review::from(r)).collect(),
        )),
        Err(e) => Err(error::handle_db_error(&e, "get_all_reviews")),
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
    review_service: web::Data<ReviewService>,
    new_review: web::Json<NewReview>,
) -> Result<web::Json<Review>, actix_web::Error> {
    match review_service.create_review(new_review.into_inner().into()).await {
        Ok(review) => Ok(web::Json(Review::from(review))),
        Err(e) => Err(error::handle_db_error(&e, "create_review")),
    }
}

#[utoipa::path(
	params(
		("review_id" = Uuid, Path, description = "ID of the review to retrieve")
	),
	responses(
		(status = 200, description = "Get review by ID", body = Review),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[get("/reviews/{review_id}")]
pub async fn get_review_by_id(
    review_service: web::Data<ReviewService>,
    review_id: web::Path<Uuid>,
) -> Result<web::Json<Review>, actix_web::Error> {
    match review_service.get_by_id(*review_id).await {
        Ok(review) => Ok(web::Json(Review::from(review))),
        Err(e) => Err(error::handle_db_error(&e, "get_review_by_id")),
    }
}

#[utoipa::path(
	params(
		("review_id" = Uuid, Path, description = "ID of the review to delete")
	),
	responses(
		(status = 200, description = "Delete review by ID", body = usize),
		(status = 500, description = "Internal Server Error")
	),
	tag = TAG
)]
#[delete("/reviews/{review_id}")]
pub async fn delete_review(
    review_service: web::Data<ReviewService>,
    review_id: web::Path<Uuid>,
) -> Result<web::Json<usize>, actix_web::Error> {
    match review_service.delete_review(*review_id).await {
        Ok(deleted_rows) => Ok(web::Json(deleted_rows)),
        Err(e) => Err(error::handle_db_error(&e, "delete_review")),
    }
}

#[utoipa::path(
	params(
		("review_id" = Uuid, Path, description = "ID of the review to update")
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
    review_service: web::Data<ReviewService>,
    review_id: web::Path<Uuid>,
    updated_review: web::Json<NewReview>,
) -> Result<web::Json<Review>, actix_web::Error> {
    match review_service.update_review(*review_id, updated_review.into_inner().into()).await {
        Ok(review) => Ok(web::Json(Review::from(review))),
        Err(e) => Err(error::handle_db_error(&e, "update_review")),
    }
}

#[utoipa::path(
    params(
        ("user_id" = Uuid, Path, description = "ID of the user to get reviews for"),
        ("limit" = i64, Query, description = "Max number of reviews to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Get reviews by user ID", body = [Review]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/reviews/user/{user_id}")]
pub async fn get_reviews_by_user_id(
    review_service: web::Data<ReviewService>,
    user_id: web::Path<String>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<Review>>, actix_web::Error> {

    let user_id = Uuid::parse_str(&user_id).map_err(|e| {
        error::handle_db_error(
            &diesel::result::Error::NotFound,
            &format!("Invalid UUID format for user_id: {}", e),
        )
    })?;

    match review_service.get_reviews_by_movie_id(
        user_id,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    ).await {
        Ok(reviews) => Ok(web::Json(
            reviews.into_iter().map(|r| Review::from(r)).collect(),
        )),
        Err(e) => Err(error::handle_db_error(&e, "get_reviews_by_user_id")),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = Uuid, Path, description = "ID of the movie to get reviews for"),
        ("limit" = i64, Query, description = "Max number of reviews to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Get reviews by movie ID", body = [Review]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/search/reviews/movie/{movie_id}")]
pub async fn get_reviews_by_movie_id(
    review_service: web::Data<ReviewService>,
    movie_id: web::Path<Uuid>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<Review>>, actix_web::Error> {
    match review_service.get_reviews_by_movie_id(
        *movie_id,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    ).await {
        Ok(reviews) => Ok(web::Json(
            reviews.into_iter().map(|r| Review::from(r)).collect(),
        )),
        Err(e) => Err(error::handle_db_error(&e, "get_reviews_by_movie_id")),
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    get_all_reviews,
    get_review_by_id,
    create_review,
    delete_review,
    update_review,
    get_reviews_by_user_id,
    get_reviews_by_movie_id,
))]
pub struct ApiDoc;

pub fn scoped_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(get_all_reviews);
    cfg.service(get_review_by_id);
    cfg.service(create_review);
    cfg.service(delete_review);
    cfg.service(update_review);
    cfg.service(get_reviews_by_user_id);
    cfg.service(get_reviews_by_movie_id);
}
