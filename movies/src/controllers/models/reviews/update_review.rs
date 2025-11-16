
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::data::models::review::NewReview as DataNewReview;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateReview {
    pub movie_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
}

impl From<UpdateReview> for DataNewReview {
    fn from(ur: UpdateReview) -> Self {
        Self {
            movie_id: ur.movie_id,
            user_id: ur.user_id,
            user_name: ur.user_name,
            rating: ur.rating,
            title: ur.title,
            body: ur.body,
        }
    }
}

impl From<DataNewReview> for UpdateReview {
    fn from(ur: DataNewReview) -> Self {
        Self {
            movie_id: ur.movie_id,
            user_id: ur.user_id,
            user_name: ur.user_name,
            rating: ur.rating,
            title: ur.title,
            body: ur.body,
        }
    }
}
