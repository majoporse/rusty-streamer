use derive_more::{Deref, DerefMut};
use diesel_async::pooled_connection::bb8::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    data::{models::DbConnection, reviews},
    services::dtos::review::{new_review_dto::NewReviewDto, review_dto::ReviewDto},
};

#[derive(Clone, Deref, DerefMut)]
pub struct ReviewService {
    pool: Arc<Pool<DbConnection>>,
}

impl ReviewService {
    pub fn new(pool: Arc<Pool<DbConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_reviews(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ReviewDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(reviews::list_reviews(&mut conn, limit, offset)
            .await?
            .into_iter()
            .map(ReviewDto::from)
            .collect())
    }

    pub async fn get_by_id(&self, review_id: Uuid) -> Result<ReviewDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(reviews::get_review_by_id(&mut conn, review_id)
            .await?
            .into())
    }

    pub async fn create_review(
        &self,
        new_review: NewReviewDto,
    ) -> Result<ReviewDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(reviews::create_review(&mut conn, new_review.into())
            .await?
            .into())
    }

    pub async fn update_review(
        &self,
        review_id: Uuid,
        updated: NewReviewDto,
    ) -> Result<ReviewDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(reviews::update_review(&mut conn, review_id, updated.into())
            .await?
            .into())
    }

    pub async fn delete_review(&self, review_id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(reviews::delete_review(&mut conn, review_id).await?)
    }

    pub async fn get_reviews_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ReviewDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(
            reviews::get_reviews_by_user_id(&mut conn, user_id, limit, offset)
                .await?
                .into_iter()
                .map(ReviewDto::from)
                .collect(),
        )
    }

    pub async fn get_reviews_by_movie_id(
        &self,
        movie_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ReviewDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(
            reviews::get_reviews_by_movie_id(&mut conn, movie_id, limit, offset)
                .await?
                .into_iter()
                .map(ReviewDto::from)
                .collect(),
        )
    }
}
