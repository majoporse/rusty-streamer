use crate::data::models::person::Person;
use crate::data::models::{movie, person};
use crate::schema;
use crate::services::dtos::movie_crew::new_movie_crew_dto::NewMovieCrewDto;
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable, QueryableByName};
use diesel::Selectable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Associations,
    Identifiable,
    Selectable,
    QueryableByName,
)]
#[diesel(primary_key(movie_id, person_id))]
#[diesel(belongs_to(movie::Movie, foreign_key = movie_id))]
#[diesel(belongs_to(person::Person, foreign_key = person_id))]
#[diesel(table_name = schema::movie_crew)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MovieCrew {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub role: Option<String>,
    pub billing_order: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::movie_crew)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMovieCrew {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub role: Option<String>,
    pub billing_order: Option<i32>,
}

impl NewMovieCrew {
    pub fn new(dto: NewMovieCrewDto, movie_id: Uuid) -> Self {
        Self {
            movie_id,
            person_id: dto.person_id,
            role: dto.role,
            billing_order: dto.billing_order,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovieCrewDetail {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
    pub person: Person,
}

