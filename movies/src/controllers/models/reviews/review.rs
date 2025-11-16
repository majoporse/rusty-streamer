
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::services::dtos::review::review_dto::ReviewDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Review {
    pub id: Uuid,
    pub movie_id: Uuid,
    pub user_name: String,
    pub user_id: Uuid,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<ReviewDto> for Review {
    fn from(dto: ReviewDto) -> Self {
        Self {
            id: dto.id,
            movie_id: dto.movie_id,
            user_name: dto.user_name,
            user_id: dto.user_id,
            rating: dto.rating,
            title: dto.title,
            body: dto.body,
            created_at: dto.created_at,
        }
    }
}

impl From<Review> for ReviewDto {
    fn from(model: Review) -> Self {
        Self {
            id: model.id,
            movie_id: model.movie_id,
            user_name: model.user_name,
            user_id: model.user_id,
            rating: model.rating,
            title: model.title,
            body: model.body,
            created_at: model.created_at,
        }
    }
}
