use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::services::dtos::people::person_dto::PersonDto;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Person {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<PersonDto> for Person {
    fn from(dto: PersonDto) -> Self {
        Self {
            id: dto.id,
            first_name: dto.first_name,
            last_name: dto.last_name,
            birth_date: dto.birth_date,
            bio: dto.bio,
            role: dto.role,
            created_at: dto.created_at,
        }
    }
}

impl From<Person> for PersonDto {
    fn from(model: Person) -> Self {
        Self {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            birth_date: model.birth_date,
            bio: model.bio,
            role: model.role,
            created_at: model.created_at,
        }
    }
}
