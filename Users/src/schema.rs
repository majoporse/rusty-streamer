// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        display_name -> Varchar,
        profile_picture_url -> Nullable<Text>,
        #[max_length = 5]
        country -> Nullable<Varchar>,
        #[max_length = 5]
        language_preference -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        last_login_at -> Nullable<Timestamptz>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    watch_history (id) {
        id -> Int8,
        user_id -> Uuid,
        content_id -> Uuid,
        progress_seconds -> Nullable<Int4>,
        completed -> Nullable<Bool>,
        last_watched_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    watch_room_messages (id) {
        id -> Int8,
        room_id -> Uuid,
        user_id -> Nullable<Uuid>,
        message -> Text,
        sent_at -> Nullable<Timestamptz>,
        is_system_message -> Nullable<Bool>,
    }
}

diesel::table! {
    watch_room_participants (id) {
        id -> Int8,
        room_id -> Uuid,
        user_id -> Uuid,
        joined_at -> Nullable<Timestamptz>,
        last_active_at -> Nullable<Timestamptz>,
        is_host -> Nullable<Bool>,
    }
}

diesel::table! {
    watch_rooms (id) {
        id -> Uuid,
        host_user_id -> Uuid,
        content_id -> Uuid,
        #[max_length = 100]
        room_name -> Nullable<Varchar>,
        is_private -> Bool,
        #[max_length = 10]
        invite_code -> Nullable<Varchar>,
        current_time_seconds -> Nullable<Int4>,
        is_live -> Nullable<Bool>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    watchlist (id) {
        id -> Int8,
        user_id -> Uuid,
        content_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(watch_history -> users (user_id));
diesel::joinable!(watch_room_messages -> users (user_id));
diesel::joinable!(watch_room_messages -> watch_rooms (room_id));
diesel::joinable!(watch_room_participants -> users (user_id));
diesel::joinable!(watch_room_participants -> watch_rooms (room_id));
diesel::joinable!(watch_rooms -> users (host_user_id));
diesel::joinable!(watchlist -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    watch_history,
    watch_room_messages,
    watch_room_participants,
    watch_rooms,
    watchlist,
);