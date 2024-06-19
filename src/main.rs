pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub use presentation::sdk::SDK;
// dotenv
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let email = env::var("EMAIL").expect("EMAIL must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");

    let sdk = SDK::new();

    match sdk.authenticate(email, password).await {
        Ok(token) => {
            info!("Authenticated successfully: {:?}", token);
            match sdk.get_portfolio().await {
                Ok(portfolio) => info!("Retrieved portfolio: {:?}", portfolio),
                Err(err) => info!("Failed to retrieve portfolio: {}", err),
            }
        }
        Err(err) => info!("Authentication failed: {}", err),
    }
}
