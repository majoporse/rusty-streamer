
use diesel::{ExpressionMethods, QueryDsl};
use uuid::Uuid;
use crate::{
    data::models::{
        DbConnection, genre::{Genre, NewGenre}
    },
    schema::genres,
};
use diesel_async::RunQueryDsl;

pub async fn get_genre(conn: &mut DbConnection, genre_id: Uuid) -> anyhow::Result<Genre, diesel::result::Error> {
    let genre_item = genres::table
        .filter(genres::id.eq(genre_id))
        .first::<Genre>(conn).await?;

    Ok(genre_item)
}

pub async fn create_genre(conn: &mut DbConnection, new_genre: NewGenre) -> anyhow::Result<Genre, diesel::result::Error> {
    let genre_item = diesel::insert_into(genres::table)
        .values(&new_genre)
        .get_result::<Genre>(conn).await?;

    Ok(genre_item)
}

pub async fn list_genres(
    conn: &mut DbConnection,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Genre>, diesel::result::Error> {
    let genre_items = genres::table
        .limit(limit)
        .offset(offset)
        .load::<Genre>(conn).await?;
    Ok(genre_items)
}

pub async fn delete_genre(conn: &mut DbConnection, genre_id: Uuid) -> anyhow::Result<usize, diesel::result::Error> {
    let deleted_rows =
        diesel::delete(genres::table.filter(genres::id.eq(genre_id))).execute(conn).await?;

    Ok(deleted_rows)
}

pub async fn update_genre(
    conn: &mut DbConnection,
    genre_id: Uuid,
    updated_genre: NewGenre,
) -> anyhow::Result<Genre, diesel::result::Error> {
    let genre_item = diesel::update(genres::table.filter(genres::id.eq(genre_id)))
        .set(genres::name.eq(updated_genre.name))
        .get_result::<Genre>(conn).await?;

    Ok(genre_item)
}
