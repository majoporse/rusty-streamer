pub mod movie;
pub mod person;
pub mod review;
pub mod movie_crew;
pub mod genre;
pub mod movie_genres;


pub type DbConnection = diesel_async::AsyncPgConnection;
