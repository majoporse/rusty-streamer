
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::services::dtos::movie_crew::new_movie_crew_dto::NewMovieCrewDto;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, )]
pub struct NewMovieCrew{
    pub person_id: Uuid,
    pub role: Option<String>,
    pub billing_order: Option<i32>,
}

impl From<NewMovieCrewDto> for NewMovieCrew {
    fn from(dto: NewMovieCrewDto) -> Self {
        Self {
            person_id: dto.person_id,
            role: dto.role,
            billing_order: dto.billing_order,
        }
    }
}

