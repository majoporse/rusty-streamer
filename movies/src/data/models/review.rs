use chrono::NaiveDateTime;
use diesel::{Selectable, prelude::{Associations, Identifiable, Insertable, Queryable}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{schema::reviews, data::models::movie::Movie};

#[derive(Debug, Selectable, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Movie))]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Review {
    pub id: Uuid,
    pub movie_id: Uuid,
    pub user_name: String,
    pub user_id: Uuid,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = reviews)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewReview {
    pub movie_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub rating: i16,
    pub title: Option<String>,
    pub body: Option<String>,
}
