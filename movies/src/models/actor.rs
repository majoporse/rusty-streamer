use diesel::{prelude::Queryable, Selectable};

use crate::schema;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::{Identifiable, Insertable};
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::actors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Actor {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::actors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewActor {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
}
