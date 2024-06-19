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

    match sdk.authenticate(email.clone(), password.clone()).await {
        Ok(_) => info!("Login successful"),
        Err(err) => info!("Login failed: {}", err),
    }

    // match sdk.get_portfolio().await {
    //     Ok(_) => info!("Portfolio retrieved successfully"),
    //     Err(err) => info!("Failed to retrieve portfolio: {}", err),
    // }
}
