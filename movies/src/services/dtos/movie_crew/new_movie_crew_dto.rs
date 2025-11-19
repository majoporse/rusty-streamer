
use uuid::Uuid;
use serde::Deserialize;
use serde::Serialize;

use crate::controllers::models::movie_crew::new_movie_crew::NewMovieCrew;




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMovieCrewDto {
    pub person_id: Uuid,
    pub role: Option<String>,
    pub billing_order: Option<i32>,
}

impl From<NewMovieCrew> for NewMovieCrewDto {
    fn from(model: NewMovieCrew) -> Self {
        Self {
            person_id: model.person_id,
            role: model.role,
            billing_order: model.billing_order,
        }
    }
}
