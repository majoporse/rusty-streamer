use crate::controllers::error::handle_client_error;
use crate::{
    controllers::{movies::client_config, users::pagination::Pagination},
    models::movies::{WrapperNewReview, WrapperReview},
};
use actix_web::error::ErrorInternalServerError;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use movies_client::apis::reviews_api;
use movies_client::models::review;
use users_client::models::user;
use utoipa::OpenApi;
use uuid::Uuid;

static TAG: &str = "Reviews";

#[utoipa::path(
    params(
        ("limit" = Option<i64>, Query, description = "Max number of reviews to return", example = 100),
        ("offset" = Option<i64>, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "List all reviews", body = [WrapperReview]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/reviews")]
pub async fn get_all_reviews(
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<WrapperReview>>, actix_web::Error> {
    let config = client_config();

    match reviews_api::get_all_reviews(
        &config,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(reviews) => Ok(web::Json(
            reviews
                .into_iter()
                .map(|e| WrapperReview::from(e))
                .collect(),
        )),
        Err(err) => Err(handle_client_error(err, "Fetching all reviews")),
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
pub async fn get_review_by_id(
    review_id: web::Path<Uuid>,
) -> Result<web::Json<WrapperReview>, actix_web::Error> {
    let config = client_config();
    let review_id = review_id.to_string();

    match reviews_api::get_review_by_id(&config, &review_id).await {
        Ok(review) => Ok(web::Json(review.into())),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Fetching review {}", review_id),
        )),
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
pub async fn create_review(
    new_review: web::Json<WrapperNewReview>,
) -> Result<web::Json<WrapperReview>, actix_web::Error> {
    let config = client_config();
    let new_review = new_review.into_inner();

    match reviews_api::create_review(&config, new_review.into()).await {
        Ok(review) => Ok(web::Json(review.into())),
        Err(err) => Err(handle_client_error(err, "Creating review")),
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
pub async fn delete_review(review_id: web::Path<Uuid>) -> Result<web::Json<i32>, actix_web::Error> {
    let config = client_config();
    let review_id = review_id.to_string();

    match reviews_api::delete_review(&config, &review_id).await {
        Ok(deleted) => Ok(web::Json(deleted)),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Deleting review {}", review_id),
        )),
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
    review_id: web::Path<Uuid>,
    updated_review: web::Json<WrapperNewReview>,
) -> Result<web::Json<WrapperReview>, actix_web::Error> {
    let config = client_config();
    let updated_review = updated_review.into_inner();
    let review_id = review_id.to_string();

    match reviews_api::update_review(&config, &review_id, updated_review.into()).await {
        Ok(review) => Ok(web::Json(review.into())),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Updating review {}", review_id),
        )),
    }
}

#[utoipa::path(
    params(
        ("user_id" = i32, Path, description = "ID of the user to get reviews for"),
        ("limit" = i64, Query, description = "Max number of reviews to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Get reviews by user ID", body = [WrapperReview]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/reviews/user/{user_id}")]
pub async fn get_reviews_by_user_id(
    user_id: web::Path<Uuid>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<WrapperReview>>, actix_web::Error> {
    let config = client_config();
    let user_id = user_id.to_string();

    match reviews_api::get_reviews_by_user_id(
        &config,
        &user_id,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(review) => Ok(web::Json(
            review.into_iter().map(|e| WrapperReview::from(e)).collect(),
        )),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Fetching reviews for user {}", user_id),
        )),
    }
}

#[utoipa::path(
    params(
        ("movie_id" = i32, Path, description = "ID of the movie to get reviews for"),
        ("limit" = i64, Query, description = "Max number of reviews to return", example = 100),
        ("offset" = i64, Query, description = "Pagination offset", example = 0)
    ),
    responses(
        (status = 200, description = "Get reviews by movie ID", body = [WrapperReview]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[get("/reviews/movie/{movie_id}")]
pub async fn get_reviews_by_movie_id(
    movie_id: web::Path<Uuid>,
    pagination: web::Query<Pagination>,
) -> Result<web::Json<Vec<WrapperReview>>, actix_web::Error> {
    let config = client_config();
    let movie_id = movie_id.to_string();

    match reviews_api::get_reviews_by_movie_id(
        &config,
        &movie_id,
        pagination.limit.unwrap_or(100),
        pagination.offset.unwrap_or(0),
    )
    .await
    {
        Ok(review) => Ok(web::Json(
            review.into_iter().map(|e| WrapperReview::from(e)).collect(),
        )),
        Err(err) => Err(handle_client_error(
            err,
            &format!("Fetching reviews for movie {}", movie_id),
        )),
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
