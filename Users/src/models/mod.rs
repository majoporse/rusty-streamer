pub mod user;
pub mod watch_history;
pub mod watch_list;
pub mod watch_room_message;
pub mod watch_room_participant;
pub mod watch_room;

pub type DbConnection = diesel::pg::PgConnection;
