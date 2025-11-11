use chrono::NaiveDateTime;
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{schema::reviews, models::movie::Movie};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations, ToSchema)]
#[diesel(belongs_to(Movie))]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
    pub id: i32,
    pub movie_id: i32,
    pub user_name: String,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewReview {
    pub movie_id: i32,
    pub user_name: String,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
}
