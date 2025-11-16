use uuid::Uuid;

use crate::{
    data::models::movie_crew::MovieCrewDetail, services::dtos::people::person_dto::PersonDto,
};

pub struct MovieCrewDetailDto {
    pub movie_id: Uuid,
    pub person_id: Uuid,
    pub character_name: Option<String>,
    pub billing_order: Option<i32>,
    pub person: PersonDto,
}

impl From<MovieCrewDetail> for MovieCrewDetailDto {
    fn from(v: MovieCrewDetail) -> Self {
        MovieCrewDetailDto {
            movie_id: v.movie_id,
            person_id: v.person_id,
            character_name: v.character_name,
            billing_order: v.billing_order,
            person: v.person.into(),
        }
    }
}
