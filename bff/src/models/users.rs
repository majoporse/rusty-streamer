use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use users_client::models as client_models;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct WatchRoom {
    pub id: Uuid,
    pub host_user_id: Uuid,
    pub content_id: Uuid,
    pub room_name: Option<Option<String>>,
    pub is_private: bool,
    pub invite_code: Option<Option<String>>,
    pub current_time_seconds: Option<Option<i32>>,
    pub is_live: Option<Option<bool>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<Option<NaiveDateTime>>,
}

impl From<client_models::WatchRoom> for WatchRoom {
    fn from(v: client_models::WatchRoom) -> Self {
        WatchRoom {
            id: v.id,
            host_user_id: v.host_user_id,
            content_id: v.content_id,
            room_name: v.room_name,
            is_private: v.is_private,
            invite_code: v.invite_code,
            current_time_seconds: v.current_time_seconds,
            is_live: v.is_live,
            created_at: v.created_at.parse().unwrap(),
            updated_at: v.updated_at.map(|dt| dt.map(|d| d.parse().unwrap())),
        }
    }
}

impl From<WatchRoom> for client_models::WatchRoom {
    fn from(w: WatchRoom) -> Self {
        client_models::WatchRoom {
            id: w.id,
            host_user_id: w.host_user_id,
            content_id: w.content_id,
            room_name: w.room_name,
            is_private: w.is_private,
            invite_code: w.invite_code,
            current_time_seconds: w.current_time_seconds,
            is_live: w.is_live,
            created_at: w.created_at.to_string(),
            updated_at: w.updated_at.map(|e| e.map(|ee| ee.to_string())),
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NewWatchRoom {
    pub host_user_id: Uuid,
    pub content_id: Uuid,
    pub room_name: Option<Option<String>>,
    pub is_private: Option<Option<bool>>,
    pub invite_code: Option<Option<String>>,
}

impl From<client_models::NewWatchRoom> for NewWatchRoom {
    fn from(v: client_models::NewWatchRoom) -> Self {
        NewWatchRoom {
            host_user_id: v.host_user_id,
            content_id: v.content_id,
            room_name: v.room_name,
            is_private: v.is_private,
            invite_code: v.invite_code,
        }
    }
}

impl From<NewWatchRoom> for client_models::NewWatchRoom {
    fn from(w: NewWatchRoom) -> Self {
        client_models::NewWatchRoom {
            host_user_id: w.host_user_id,
            content_id: w.content_id,
            room_name: w.room_name,
            is_private: w.is_private,
            invite_code: w.invite_code,
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWatchRoom {
    pub current_time_seconds: Option<Option<i32>>,
    pub is_live: Option<Option<bool>>,
}

impl From<client_models::UpdateWatchRoom> for UpdateWatchRoom {
    fn from(v: client_models::UpdateWatchRoom) -> Self {
        UpdateWatchRoom {
            current_time_seconds: v.current_time_seconds,
            is_live: v.is_live,
        }
    }
}

impl From<UpdateWatchRoom> for client_models::UpdateWatchRoom {
    fn from(w: UpdateWatchRoom) -> Self {
        client_models::UpdateWatchRoom {
            current_time_seconds: w.current_time_seconds,
            is_live: w.is_live,
        }
    }
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct WatchRoomParticipant {
    pub id: i64,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: Option<Option<NaiveDateTime>>,
    pub last_active_at: Option<Option<NaiveDateTime>>,
    pub is_host: Option<Option<bool>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NewWatchRoomParticipant {
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub is_host: Option<Option<bool>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct WatchRoomMessage {
    pub id: i64,
    pub room_id: Uuid,
    pub user_id: Option<Option<Uuid>>,
    pub message: String,
    pub sent_at: Option<Option<NaiveDateTime>>,
    pub is_system_message: Option<Option<bool>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NewWatchRoomMessage {
    pub room_id: Uuid,
    pub user_id: Option<Option<Uuid>>,
    pub message: String,
    pub is_system_message: Option<Option<bool>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct Watchlist {
    pub id: i64,
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NewWatchlist {
    pub user_id: Uuid,
    pub content_id: Uuid,
}
// TODO ADD MOVIE NAME FOR CACHE
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct WatchHistory {
    pub id: i64,
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub progress_seconds: Option<Option<i32>>,
    pub completed: Option<Option<bool>>,
    pub last_watched_at: Option<Option<NaiveDateTime>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NewWatchHistory {
    pub user_id: Uuid,
    pub content_id: Uuid,
    pub progress_seconds: Option<Option<i32>>,
    pub completed: Option<Option<bool>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateWatchHistory {
    pub progress_seconds: Option<Option<i32>>,
    pub completed: Option<Option<bool>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub profile_picture_url: Option<Option<String>>,
    pub country: Option<Option<String>>,
    pub language_preference: Option<Option<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<Option<NaiveDateTime>>,
    pub last_login_at: Option<Option<NaiveDateTime>>,
    pub status: Option<Option<String>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<Option<String>>,
    pub profile_picture_url: Option<Option<String>>,
    pub country: Option<Option<String>>,
    pub language_preference: Option<Option<String>>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub display_name: Option<Option<String>>,
    pub profile_picture_url: Option<Option<String>>,
    pub language_preference: Option<Option<String>>,
    pub country: Option<Option<String>>,
    pub status: Option<Option<String>>,
}

/// WatchRoomParticipant
impl From<client_models::WatchRoomParticipant> for WatchRoomParticipant {
    fn from(v: client_models::WatchRoomParticipant) -> Self {
        WatchRoomParticipant {
            id: v.id,
            room_id: v.room_id,
            user_id: v.user_id,
            joined_at: v.joined_at.map(|dt| dt.map(|d| d.parse().unwrap())),
            last_active_at: v.last_active_at.map(|dt| dt.map(|d| d.parse().unwrap())),
            is_host: v.is_host,
        }
    }
}

impl From<WatchRoomParticipant> for client_models::WatchRoomParticipant {
    fn from(w: WatchRoomParticipant) -> Self {
        client_models::WatchRoomParticipant {
            id: w.id,
            room_id: w.room_id,
            user_id: w.user_id,
            joined_at: w.joined_at.map(|dt| dt.map(|d| d.to_string())),
            last_active_at: w.last_active_at.map(|dt| dt.map(|d| d.to_string())),
            is_host: w.is_host,
        }
    }
}

/// NewWatchRoomParticipant
impl From<client_models::NewWatchRoomParticipant> for NewWatchRoomParticipant {
    fn from(v: client_models::NewWatchRoomParticipant) -> Self {
        NewWatchRoomParticipant {
            room_id: v.room_id,
            user_id: v.user_id,
            is_host: v.is_host,
        }
    }
}

impl From<NewWatchRoomParticipant> for client_models::NewWatchRoomParticipant {
    fn from(w: NewWatchRoomParticipant) -> Self {
        client_models::NewWatchRoomParticipant {
            room_id: w.room_id,
            user_id: w.user_id,
            is_host: w.is_host,
        }
    }
}

/// WatchRoomMessage
impl From<client_models::WatchRoomMessage> for WatchRoomMessage {
    fn from(v: client_models::WatchRoomMessage) -> Self {
        WatchRoomMessage {
            id: v.id,
            room_id: v.room_id,
            user_id: v.user_id,
            message: v.message,
            sent_at: v.sent_at.map(|dt| dt.map(|d| d.parse().unwrap())),
            is_system_message: v.is_system_message,
        }
    }
}

impl From<WatchRoomMessage> for client_models::WatchRoomMessage {
    fn from(w: WatchRoomMessage) -> Self {
        client_models::WatchRoomMessage {
            id: w.id,
            room_id: w.room_id,
            user_id: w.user_id,
            message: w.message,
            sent_at: w.sent_at.map(|dt| dt.map(|d| d.to_string())),
            is_system_message: w.is_system_message,
        }
    }
}

/// NewWatchRoomMessage
impl From<client_models::NewWatchRoomMessage> for NewWatchRoomMessage {
    fn from(v: client_models::NewWatchRoomMessage) -> Self {
        NewWatchRoomMessage {
            room_id: v.room_id,
            user_id: v.user_id,
            message: v.message,
            is_system_message: v.is_system_message,
        }
    }
}

impl From<NewWatchRoomMessage> for client_models::NewWatchRoomMessage {
    fn from(w: NewWatchRoomMessage) -> Self {
        client_models::NewWatchRoomMessage {
            room_id: w.room_id,
            user_id: w.user_id,
            message: w.message,
            is_system_message: w.is_system_message,
        }
    }
}

/// Watchlist
impl From<client_models::Watchlist> for Watchlist {
    fn from(v: client_models::Watchlist) -> Self {
        Watchlist {
            id: v.id,
            user_id: v.user_id,
            content_id: v.content_id,
            created_at: v.created_at.parse().unwrap(),
        }
    }
}

impl From<Watchlist> for client_models::Watchlist {
    fn from(w: Watchlist) -> Self {
        client_models::Watchlist {
            id: w.id,
            user_id: w.user_id,
            content_id: w.content_id,
            created_at: w.created_at.to_string(),
        }
    }
}

/// NewWatchlist
impl From<client_models::NewWatchlist> for NewWatchlist {
    fn from(v: client_models::NewWatchlist) -> Self {
        NewWatchlist {
            user_id: v.user_id,
            content_id: v.content_id,
        }
    }
}

impl From<NewWatchlist> for client_models::NewWatchlist {
    fn from(w: NewWatchlist) -> Self {
        client_models::NewWatchlist {
            user_id: w.user_id,
            content_id: w.content_id,
        }
    }
}

/// WatchHistory
impl From<client_models::WatchHistory> for WatchHistory {
    fn from(v: client_models::WatchHistory) -> Self {
        WatchHistory {
            id: v.id,
            user_id: v.user_id,
            content_id: v.content_id,
            progress_seconds: v.progress_seconds,
            completed: v.completed,
            last_watched_at: v.last_watched_at.map(|dt| dt.map(|d| d.parse().unwrap())),
        }
    }
}

impl From<WatchHistory> for client_models::WatchHistory {
    fn from(w: WatchHistory) -> Self {
        client_models::WatchHistory {
            id: w.id,
            user_id: w.user_id,
            content_id: w.content_id,
            progress_seconds: w.progress_seconds,
            completed: w.completed,
            last_watched_at: w.last_watched_at.map(|dt| dt.map(|d| d.to_string())),
        }
    }
}

/// NewWatchHistory
impl From<client_models::NewWatchHistory> for NewWatchHistory {
    fn from(v: client_models::NewWatchHistory) -> Self {
        NewWatchHistory {
            user_id: v.user_id,
            content_id: v.content_id,
            progress_seconds: v.progress_seconds,
            completed: v.completed,
        }
    }
}

impl From<NewWatchHistory> for client_models::NewWatchHistory {
    fn from(w: NewWatchHistory) -> Self {
        client_models::NewWatchHistory {
            user_id: w.user_id,
            content_id: w.content_id,
            progress_seconds: w.progress_seconds,
            completed: w.completed,
        }
    }
}

/// UpdateWatchHistory
impl From<client_models::UpdateWatchHistory> for UpdateWatchHistory {
    fn from(v: client_models::UpdateWatchHistory) -> Self {
        UpdateWatchHistory {
            progress_seconds: v.progress_seconds,
            completed: v.completed,
        }
    }
}

impl From<UpdateWatchHistory> for client_models::UpdateWatchHistory {
    fn from(w: UpdateWatchHistory) -> Self {
        client_models::UpdateWatchHistory {
            progress_seconds: w.progress_seconds,
            completed: w.completed,
        }
    }
}

/// User
impl From<client_models::User> for User {
    fn from(v: client_models::User) -> Self {
        User {
            id: v.id,
            username: v.username,
            email: v.email,
            password_hash: v.password_hash,
            display_name: v.display_name,
            profile_picture_url: v.profile_picture_url,
            country: v.country,
            language_preference: v.language_preference,
            created_at: v.created_at.parse().unwrap(),
            updated_at: v.updated_at.map(|e| e.map(|ee| ee.parse().unwrap())),
            last_login_at: v.last_login_at.map(|dt| dt.map(|d| d.parse().unwrap())),
            status: v.status,
        }
    }
}

impl From<User> for client_models::User {
    fn from(w: User) -> Self {
        client_models::User {
            id: w.id,
            username: w.username,
            email: w.email,
            password_hash: w.password_hash,
            display_name: w.display_name,
            profile_picture_url: w.profile_picture_url,
            country: w.country,
            language_preference: w.language_preference,
            created_at: w.created_at.to_string(),
            updated_at: w.updated_at.map(|e| e.map(|ee| ee.to_string())),
            last_login_at: w.last_login_at.map(|dt| dt.map(|d| d.to_string())),
            status: w.status,
        }
    }
}

/// NewUser
impl From<client_models::NewUser> for NewUser {
    fn from(v: client_models::NewUser) -> Self {
        NewUser {
            username: v.username,
            email: v.email,
            password_hash: v.password_hash,
            display_name: v.display_name,
            profile_picture_url: v.profile_picture_url,
            country: v.country,
            language_preference: v.language_preference,
        }
    }
}

impl From<NewUser> for client_models::NewUser {
    fn from(w: NewUser) -> Self {
        client_models::NewUser {
            username: w.username,
            email: w.email,
            password_hash: w.password_hash,
            display_name: w.display_name,
            profile_picture_url: w.profile_picture_url,
            country: w.country,
            language_preference: w.language_preference,
        }
    }
}

/// UpdateUser
impl From<client_models::UpdateUser> for UpdateUser {
    fn from(v: client_models::UpdateUser) -> Self {
        UpdateUser {
            display_name: v.display_name,
            profile_picture_url: v.profile_picture_url,
            language_preference: v.language_preference,
            country: v.country,
            status: v.status,
        }
    }
}

impl From<UpdateUser> for client_models::UpdateUser {
    fn from(w: UpdateUser) -> Self {
        client_models::UpdateUser {
            display_name: w.display_name,
            profile_picture_url: w.profile_picture_url,
            language_preference: w.language_preference,
            country: w.country,
            status: w.status,
        }
    }
}
