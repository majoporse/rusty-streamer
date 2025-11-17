use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use crate::data::models::movie::Movie;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieDto {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Movie> for MovieDto {
    fn from(m: Movie) -> Self {
        Self {
            id: m.id,
            title: m.title,
            slug: m.slug,
            description: m.description,
            release_date: m.release_date,
            duration_minutes: m.duration_minutes,
            mpaa_rating: m.mpaa_rating,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}