
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use crate::data::models::movie::NewMovie as DataNewMovie;
use crate::controllers::models::people::person::Person;
use crate::controllers::models::genre::genre::Genre;
use crate::controllers::models::reviews::review::Review;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NewMovie {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub people: Vec<Person>,
    pub genres: Vec<Genre>,
    pub reviews: Vec<Review>,
}

impl From<DataNewMovie> for NewMovie {
    fn from(m: DataNewMovie) -> Self {
        Self {
            title: m.title,
            slug: m.slug,
            description: m.description,
            release_date: m.release_date,
            duration_minutes: m.duration_minutes,
            mpaa_rating: m.mpaa_rating,
            people: Vec::new(),
            genres: Vec::new(),
            reviews: Vec::new(),
        }
    }
}

impl From<NewMovie> for DataNewMovie {
    fn from(m: NewMovie) -> Self {
        Self {
            title: m.title,
            slug: m.slug,
            description: m.description,
            release_date: m.release_date,
            duration_minutes: m.duration_minutes,
            mpaa_rating: m.mpaa_rating,
        }
    }
}
