


use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use crate::services::dtos::people::new_person_dto::NewPersonDto;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
    pub image_url: Option<String>,
}

impl From<NewPerson> for NewPersonDto {
    fn from(dto: NewPerson) -> Self {
        Self {
            first_name: dto.first_name,
            last_name: dto.last_name,
            birth_date: dto.birth_date,
            bio: dto.bio,
            role: dto.role,
            image_url: dto.image_url,
        }
    }
}

impl From<NewPersonDto> for NewPerson {
    fn from(model: NewPersonDto) -> Self {
        Self {
            first_name: model.first_name,
            last_name: model.last_name,
            birth_date: model.birth_date,
            bio: model.bio,
            role: model.role,
            image_url: model.image_url,
        }
    }
}