

use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use crate::services::dtos::movie::new_movie_dto::NewMovieDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewMovie {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
}

impl From<NewMovie> for NewMovieDto {
    fn from(dto: NewMovie) -> Self {
        Self {
            title: dto.title,
            slug: dto.slug,
            description: dto.description,
            release_date: dto.release_date,
            duration_minutes: dto.duration_minutes,
            mpaa_rating: dto.mpaa_rating,
        }
    }
}
