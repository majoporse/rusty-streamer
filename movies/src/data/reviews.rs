use diesel::{ExpressionMethods, QueryDsl};
use uuid::Uuid;
use crate::{
    data::models::{
        review::{NewReview, Review},
        DbConnection,
    },
    schema::reviews,
};
use diesel_async::RunQueryDsl;

pub async fn get_review_by_id(conn: &mut DbConnection, review_id: Uuid) -> anyhow::Result<Review, diesel::result::Error> {
    let review_item = reviews::table
        .filter(reviews::id.eq(review_id))
        .first::<Review>(conn).await?;

    Ok(review_item)
}

pub async fn create_review(conn: &mut DbConnection, new_review: NewReview) -> anyhow::Result<Review, diesel::result::Error> {
    let review_item = diesel::insert_into(reviews::table)
        .values(new_review)
        .get_result::<Review>(conn).await?;

    Ok(review_item)
}

pub async fn list_reviews(
    conn: &mut DbConnection,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Review>, diesel::result::Error> {
    let review_items = reviews::table
        .limit(limit)
        .offset(offset)
        .load::<Review>(conn).await?;

    Ok(review_items)
}

pub async fn delete_review(conn: &mut DbConnection, review_id: Uuid) -> anyhow::Result<usize, diesel::result::Error> {
    let deleted_rows =
        diesel::delete(reviews::table.filter(reviews::id.eq(review_id))).execute(conn).await?;

    Ok(deleted_rows)
}

pub async fn update_review(
    conn: &mut DbConnection,
    review_id: Uuid,
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
        .get_result::<Review>(conn).await?;

    Ok(review_item)
}


pub async fn get_reviews_by_movie_id(
    conn: &mut DbConnection,
    movie_id: Uuid,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Review>, diesel::result::Error> {
    let review_items = reviews::table
        .filter(reviews::movie_id.eq(movie_id))
        .limit(limit)
        .offset(offset)
        .load::<Review>(conn).await?;

    Ok(review_items)
}

pub async fn get_reviews_by_user_id(
    conn: &mut DbConnection,
    user_name: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<Review>, diesel::result::Error> {
    let review_items = reviews::table
        .filter(reviews::user_name.eq(user_name))
        .limit(limit)
        .offset(offset)
        .load::<Review>(conn).await?;

    Ok(review_items)
}
