use diesel::{
    BoolExpressionMethods as _, ExpressionMethods, JoinOnDsl, PgTextExpressionMethods as _, QueryDsl
};
use uuid::Uuid;
use diesel_async::RunQueryDsl;

use crate::{
    data::models::{
        DbConnection, person::{NewPerson, Person}
    },
    schema::{movie_crew, people},
};

pub async fn get_person_by_id(conn: &mut DbConnection, person_id: Uuid) -> anyhow::Result<Person, diesel::result::Error> {
    let person_item = people::table
        .filter(people::id.eq(person_id))
        .first::<Person>(conn).await?;

    Ok(person_item)
}

pub async fn create_person(conn: &mut DbConnection, new_person: NewPerson) -> anyhow::Result<Person, diesel::result::Error> {
    let person_item = diesel::insert_into(people::table)
        .values(new_person)
        .get_result::<Person>(conn).await?;

    Ok(person_item)
}

pub async fn list_people(conn: &mut DbConnection, limit: i64, offset: i64) -> anyhow::Result<Vec<Person>, diesel::result::Error> {
    let person_items = people::table
        .limit(limit)
        .offset(offset)
        .load::<Person>(conn).await?;

    Ok(person_items)
}

pub async fn delete_person(conn: &mut DbConnection, person_id: Uuid) -> anyhow::Result<usize, diesel::result::Error> {
    let deleted_rows =
        diesel::delete(people::table.filter(people::id.eq(person_id))).execute(conn).await?;

    Ok(deleted_rows)
}

pub async fn update_person(
    conn: &mut DbConnection,
    person_id: Uuid,
    updated_person: NewPerson,
) -> anyhow::Result<Person, diesel::result::Error> {
    let person_item = diesel::update(people::table.filter(people::id.eq(person_id)))
        .set((
            people::first_name.eq(updated_person.first_name),
            people::last_name.eq(updated_person.last_name),
            people::birth_date.eq(updated_person.birth_date),
            people::bio.eq(updated_person.bio),
        ))
        .get_result::<Person>(conn).await?;

    Ok(person_item)
}

pub async fn search_people_by_name(
    conn: &mut DbConnection,
    name_query: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Person>, diesel::result::Error> {
    let pattern = format!("%{}%", name_query);

    let person_items = people::table
        .filter(
            people::first_name
                .ilike(&pattern)
                .or(people::last_name.ilike(&pattern)),
        )
        .limit(limit)
        .offset(offset)
        .load::<Person>(conn).await?;

    Ok(person_items)
}

pub async fn get_people_by_movie_id(
    conn: &mut DbConnection,
    movie_id: Uuid,
) -> anyhow::Result<Vec<Person>, diesel::result::Error> {

    let person_items = people::table
        .inner_join(movie_crew::table.on(movie_crew::person_id.eq(people::id)))
        .filter(movie_crew::movie_id.eq(movie_id))
        .select(people::all_columns)
        .load::<Person>(conn).await?;

    Ok(person_items)
}
