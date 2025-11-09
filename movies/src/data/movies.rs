use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl as _};

use crate::{models::{DbConnection, movie::{Movie, NewMovie}}, schema::movies};



pub fn get_movie_by_id(
    conn: &mut DbConnection,
    movie_id: i32,
) -> anyhow::Result<Movie> {

    let movie_item = movies::table
        .filter(movies::id.eq(movie_id))
        .first::<Movie>(conn)?;

    Ok(movie_item)
}

pub fn create_movie(
    conn: &mut DbConnection,
    new_movie: &NewMovie,
) -> anyhow::Result<Movie> {

    let movie_item = diesel::insert_into(movies::table)
        .values(new_movie)
        .get_result::<Movie>(conn)?;

    Ok(movie_item)
}

pub fn list_movies(
    conn: &mut DbConnection,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Movie>> {

    let movie_items = movies::table
        .limit(limit)
        .offset(offset)
        .load::<Movie>(conn)?;

    Ok(movie_items)
}


pub fn delete_movie(
    conn: &mut DbConnection,
    movie_id: i32,
) -> anyhow::Result<usize> {

    let deleted_rows = diesel::delete(movies::table.filter(movies::id.eq(movie_id)))
        .execute(conn)?;

    Ok(deleted_rows)
}

pub fn update_movie(
    conn: &mut DbConnection,
    movie_id: i32,
    updated_movie: &crate::models::movie::NewMovie,
) -> anyhow::Result<Movie> {

    let movie_item = diesel::update(movies::table.filter(movies::id.eq(movie_id)))
        .set((
            movies::title.eq(updated_movie.title),
            movies::slug.eq(updated_movie.slug),
            movies::description.eq(updated_movie.description),
            movies::release_date.eq(updated_movie.release_date),
            movies::duration_minutes.eq(updated_movie.duration_minutes),
            movies::mpaa_rating.eq(updated_movie.mpaa_rating),
        ))
        .get_result::<Movie>(conn)?;

    Ok(movie_item)
}