use diesel::{Insertable, Selectable, prelude::{Identifiable, Queryable}};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;
use diesel::query_builder::AsChangeset;

use crate::schema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub profile_picture_url: Option<String>,
    pub country: Option<String>,
    pub language_preference: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub country: Option<String>,
    pub language_preference: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::users)]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateUser {
    pub display_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub language_preference: Option<String>,
    pub country: Option<String>,
    pub status: Option<String>,
}
