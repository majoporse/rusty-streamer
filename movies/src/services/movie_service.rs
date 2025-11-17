use std::sync::Arc;

use derive_more::Deref;
use derive_more::DerefMut;
use diesel_async::pooled_connection::bb8::Pool;
use uuid::Uuid;

use crate::data::models::movie_crew::NewMovieCrew;
use crate::services::dtos::movie::movie_details_dto::MovieDetailsDto;
use crate::{
    data::{models::DbConnection, movies},
    services::dtos::movie::{movie_dto::MovieDto, new_movie_dto::NewMovieDto},
};

#[derive(Clone, Deref, DerefMut)]
pub struct MovieService {
    pool: Arc<Pool<DbConnection>>,
}

impl MovieService {
    pub fn new(pool: Arc<Pool<DbConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_movies(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MovieDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(movies::list_movies(&mut conn, limit, offset)
            .await?
            .into_iter()
            .map(|m| MovieDto::from(m))
            .collect())
    }

    pub async fn search_by_title(
        &self,
        title_query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MovieDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(
            movies::search_movies_by_title(&mut conn, title_query, limit, offset)
                .await?
                .into_iter()
                .map(|m| MovieDto::from(m))
                .collect(),
        )
    }

    pub async fn get_movie_details_by_id(
        &self,
        movie_id: Uuid,
    ) -> Result<MovieDetailsDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(movies::get_movie_details_by_id(&mut conn, movie_id)
            .await?
            .into())
    }

    pub async fn search_by_actor(
        &self,
        actor_name: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MovieDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(
            movies::search_movies_by_actor(&mut conn, actor_name, limit, offset)
                .await?
                .into_iter()
                .map(|m| MovieDto::from(m))
                .collect(),
        )
    }

    pub async fn get_by_id(&self, movie_id: Uuid) -> Result<MovieDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(movies::get_movie_by_id(&mut conn, movie_id).await?.into())
    }

    pub async fn create_movie(
        &self,
        mut new_movie: NewMovieDto,
    ) -> Result<MovieDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");
// TODO TRANSACTION

        let genre_ids = new_movie.genre_ids;
        let people_ids = new_movie.people_ids;
        new_movie.genre_ids = None;
        new_movie.people_ids = None;

        let movie: MovieDto = movies::create_movie(&mut conn, new_movie.into())
            .await?
            .into();

        if let Some(genre_ids) = genre_ids {
            movies::add_genres_to_movie(&mut conn, movie.id, genre_ids).await?;
        }

        if let Some(people_ids) = people_ids {
            movies::add_people_to_movie(
                &mut conn,
                people_ids
                    .into_iter()
                    .map(|e| NewMovieCrew::from(e))
                    .collect(),
            )
            .await?;
        }

        Ok(movie)
    }

    pub async fn update_movie(
        &self,
        movie_id: Uuid,
        updated: NewMovieDto,
    ) -> Result<MovieDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(movies::update_movie(&mut conn, movie_id, updated.into())
            .await?
            .into())
    }

    pub async fn delete_movie(&self, movie_id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(movies::delete_movie(&mut conn, movie_id).await?)
    }
}
