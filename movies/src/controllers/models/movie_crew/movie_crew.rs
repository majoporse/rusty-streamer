use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::services::dtos::movie_crew::movie_crew_dto::MovieCrewDto;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, )]
pub struct MovieCrew{
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
}

impl From<MovieCrewDto> for MovieCrew {
    fn from(dto: MovieCrewDto) -> Self {
        Self {
            movie_id: dto.movie_id,
            person_id: dto.person_id,
            character_name: dto.character_name,
            billing_order: dto.billing_order,
        }
    }
}