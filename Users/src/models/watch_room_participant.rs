use diesel::{prelude::Queryable, Selectable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::schema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::watch_room_participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WatchRoomParticipant {
    pub id: i64,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: Option<NaiveDateTime>,
    pub last_active_at: Option<NaiveDateTime>,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::watch_room_participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWatchRoomParticipant {
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub is_admin: bool,
}
