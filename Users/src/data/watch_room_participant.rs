use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    models::{
        DbConnection,
        watch_room_participant::{NewWatchRoomParticipant, WatchRoomParticipant},
    },
    schema::watch_room_participants,
};

/// Add a new participant to a watch room
pub fn create_watch_room_participant(
    conn: &mut DbConnection,
    new_participant: NewWatchRoomParticipant,
) -> Result<WatchRoomParticipant, diesel::result::Error> {
    let participant = diesel::insert_into(watch_room_participants::table)
        .values(new_participant)
        .get_result::<WatchRoomParticipant>(conn)?;
    Ok(participant)
}

/// Get a participant by database ID
pub fn get_watch_room_participant_by_id(
    conn: &mut DbConnection,
    id: i64,
) -> Result<WatchRoomParticipant, diesel::result::Error> {
    let participant = watch_room_participants::table
        .filter(watch_room_participants::id.eq(id))
        .first::<WatchRoomParticipant>(conn)?;
    Ok(participant)
}

/// List all participants in a given room
pub fn list_participants_by_room(
    conn: &mut DbConnection,
    room_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WatchRoomParticipant>, diesel::result::Error> {
    let participants = watch_room_participants::table
        .filter(watch_room_participants::room_id.eq(room_id))
        .order(watch_room_participants::joined_at.asc())
        .limit(limit)
        .offset(offset)
        .load::<WatchRoomParticipant>(conn)?;
    Ok(participants)
}

/// Remove a participant from a room (e.g. when they leave)
pub fn delete_watch_room_participant(
    conn: &mut DbConnection,
    room_id: Uuid,
    user_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    let deleted = diesel::delete(
        watch_room_participants::table
            .filter(watch_room_participants::room_id.eq(room_id))
            .filter(watch_room_participants::user_id.eq(user_id)),
    )
    .execute(conn)?;
    Ok(deleted)
}

/// Delete all participants from a room (e.g. when room is closed)
pub fn delete_participants_by_room(
    conn: &mut DbConnection,
    room_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    let deleted = diesel::delete(
        watch_room_participants::table.filter(watch_room_participants::room_id.eq(room_id)),
    )
    .execute(conn)?;
    Ok(deleted)
}
