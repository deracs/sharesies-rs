pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use domain::entities::instrument::InstrumentRequest;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
pub use presentation::sdk::Sharesies;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let sdk = Sharesies::new();

    match sdk
        .authenticate(
            env::var("EMAIL").expect("EMAIL must be set"),
            env::var("PASSWORD").expect("PASSWORD must be set"),
        )
        .await
    {
        Ok(token) => {
            info!("Authenticated successfully: {:?}", token);
            match sdk.get_portfolio().await {
                Ok(portfolio) => println!("Retrieved portfolio: {:?}", portfolio),
                Err(err) => info!("Failed to retrieve portfolio: {}", err),
            }

            let instrument_request = InstrumentRequest::create(
                vec![
                    "a31d15e0-f08b-4e9d-8294-beb76a159346".to_string(),
                    "b8b7ef58-b270-4762-a256-9d68aebc3e23".to_string(),
                    "24f86dd0-5869-4992-8398-4746e44e8d0f".to_string(),
                    "af87fb44-ebf6-4239-ba08-ae1cc9a6461c".to_string(),
                    "67a17798-c341-4af3-a06e-5a621d342adb".to_string(),
                    "5462d6d3-ad50-441a-a15b-e00d8bb37e17".to_string(),
                    "5df0acd5-3e08-41ca-b8ce-1046f39acd41".to_string(),
                    "cb928f59-a818-4ea2-adc8-ec77779609c4".to_string(),
                    "84fb0b94-e7dd-4a48-996d-fd261b781c11".to_string(),
                    "1fa21793-40ff-47cc-99e1-0f94d4341e26".to_string(),
                    "8cd3115a-831b-4a5a-9cdd-a5d523c6814f".to_string(),
                    "3ab93925-eeb0-4de8-b003-1a34100a874d".to_string(),
                    "77866efa-d81e-4f71-beaf-371bb210ac8c".to_string(),
                ],
                Some(1),
                Some(10),
            );
            match sdk.get_instruments(instrument_request).await {
                Ok(instruments) => println!("Retrieved instruments: {:?}", instruments),
                Err(err) => info!("Failed to retrieve instruments: {}", err),
            }
        }
        Err(err) => info!("Authentication failed: {}", err),
    }
}
