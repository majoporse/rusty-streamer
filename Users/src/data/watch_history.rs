use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{DbConnection, watch_history::{NewWatchHistory, UpdateWatchHistory, WatchHistory}},
    schema::watch_history,
};

pub fn create_watch_history(
    conn: &mut DbConnection,
    new_item: NewWatchHistory,
) -> Result<WatchHistory, diesel::result::Error> {
    let item = diesel::insert_into(watch_history::table)
        .values(new_item)
        .get_result::<WatchHistory>(conn)?;
    Ok(item)
}

pub fn get_watch_history_by_id(
    conn: &mut DbConnection,
    id: i64,
) -> Result<WatchHistory, diesel::result::Error> {
    let item = watch_history::table
        .filter(watch_history::id.eq(id))
        .first::<WatchHistory>(conn)?;
    Ok(item)
}

pub fn list_watch_history_by_user(
    conn: &mut DbConnection,
    user_id: uuid::Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WatchHistory>, diesel::result::Error> {
    let items = watch_history::table
        .filter(watch_history::user_id.eq(user_id))
        .order(watch_history::last_watched_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<WatchHistory>(conn)?;
    Ok(items)
}

pub fn update_watch_history(
    conn: &mut DbConnection,
    id: i64,
    update_data: UpdateWatchHistory,
) -> Result<WatchHistory, diesel::result::Error> {
    let item = diesel::update(watch_history::table.filter(watch_history::id.eq(id)))
        .set((
            watch_history::progress_seconds.eq(update_data.progress_seconds),
            watch_history::completed.eq(update_data.completed),
            watch_history::last_watched_at.eq(diesel::dsl::now),
        ))
        .get_result::<WatchHistory>(conn)?;
    Ok(item)
}

pub fn delete_watch_history(
    conn: &mut DbConnection,
    id: i64,
) -> Result<usize, diesel::result::Error> {
    let deleted =
        diesel::delete(watch_history::table.filter(watch_history::id.eq(id))).execute(conn)?;
    Ok(deleted)
}
