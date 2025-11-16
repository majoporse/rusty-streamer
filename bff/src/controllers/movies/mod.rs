use movies_client::apis::configuration::Configuration;
use reqwest_middleware::ClientBuilder;
use shared::http_logging_middleware::LoggingMiddleware;

pub mod people_controller;
pub mod movies_controller;
pub mod reviews_controller;
pub mod pagination;

fn client_config() -> Configuration {
    let mut config = Configuration::default();
    let reqwest_client = reqwest::Client::builder().build().unwrap();

    let client = ClientBuilder::new(reqwest_client)
        .with(LoggingMiddleware)
        .build();
    config.client = client;

    let movies_uri = std::env::var("MOVIES_URI").expect("MOVIES_URI not specified");
    config.base_path = movies_uri;
    log::info!("Movies service URI: {}", config.base_path);

    config
}
