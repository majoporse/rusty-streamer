

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::services::dtos::genre::genre_dto::GenreDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Genre {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<GenreDto> for Genre {
    fn from(dto: GenreDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
        }
    }
}