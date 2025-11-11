use diesel::{
    BoolExpressionMethods as _, ExpressionMethods, PgTextExpressionMethods as _, QueryDsl,
    RunQueryDsl,
};

use crate::{
    models::{
        actor::{Actor, NewActor},
        DbConnection,
    },
    schema::actors,
};

pub fn get_actor_by_id(conn: &mut DbConnection, actor_id: i32) -> anyhow::Result<Actor> {
    let actor_item = actors::table
        .filter(actors::id.eq(actor_id))
        .first::<Actor>(conn)?;

    Ok(actor_item)
}

pub fn create_actor(conn: &mut DbConnection, new_actor: NewActor) -> anyhow::Result<Actor> {
    let actor_item = diesel::insert_into(actors::table)
        .values(new_actor)
        .get_result::<Actor>(conn)?;

    Ok(actor_item)
}

pub fn list_actors(conn: &mut DbConnection, limit: i64, offset: i64) -> anyhow::Result<Vec<Actor>> {
    let actor_items = actors::table
        .limit(limit)
        .offset(offset)
        .load::<Actor>(conn)?;

    Ok(actor_items)
}

pub fn delete_actor(conn: &mut DbConnection, actor_id: i32) -> anyhow::Result<usize> {
    let deleted_rows =
        diesel::delete(actors::table.filter(actors::id.eq(actor_id))).execute(conn)?;

    Ok(deleted_rows)
}

pub fn update_actor(
    conn: &mut DbConnection,
    actor_id: i32,
    updated_actor: NewActor,
) -> anyhow::Result<Actor> {
    let actor_item = diesel::update(actors::table.filter(actors::id.eq(actor_id)))
        .set((
            actors::first_name.eq(updated_actor.first_name),
            actors::last_name.eq(updated_actor.last_name),
            actors::birth_date.eq(updated_actor.birth_date),
            actors::bio.eq(updated_actor.bio),
        ))
        .get_result::<Actor>(conn)?;

    Ok(actor_item)
}

pub fn search_actors_by_name(
    conn: &mut DbConnection,
    name_query: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Actor>> {
    let pattern = format!("%{}%", name_query);

    let actor_items = actors::table
        .filter(
            actors::first_name
                .ilike(&pattern)
                .or(actors::last_name.ilike(&pattern)),
        )
        .limit(limit)
        .offset(offset)
        .load::<Actor>(conn)?;

    Ok(actor_items)
}
