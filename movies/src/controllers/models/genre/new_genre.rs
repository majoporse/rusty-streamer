use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::services::dtos::genre::new_genre_dto::NewGenreDto;

#[derive(Debug, Clone, ToSchema, Serialize, Deserialize)]
pub struct NewGenre {
    pub name: String,
}

impl From<NewGenreDto> for NewGenre {
    fn from(genre: NewGenreDto) -> Self {
        NewGenre {
            name: genre.name,
        }
    }
}
impl From<NewGenre> for NewGenreDto {
    fn from(dto: NewGenre) -> Self {
        NewGenreDto {
            name: dto.name,
        }
    }
}