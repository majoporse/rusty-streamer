use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use crate::data::models::movie::NewMovie;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMovieDto {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
}

impl From<NewMovieDto> for NewMovie {
    fn from(dto: NewMovieDto) -> Self {
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

impl From<NewMovie> for NewMovieDto {
    fn from(model: NewMovie) -> Self {
        Self {
            title: model.title,
            slug: model.slug,
            description: model.description,
            release_date: model.release_date,
            duration_minutes: model.duration_minutes,
            mpaa_rating: model.mpaa_rating,
        }
    }
}
