use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::schema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = schema::genres)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Genre {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = schema::genres)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGenre<'a> {
    pub name: &'a str,
}
