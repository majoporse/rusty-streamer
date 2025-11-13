use diesel::{prelude::Queryable, Selectable, Identifiable, Insertable};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::schema;

#[derive(Debug, Clone, Serialize, Queryable, ToSchema, Identifiable, Selectable)]
#[diesel(table_name = schema::watchlist)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Watchlist {
    pub id: i64,
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = schema::watchlist)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWatchlist {
    pub user_id: Uuid,
    pub content_id: Uuid,
}
