use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use crate::data::models::person::NewPerson;


#[derive(Serialize, Deserialize)]
pub struct NewPersonDto {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
}

impl From<NewPersonDto> for NewPerson {
    fn from(dto: NewPersonDto) -> Self {
        Self {
            first_name: dto.first_name,
            last_name: dto.last_name,
            birth_date: dto.birth_date,
            bio: dto.bio,
            role: dto.role,
        }
    }
}

impl From<NewPerson> for NewPersonDto {
    fn from(model: NewPerson) -> Self {
        Self {
            first_name: model.first_name,
            last_name: model.last_name,
            birth_date: model.birth_date,
            bio: model.bio,
            role: model.role,
        }
    }
}