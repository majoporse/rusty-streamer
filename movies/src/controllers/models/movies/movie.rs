
use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::services::dtos::movie::movie_dto::MovieDto;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, )]
pub struct Movie {
    pub id: Uuid,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub poster_url: Option<String>,
    pub title: String,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<MovieDto> for Movie {
    fn from(dto: MovieDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            poster_url: dto.poster_url,
            video_url: dto.video_url,
            description: dto.description,
            release_date: dto.release_date,
            duration_minutes: dto.duration_minutes,
            mpaa_rating: dto.mpaa_rating,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

impl From<Movie> for MovieDto {
    fn from(model: Movie) -> Self {
        Self {
            id: model.id,
            title: model.title,
            poster_url: model.poster_url,
            video_url: model.video_url,
            description: model.description,
            release_date: model.release_date,
            duration_minutes: model.duration_minutes,
            mpaa_rating: model.mpaa_rating,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}