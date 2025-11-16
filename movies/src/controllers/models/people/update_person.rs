
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use crate::data::models::person::NewPerson as DataNewPerson;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
}


impl From<NewPerson> for DataNewPerson {
    fn from(np: NewPerson) -> Self {
        Self {
            first_name: np.first_name,
            last_name: np.last_name,
            birth_date: np.birth_date,
            bio: np.bio,
            role: np.role,
        }
    }
}

impl From<DataNewPerson> for NewPerson {
    fn from(np: DataNewPerson) -> Self {
        Self {
            first_name: np.first_name,
            last_name: np.last_name,
            birth_date: np.birth_date,
            bio: np.bio,
            role: np.role,
        }
    }
}