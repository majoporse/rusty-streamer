use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl as _};

use crate::{
    models::{
        review::{NewReview, Review},
        DbConnection,
    },
    schema::reviews,
};

pub fn get_review_by_id(conn: &mut DbConnection, review_id: i32) -> anyhow::Result<Review, diesel::result::Error> {
    let review_item = reviews::table
        .filter(reviews::id.eq(review_id))
        .first::<Review>(conn)?;

    Ok(review_item)
}

pub fn create_review(conn: &mut DbConnection, new_review: NewReview) -> anyhow::Result<Review, diesel::result::Error> {
    let review_item = diesel::insert_into(reviews::table)
        .values(new_review)
        .get_result::<Review>(conn)?;

    Ok(review_item)
}

pub fn list_reviews(
    conn: &mut DbConnection,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Review>, diesel::result::Error> {
    let review_items = reviews::table
        .limit(limit)
        .offset(offset)
        .load::<Review>(conn)?;

    Ok(review_items)
}

pub fn delete_review(conn: &mut DbConnection, review_id: i32) -> anyhow::Result<usize, diesel::result::Error> {
    let deleted_rows =
        diesel::delete(reviews::table.filter(reviews::id.eq(review_id))).execute(conn)?;

    Ok(deleted_rows)
}

pub fn update_review(
    conn: &mut DbConnection,
    review_id: i32,
    updated_review: NewReview,
) -> anyhow::Result<Review, diesel::result::Error> {
    let review_item = diesel::update(reviews::table.filter(reviews::id.eq(review_id)))
        .set((
            reviews::movie_id.eq(updated_review.movie_id),
            reviews::user_name.eq(updated_review.user_name),
            reviews::rating.eq(updated_review.rating),
            reviews::title.eq(updated_review.title),
            reviews::body.eq(updated_review.body),
        ))
        .get_result::<Review>(conn)?;

    Ok(review_item)
}
