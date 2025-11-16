use crate::data::models::person::Person;
use crate::data::models::{movie, person};
use crate::schema;
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
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::movie_crew)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMovieCrew<'a> {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<&'a str>,
    pub billing_order: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct MovieCrewDetail {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
    pub person: Person,
}

