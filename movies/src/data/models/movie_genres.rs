use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{data::models::genre, data::models::movie, schema};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Associations, Identifiable, Selectable)]
#[diesel(primary_key(movie_id, genre_id))]
#[diesel(table_name = schema::movie_genres)]
#[diesel(belongs_to(movie::Movie, foreign_key = movie_id))]
#[diesel(belongs_to(genre::Genre, foreign_key = genre_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MovieGenre {
    pub movie_id: Uuid,
    pub genre_id: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::movie_genres)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMovieGenre {
    pub movie_id: Uuid,
    pub genre_id: Uuid,
}
