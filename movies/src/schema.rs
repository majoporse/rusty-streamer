// @generated automatically by Diesel CLI.

diesel::table! {
    actors (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        birth_date -> Nullable<Date>,
        bio -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    movie_actors (movie_id, actor_id) {
        movie_id -> Int4,
        actor_id -> Int4,
        character_name -> Nullable<Text>,
        billing_order -> Nullable<Int4>,
    }
}

diesel::table! {
    movies (id) {
        id -> Int4,
        title -> Text,
        slug -> Text,
        description -> Nullable<Text>,
        release_date -> Nullable<Date>,
        duration_minutes -> Nullable<Int4>,
        mpaa_rating -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        movie_id -> Int4,
        user_name -> Text,
        rating -> Int2,
        title -> Nullable<Text>,
        body -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(movie_actors -> actors (actor_id));
diesel::joinable!(movie_actors -> movies (movie_id));
diesel::joinable!(reviews -> movies (movie_id));

diesel::allow_tables_to_appear_in_same_query!(actors, movie_actors, movies, reviews,);
