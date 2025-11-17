use uuid::Uuid;
use serde::Deserialize;
use serde::Serialize;

use crate::controllers::models::movie_crew::movie_crew::MovieCrew;
use crate::data::models::movie_crew::NewMovieCrew;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieCrewDto {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
}

impl From<MovieCrew> for MovieCrewDto {
    fn from(model: MovieCrew) -> Self {
        Self {
            movie_id: model.movie_id,
            person_id: model.person_id,
            character_name: model.character_name,
            billing_order: model.billing_order,
        }
    }
}

impl From<MovieCrewDto> for NewMovieCrew {
    fn from(dto: MovieCrewDto) -> Self {
        Self {
            movie_id: dto.movie_id,
            person_id: dto.person_id,
            character_name: dto.character_name,
            billing_order: dto.billing_order,
        }
    }
}