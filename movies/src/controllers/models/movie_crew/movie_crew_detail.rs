use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::controllers::models::people::person::Person;
use crate::services::dtos::movie_crew::movie_crew_detail_dto::MovieCrewDetailDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, )]
pub struct MovieCrewDetail {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
    pub person: Person,
}

impl From<MovieCrewDetailDto> for MovieCrewDetail {
    fn from(dto: MovieCrewDetailDto) -> Self {
        Self {
            movie_id: dto.movie_id,
            person_id: dto.person_id,
            character_name: dto.character_name,
            billing_order: dto.billing_order,
            person: Person::from(dto.person),
        }
    }
}