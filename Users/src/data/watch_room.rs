use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    models::{
        DbConnection,
        watch_room::{NewWatchRoom, UpdateWatchRoom, WatchRoom},
    },
    schema::watch_rooms,
};

/// Create a new watch room
pub fn create_watch_room(
    conn: &mut DbConnection,
    new_room: NewWatchRoom,
) -> Result<WatchRoom, diesel::result::Error> {
    let room = diesel::insert_into(watch_rooms::table)
        .values(new_room)
        .get_result::<WatchRoom>(conn)?;
    Ok(room)
}

/// Get a single watch room by its ID
pub fn get_watch_room_by_id(
    conn: &mut DbConnection,
    room_id: Uuid,
) -> Result<WatchRoom, diesel::result::Error> {
    let room = watch_rooms::table
        .filter(watch_rooms::id.eq(room_id))
        .first::<WatchRoom>(conn)?;
    Ok(room)
}

/// List all rooms hosted by a specific user
pub fn list_rooms_by_host(
    conn: &mut DbConnection,
    host_user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WatchRoom>, diesel::result::Error> {
    let rooms = watch_rooms::table
        .filter(watch_rooms::host_user_id.eq(host_user_id))
        .order(watch_rooms::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<WatchRoom>(conn)?;
    Ok(rooms)
}

/// Find a room by its invite code (for joining private rooms)
pub fn get_room_by_invite_code(
    conn: &mut DbConnection,
    code: &str,
) -> Result<WatchRoom, diesel::result::Error> {
    let room = watch_rooms::table
        .filter(watch_rooms::invite_code.eq(code))
        .first::<WatchRoom>(conn)?;
    Ok(room)
}

/// Update watch room state (partial update)
pub fn update_watch_room(
    conn: &mut DbConnection,
    room_id: Uuid,
    update_data: UpdateWatchRoom,
) -> Result<WatchRoom, diesel::result::Error> {
    use diesel::dsl::now;

    let query = diesel::update(watch_rooms::table.filter(watch_rooms::id.eq(room_id)))
        .set(watch_rooms::updated_at.eq(now))
        .into_boxed();

    // // Apply optional updates
    // if let Some(time) = update_data.current_time_seconds {
    //     query = query.set(watch_rooms::current_time_seconds.eq(time));
    // }
    // if let Some(is_live) = update_data.is_live {
    //     query = query.set(watch_rooms::is_live.eq(is_live));
    // }

    let updated_room = query.get_result::<WatchRoom>(conn)?;
    Ok(updated_room)
}

/// Delete a watch room (e.g. when ended)
pub fn delete_watch_room(
    conn: &mut DbConnection,
    room_id: Uuid,
) -> Result<usize, diesel::result::Error> {
    let deleted =
        diesel::delete(watch_rooms::table.filter(watch_rooms::id.eq(room_id))).execute(conn)?;
    Ok(deleted)
}
