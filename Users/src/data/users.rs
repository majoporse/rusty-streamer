use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, PgTextExpressionMethods};
use anyhow::Result;
use crate::{
    models::user::{User, NewUser, UpdateUser},
    schema::users,
    DbConnection,
};

pub fn create_user(conn: &mut DbConnection, new_user: NewUser) -> Result<User, diesel::result::Error> {
    let user = diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(conn)?;
    Ok(user)
}

pub fn get_user_by_id(conn: &mut DbConnection, user_id: uuid::Uuid) -> Result<User, diesel::result::Error> {
    let user = users::table
        .filter(users::id.eq(user_id))
        .first::<User>(conn)?;
    Ok(user)
}

pub fn get_user_by_email(conn: &mut DbConnection, email: &str) -> Result<User, diesel::result::Error> {
    let user = users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)?;
    Ok(user)
}

pub fn search_users_by_username(conn: &mut DbConnection, query: &str, limit: i64, offset: i64) -> Result<Vec<User>, diesel::result::Error> {
    let pattern = format!("%{}%", query);
    let users_list = users::table
        .filter(users::username.ilike(&pattern))
        .limit(limit)
        .offset(offset)
        .load::<User>(conn)?;
    Ok(users_list)
}

pub fn list_users(conn: &mut DbConnection, limit: i64, offset: i64) -> Result<Vec<User>, diesel::result::Error> {
    let items = users::table
        .limit(limit)
        .offset(offset)
        .load::<User>(conn)?;
    Ok(items)
}

pub fn update_user(conn: &mut DbConnection, user_id: uuid::Uuid, updated: UpdateUser) -> Result<User, diesel::result::Error> {
    let user = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set(&updated)
        .get_result::<User>(conn)?;
    Ok(user)
}

pub fn delete_user(conn: &mut DbConnection, user_id: uuid::Uuid) -> Result<usize, diesel::result::Error> {
    let deleted = diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)?;
    Ok(deleted)
}
