pub mod movie;
pub mod actor;
pub mod review;
pub mod movie_actor;

pub type DbConnection = diesel::pg::PgConnection;
