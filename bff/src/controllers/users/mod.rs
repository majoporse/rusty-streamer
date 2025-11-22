use users_client::apis::configuration::Configuration;

pub mod error;
pub mod pagination;
pub mod users_controller;
pub mod watch_history_controller;
pub mod watch_list_controller;
pub mod watch_room_controller;
pub mod watch_room_messages_controller;
pub mod watch_room_participants_controller;
pub mod auth;

use shared::http_logging_middleware::LoggingMiddleware;

use reqwest_middleware::ClientBuilder;

fn client_config() -> Configuration {
    let mut config = Configuration::default();
    let reqwest_client = reqwest::Client::builder().build().unwrap();

    let client = ClientBuilder::new(reqwest_client)
        .with(LoggingMiddleware)
        .build();
    config.client = client;

    let users_uri = std::env::var("USERS_URI").expect("USERS_URI not specified");
    config.base_path = users_uri;

    config
}

 fn redis_conn() -> redis::Client {
    let redis_str = std::env::var("REDIS_URI").expect("REDIS_STR not specified");
    redis::Client::open(redis_str).expect("Failed to create Redis client")
}
