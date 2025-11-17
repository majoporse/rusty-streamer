use crate::controllers::models::movie_crew::movie_crew::MovieCrew;
use crate::services::dtos::movie::new_movie_dto::NewMovieDto;
use crate::services::dtos::movie_crew::movie_crew_dto::MovieCrewDto;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewMovie {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub people_ids: Option<Vec<MovieCrew>>,
    pub genre_ids: Option<Vec<Uuid>>,
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
            people_ids: dto
                .people_ids
                .map(|dtos| dtos.into_iter().map(MovieCrew::from).collect()),
            genre_ids: dto.genre_ids,
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
            people_ids: model
                .people_ids
                .map(|models| models.into_iter().map(MovieCrewDto::from).collect()),
            genre_ids: model.genre_ids,
        }
    }
}
