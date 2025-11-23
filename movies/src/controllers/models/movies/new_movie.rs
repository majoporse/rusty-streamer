use crate::controllers::models::movie_crew::new_movie_crew::NewMovieCrew;
use crate::services::dtos::movie::new_movie_dto::NewMovieDto;
use crate::services::dtos::movie_crew::new_movie_crew_dto::NewMovieCrewDto;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewMovie {
    pub title: String,
    pub poster_url: Option<String>,
    pub video_url: Option<String>,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub people_ids: Option<Vec<NewMovieCrew>>,
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
            people_ids: dto
                .people_ids
                .map(|dtos| dtos.into_iter().map(NewMovieCrew::from).collect()),
            genre_ids: dto.genre_ids,
        }
    }
}

impl From<NewMovie> for NewMovieDto {
    fn from(model: NewMovie) -> Self {
        Self {
            title: model.title,
            poster_url: model.poster_url,
            video_url: model.video_url,
            description: model.description,
            release_date: model.release_date,
            duration_minutes: model.duration_minutes,
            mpaa_rating: model.mpaa_rating,
            people_ids: model
                .people_ids
                .map(|models| models.into_iter().map(NewMovieCrewDto::from).collect()),
            genre_ids: model.genre_ids,
        }
    }
}
