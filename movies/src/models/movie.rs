use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::schema;

#[derive(Deserialize, Serialize, Debug, Clone, Queryable, Identifiable, Selectable, ToSchema)]
#[diesel(table_name = schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::movies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMovie {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub duration_minutes: Option<i32>,
    pub mpaa_rating: Option<String>,
}
