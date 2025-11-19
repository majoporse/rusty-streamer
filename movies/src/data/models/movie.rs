use crate::schema;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub video_url: Option<String>,
    pub poster_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMovie {
    pub title: String,
    pub poster_url: Option<String>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MovieDetails {
    pub id: Uuid,
    pub title: String,
    pub poster_url: Option<String>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub people: Vec<crate::data::models::movie_crew::MovieCrewDetail>,
    pub genres: Vec<crate::data::models::genre::Genre>,
    pub reviews: Vec<crate::data::models::review::Review>,
}

impl From<Movie> for MovieDetails {
    fn from(movie: Movie) -> Self {
        MovieDetails {
            id: movie.id,
            title: movie.title,
            poster_url: movie.poster_url,
            video_url: movie.video_url,
            description: movie.description,
            release_date: movie.release_date,
            duration_minutes: movie.duration_minutes,
            mpaa_rating: movie.mpaa_rating,
            created_at: movie.created_at,
            updated_at: movie.updated_at,
            people: Vec::new(),
            genres: Vec::new(),
            reviews: Vec::new(),
        }
    }
}
