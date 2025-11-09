use diesel::{Selectable, prelude::Queryable};

use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::{Identifiable, Insertable};
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema;


#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::actors)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Actor {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::actors)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewActor<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub birth_date: Option<NaiveDate>,
    pub bio: Option<&'a str>,
}
