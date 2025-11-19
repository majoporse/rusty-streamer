use diesel::{prelude::Queryable, Selectable};

use crate::schema;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::{Identifiable, Insertable};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPerson {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub role: Option<String>,
    pub image_url: Option<String>,
}