use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{models::{DbConnection, watch_list::{NewWatchlist, Watchlist},}, schema::watchlist};

pub fn create_watchlist_item(conn: &mut DbConnection, new_item: NewWatchlist) -> Result<Watchlist, diesel::result::Error> {
    let item = diesel::insert_into(watchlist::table)
        .values(new_item)
        .get_result::<Watchlist>(conn)?;
    Ok(item)
}

pub fn get_watchlist_by_user(conn: &mut DbConnection, user_id: uuid::Uuid, limit: i64, offset: i64) -> Result<Vec<Watchlist>, diesel::result::Error> {
    let items = watchlist::table
        .filter(watchlist::user_id.eq(user_id))
        .order(watchlist::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<Watchlist>(conn)?;
    Ok(items)
}

pub fn delete_watchlist_item(conn: &mut DbConnection, user_id: uuid::Uuid, content_id: uuid::Uuid) -> Result<usize, diesel::result::Error> {
    let deleted = diesel::delete(
        watchlist::table
            .filter(watchlist::user_id.eq(user_id))
            .filter(watchlist::content_id.eq(content_id)),
    )
    .execute(conn)?;
    Ok(deleted)
}
