use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::services::dtos::review::new_review_dto::NewReviewDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewReview {
    pub movie_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
}

impl From<NewReviewDto> for NewReview {
    fn from(dto: NewReviewDto) -> Self {
        Self {
            movie_id: dto.movie_id,
            user_id: dto.user_id,
            user_name: dto.user_name,
            rating: dto.rating,
            title: dto.title,
            body: dto.body,
        }
    }
}

impl From<NewReview> for NewReviewDto {
    fn from(model: NewReview) -> Self {
        Self {
            movie_id: model.movie_id,
            user_id: model.user_id,
            user_name: model.user_name,
            rating: model.rating,
            title: model.title,
            body: model.body,
        }
    }
}