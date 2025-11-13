use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    models::{
        DbConnection,
        watch_room_message::{NewWatchRoomMessage, WatchRoomMessage},
    },
    schema::watch_room_messages,
};

/// Create a new message in a watch room
pub fn create_watch_room_message(
    conn: &mut DbConnection,
    new_message: NewWatchRoomMessage,
) -> Result<WatchRoomMessage, diesel::result::Error> {
    let item = diesel::insert_into(watch_room_messages::table)
        .values(new_message)
        .get_result::<WatchRoomMessage>(conn)?;
    Ok(item)
}

/// Get a single message by its ID
pub fn get_watch_room_message_by_id(
    conn: &mut DbConnection,
    id: i64,
) -> Result<WatchRoomMessage, diesel::result::Error> {
    let item = watch_room_messages::table
        .filter(watch_room_messages::id.eq(id))
        .first::<WatchRoomMessage>(conn)?;
    Ok(item)
}

/// List all messages in a specific room, ordered by send time (ascending)
pub fn list_messages_by_room(
    conn: &mut DbConnection,
    room_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WatchRoomMessage>, diesel::result::Error> {
    let items = watch_room_messages::table
        .filter(watch_room_messages::room_id.eq(room_id))
        .order(watch_room_messages::sent_at.asc())
        .limit(limit)
        .offset(offset)
        .load::<WatchRoomMessage>(conn)?;
    Ok(items)
}

/// Delete a specific message by ID
pub fn delete_watch_room_message(
    conn: &mut DbConnection,
    id: i64,
) -> Result<usize, diesel::result::Error> {
    let deleted = diesel::delete(watch_room_messages::table.filter(watch_room_messages::id.eq(id)))
        .execute(conn)?;
    Ok(deleted)
}

/// Delete all messages from a room (e.g. when the room is closed)
pub fn delete_messages_by_room(
    conn: &mut DbConnection,
    room_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    let deleted =
        diesel::delete(watch_room_messages::table.filter(watch_room_messages::room_id.eq(room_id)))
            .execute(conn)?;
    Ok(deleted)
}
