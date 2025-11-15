use movies_client::apis::configuration::Configuration;
use reqwest_middleware::ClientBuilder;
use shared::http_logging_middleware::LoggingMiddleware;

pub mod actors;
pub mod movies;
pub mod reviews;

fn client_config() -> Configuration {
    let mut config = Configuration::default();
    let movies_uri = std::env::var("MOVIES_URI").expect("MOVIES_URI not specified");

    let reqwest_client = reqwest::Client::builder().build().unwrap();

    let client = ClientBuilder::new(reqwest_client)
        .with(LoggingMiddleware)
        .build();

    config.client = client;
    config.base_path = movies_uri;

    config
}
