use derive_more::{Deref, DerefMut};
use diesel_async::pooled_connection::bb8::Pool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    data::{models::DbConnection, people},
    services::dtos::people::{new_person_dto::NewPersonDto, person_dto::PersonDto},
};

#[derive(Clone, Deref, DerefMut)]
pub struct PeopleService {
    pool: Arc<Pool<DbConnection>>,
}

impl PeopleService {
    pub fn new(pool: Arc<Pool<DbConnection>>) -> Self {
        Self { pool }
    }

    pub async fn list_people(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PersonDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(people::list_people(&mut conn, limit, offset)
            .await?
            .into_iter()
            .map(|p| PersonDto::from(p))
            .collect())
    }

    pub async fn get_by_id(&self, person_id: Uuid) -> Result<PersonDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(people::get_person_by_id(&mut conn, person_id).await?.into())
    }

    pub async fn create_person(
        &self,
        new_person: NewPersonDto,
    ) -> Result<PersonDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(people::create_person(&mut conn, new_person.into())
            .await?
            .into())
    }

    pub async fn update_person(
        &self,
        person_id: Uuid,
        updated: NewPersonDto,
    ) -> Result<PersonDto, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(people::update_person(&mut conn, person_id, updated.into())
            .await?
            .into())
    }

    pub async fn delete_person(&self, person_id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(people::delete_person(&mut conn, person_id).await?)
    }

    pub async fn get_people_by_movie_id(
        &self,
        movie_id: Uuid,
    ) -> Result<Vec<PersonDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(people::get_people_by_movie_id(&mut conn, movie_id)
            .await?
            .into_iter()
            .map(|p| PersonDto::from(p))
            .collect())
    }

    pub async fn get_people_by_name(
        &self,
        name_query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PersonDto>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .await
            .expect("Couldn't get DB connection from pool");

        Ok(
            people::search_people_by_name(&mut conn, name_query, limit, offset)
                .await?
                .into_iter()
                .map(|p| PersonDto::from(p))
                .collect(),
        )
    }
}
