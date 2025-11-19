// @generated automatically by Diesel CLI.

diesel::table! {
    genres (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    movie_crew (movie_id, person_id) {
        movie_id -> Uuid,
        person_id -> Uuid,
        role -> Nullable<Text>,
        billing_order -> Nullable<Int4>,
    }
}

diesel::table! {
    movie_genres (movie_id, genre_id) {
        movie_id -> Uuid,
        genre_id -> Uuid,
    }
}

diesel::table! {
    movies (id) {
        id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        release_date -> Nullable<Date>,
        duration_minutes -> Nullable<Int4>,
        mpaa_rating -> Nullable<Text>,
        video_url -> Nullable<Text>,
        poster_url -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    people (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        birth_date -> Nullable<Date>,
        bio -> Nullable<Text>,
        role -> Nullable<Text>,
        image_url -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    reviews (id) {
        id -> Uuid,
        movie_id -> Uuid,
        user_name -> Text,
        user_id -> Uuid,
        rating -> Int2,
        title -> Nullable<Text>,
        body -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(movie_crew -> movies (movie_id));
diesel::joinable!(movie_crew -> people (person_id));
diesel::joinable!(movie_genres -> genres (genre_id));
diesel::joinable!(movie_genres -> movies (movie_id));
diesel::joinable!(reviews -> movies (movie_id));

diesel::allow_tables_to_appear_in_same_query!(
    genres,
    movie_crew,
    movie_genres,
    movies,
    people,
    reviews,
);
