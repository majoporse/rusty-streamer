use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::services::dtos::{
    genre::genre_dto::GenreDto, movie_crew::movie_crew_detail_dto::MovieCrewDetailDto,
    review::review_dto::ReviewDto,
};

pub struct MovieDetailsDto {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub people: Vec<MovieCrewDetailDto>,
    pub genres: Vec<GenreDto>,
    pub reviews: Vec<ReviewDto>,
}

impl From<crate::data::models::movie::MovieDetails> for MovieDetailsDto {
    fn from(m: crate::data::models::movie::MovieDetails) -> Self {
        MovieDetailsDto {
            id: m.id,
            title: m.title,
            slug: m.slug,
            description: m.description,
            release_date: m.release_date,
            duration_minutes: m.duration_minutes,
            mpaa_rating: m.mpaa_rating,
            created_at: m.created_at,
            updated_at: m.updated_at,
            people: m
                .people
                .into_iter()
                .map(MovieCrewDetailDto::from)
                .collect(),
            genres: m.genres.into_iter().map(GenreDto::from).collect(),
            reviews: m.reviews.into_iter().map(ReviewDto::from).collect(),
        }
    }
}
