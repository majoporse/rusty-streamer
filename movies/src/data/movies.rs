use diesel_async::RunQueryDsl;

use crate::{
    data::models::{
        DbConnection, genre::Genre, movie::{Movie, MovieDetails, NewMovie}, movie_crew::{MovieCrew, MovieCrewDetail, NewMovieCrew}, movie_genres::{MovieGenre, NewMovieGenre}, person::Person, review::Review
    },
    schema::{genres, movie_crew, movies, people, reviews},
};
use diesel::{
    associations::HasTable, BelongingToDsl, BoolExpressionMethods, ExpressionMethods, JoinOnDsl,
    PgTextExpressionMethods, QueryDsl, SelectableHelper,
};
use uuid::Uuid;

pub async fn get_movie_by_id(
    conn: &mut DbConnection,
    movie_id: Uuid,
) -> anyhow::Result<Movie, diesel::result::Error> {
    let movie_item = movies::table
        .filter(movies::id.eq(movie_id))
        .first::<Movie>(conn)
        .await?;

    Ok(movie_item)
}

pub async fn create_movie(
    conn: &mut DbConnection,
    new_movie: NewMovie,
) -> anyhow::Result<Movie, diesel::result::Error> {
    let movie_item = diesel::insert_into(movies::table)
        .values(new_movie)
        .get_result::<Movie>(conn)
        .await?;

    Ok(movie_item)
}

pub async fn add_genres_to_movie(
    conn: &mut DbConnection,
    movie_id: Uuid,
    genre_ids: Vec<Uuid>,
) -> anyhow::Result<(), diesel::result::Error> {
    let new_movie_genres: Vec<NewMovieGenre> = genre_ids
        .into_iter()
        .map(|genre_id| NewMovieGenre { movie_id, genre_id })
        .collect();

    diesel::insert_into(MovieGenre::table())
        .values(&new_movie_genres)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn add_people_to_movie(
    conn: &mut DbConnection,
    people_ids: Vec<NewMovieCrew>,
) -> anyhow::Result<(), diesel::result::Error> {

    diesel::insert_into(MovieCrew::table())
        .values(&people_ids)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn list_movies(
    conn: &mut DbConnection,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Movie>, diesel::result::Error> {
    let movie_items = movies::table
        .limit(limit)
        .offset(offset)
        .load::<Movie>(conn)
        .await?;

    Ok(movie_items)
}

pub async fn get_movie_details_by_id(
    conn: &mut DbConnection,
    movie_id: Uuid,
) -> anyhow::Result<MovieDetails, diesel::result::Error> {
    let movie: Movie = movies::table.find(movie_id).first(conn).await?;
    let crew: Vec<(Person, MovieCrew)> = MovieCrew::belonging_to(&movie)
        .inner_join(people::table)
        .select((Person::as_select(), MovieCrew::as_select()))
        .load(conn)
        .await?;

    let genres: Vec<Genre> = MovieGenre::belonging_to(&movie)
        .inner_join(genres::table)
        .select(genres::all_columns)
        .load(conn)
        .await?;

    let reviews: Vec<Review> = Review::belonging_to(&movie)
        .order(reviews::created_at.desc())
        .limit(5)
        .load(conn)
        .await?;

    let mut res = MovieDetails::from(movie);
    res.people = crew
        .into_iter()
        .map(|(person, crew)| MovieCrewDetail {
            movie_id,
            person_id: person.id,
            character_name: crew.character_name,
            billing_order: crew.billing_order,
            person,
        })
        .collect();

    res.genres = genres;
    res.reviews = reviews;

    Ok(res)
}

pub async fn delete_movie(
    conn: &mut DbConnection,
    movie_id: Uuid,
) -> anyhow::Result<usize, diesel::result::Error> {
    let deleted_rows = diesel::delete(movies::table.filter(movies::id.eq(movie_id)))
        .execute(conn)
        .await?;

    Ok(deleted_rows)
}

pub async fn update_movie(
    conn: &mut DbConnection,
    movie_id: Uuid,
    updated_movie: NewMovie,
) -> anyhow::Result<Movie, diesel::result::Error> {
    let movie_item = diesel::update(movies::table.filter(movies::id.eq(movie_id)))
        .set((
            movies::title.eq(updated_movie.title),
            movies::slug.eq(updated_movie.slug),
            movies::description.eq(updated_movie.description),
            movies::release_date.eq(updated_movie.release_date),
            movies::duration_minutes.eq(updated_movie.duration_minutes),
            movies::mpaa_rating.eq(updated_movie.mpaa_rating),
        ))
        .get_result::<Movie>(conn)
        .await?;

    Ok(movie_item)
}

pub async fn search_movies_by_title(
    conn: &mut DbConnection,
    title_query: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Movie>, diesel::result::Error> {
    let pattern = format!("%{}%", title_query);

    let movie_items = movies::table
        .filter(movies::title.ilike(pattern))
        .limit(limit)
        .offset(offset)
        .load::<Movie>(conn)
        .await?;

    Ok(movie_items)
}

pub async fn search_movies_by_actor(
    conn: &mut DbConnection,
    actor_name_query: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Movie>, diesel::result::Error> {
    let pattern = format!("%{}%", actor_name_query);

    let movie_items = movies::table
        .inner_join(movie_crew::table.on(movies::id.eq(movie_crew::movie_id)))
        .inner_join(people::table.on(movie_crew::person_id.eq(people::id)))
        .filter(
            people::first_name
                .ilike(&pattern)
                .or(people::last_name.ilike(&pattern)),
        )
        .select(movies::all_columns)
        .distinct()
        .limit(limit)
        .offset(offset)
        .load::<Movie>(conn)
        .await?;

    Ok(movie_items)
}
