use crate::models::{actor, movie};
use crate::schema;
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Associations, Identifiable)]
#[diesel(primary_key(movie_id, actor_id))]
#[diesel(belongs_to(movie::Movie, foreign_key = movie_id))]
#[diesel(belongs_to(actor::Actor, foreign_key = actor_id))]
#[diesel(table_name = schema::movie_actors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MovieActor {
    pub movie_id: i32,
    pub actor_id: i32,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::movie_actors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMovieActor<'a> {
    pub movie_id: i32,
    pub actor_id: i32,
    pub character_name: Option<&'a str>,
    pub billing_order: Option<i32>,
}
