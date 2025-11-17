use derive_more::{Deref, DerefMut};
use diesel_async::pooled_connection::bb8::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    data::{genres, models::DbConnection},
    services::dtos::genre::{genre_dto::GenreDto, new_genre_dto::NewGenreDto},
};

#[derive(Clone, Deref, DerefMut)]
pub struct GenreService {
    pool: Arc<Pool<DbConnection>>,
}

impl GenreService {
    pub fn new(pool: Arc<Pool<DbConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_genres(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GenreDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(genres::list_genres(&mut conn, limit, offset)
            .await?
            .into_iter()
            .map(GenreDto::from)
            .collect())
    }

    pub async fn get_by_id(&self, genre_id: Uuid) -> Result<GenreDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(genres::get_genre(&mut conn, genre_id).await?.into())
    }

    pub async fn create_genre(
        &self,
        new_genre: NewGenreDto,
    ) -> Result<GenreDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(genres::create_genre(&mut conn, new_genre.into())
            .await?
            .into())
    }

    pub async fn update_genre(
        &self,
        genre_id: Uuid,
        updated: NewGenreDto,
    ) -> Result<GenreDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(genres::update_genre(&mut conn, genre_id, updated.into())
            .await?
            .into())
    }

    pub async fn delete_genre(&self, genre_id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(genres::delete_genre(&mut conn, genre_id).await?)
    }
}
