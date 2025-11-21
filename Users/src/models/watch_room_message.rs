use diesel::{prelude::Queryable, Selectable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::schema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::watch_room_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WatchRoomMessage {
    pub id: i64,
    pub room_id: Uuid,
    pub user_id: Option<Uuid>,
    pub message: String,
    pub sent_at: NaiveDateTime,
    pub is_system_message: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::watch_room_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWatchRoomMessage {
    pub room_id: Uuid,
    pub user_id: Option<Uuid>,
    pub message: String,
    pub is_system_message: Option<bool>,
}
