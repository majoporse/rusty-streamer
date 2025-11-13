use diesel::{prelude::Queryable, Selectable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::schema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::watch_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WatchRoom {
    pub id: Uuid,
    pub host_user_id: Uuid,
    pub content_id: Uuid,
    pub room_name: Option<String>,
    pub is_private: bool,
    pub invite_code: Option<String>,
    pub current_time_seconds: Option<i32>,
    pub is_live: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::watch_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWatchRoom {
    pub host_user_id: Uuid,
    pub content_id: Uuid,
    pub room_name: Option<String>,
    pub is_private: Option<bool>,
    pub invite_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateWatchRoom {
    pub current_time_seconds: Option<i32>,
    pub is_live: Option<bool>,
}
