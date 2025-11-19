use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::controllers::models::genre::genre::Genre;
use crate::controllers::models::movie_crew::movie_crew_detail::MovieCrewDetail;
use crate::controllers::models::reviews::review::Review;
use crate::services::dtos::movie::movie_details_dto::MovieDetailsDto;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, )]
pub struct MovieDetail {
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
    pub people: Vec<MovieCrewDetail>,
    pub genres: Vec<Genre>,
    pub reviews: Vec<Review>,
}

impl From<MovieDetailsDto> for MovieDetail {
    fn from(dto: MovieDetailsDto) -> Self {
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
            people: dto.people.into_iter().map(MovieCrewDetail::from).collect(),
            genres: dto.genres.into_iter().map(Genre::from).collect(),
            reviews: dto.reviews.into_iter().map(Review::from).collect(),
        }
    }
}