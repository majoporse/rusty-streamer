use uuid::Uuid;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use crate::data::models::movie::NewMovie;
use crate::services::dtos::movie_crew::new_movie_crew_dto::NewMovieCrewDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMovieDto {
    pub title: String,
    pub video_url: Option<String>,
    pub poster_url: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub people_ids: Option<Vec<NewMovieCrewDto>>,
    pub genre_ids: Option<Vec<Uuid>>,
}

impl From<NewMovieDto> for NewMovie {
    fn from(dto: NewMovieDto) -> Self {
        Self {
            title: dto.title,
            poster_url: dto.poster_url,
            video_url: dto.video_url,
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
            video_url: model.video_url,
            poster_url: model.poster_url,
            description: model.description,
            release_date: model.release_date,
            duration_minutes: model.duration_minutes,
            mpaa_rating: model.mpaa_rating,
            people_ids: None,
            genre_ids: None,
        }
    }
}
