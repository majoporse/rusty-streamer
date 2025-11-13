use diesel::{prelude::Queryable, Selectable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::schema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::watch_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WatchHistory {
    pub id: i64,
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub progress_seconds: Option<i32>,
    pub completed: Option<bool>,
    pub last_watched_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::watch_history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWatchHistory {
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub progress_seconds: Option<i32>,
    pub completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateWatchHistory {
    pub progress_seconds: Option<i32>,
    pub completed: Option<bool>,
}
