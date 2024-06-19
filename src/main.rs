use dotenv::dotenv;
use env_logger::Env;
use notify_rust::Notification;
use sdk::presentation::sdk::Sharesies;
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct PortfolioRecord {
    id: Thing,
    value: f64,
    timestamp: String,
}

#[derive(Debug, Deserialize)]
struct CreatedRecord {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("println")).init();

    // Initialize SurrealDB client
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await.unwrap();

    let sdk = Sharesies::new();

    // Get the absolute path to the icon
    let icon = "resources/icon.png"; // Ensure this path is correct

    loop {
        if let Err(err) = sdk
            .authenticate(
                env::var("EMAIL").expect("EMAIL must be set"),
                env::var("PASSWORD").expect("PASSWORD must be set"),
            )
            .await
        {
            println!("Authentication failed: {}", err);
            sleep(Duration::from_secs(20)).await;
            continue;
        } else {
            println!("Authenticated successfully");
        }

        let mut previous_portfolio_value: Option<f64> = None;

        loop {
            println!("Checking portfolio value...");
            match sdk.get_portfolio().await {
                Ok(portfolio) => {
                    let current_value = portfolio.portfolio_value;
                    let formatted_value = format!("NZD {:.2}", current_value);
                    let timestamp = chrono::Utc::now().to_rfc3339();

                    // Save current portfolio value to SurrealDB
                    let record = PortfolioRecord {
                        id: Thing::from(("portfolio", timestamp.as_str())),
                        value: current_value,
                        timestamp: timestamp.clone(),
                    };

                    let created: Option<CreatedRecord> = db
                        .create(("portfolio", timestamp.as_str()))
                        .content(&record)
                        .await
                        .unwrap();

                    if let Some(_created_record) = created {
                        if let Some(prev_value) = previous_portfolio_value {
                            let difference = current_value - prev_value;
                            if difference > 0.0 {
                                Notification::new()
                                    .summary("Sharesies: Portfolio Increase")
                                    .body(&format!(
                                        "Portfolio value has increased to {} (up by NZD {:.2})",
                                        formatted_value, difference
                                    ))
                                    .icon(icon)
                                    .show()
                                    .unwrap();
                            } else if difference < 0.0 {
                                Notification::new()
                                    .summary("Sharesies: Portfolio Decrease")
                                    .body(&format!(
                                        "Portfolio value has decreased to {} (down by NZD {:.2})",
                                        formatted_value,
                                        difference.abs()
                                    ))
                                    .icon(icon)
                                    .show()
                                    .unwrap();
                            }
                            println!(
                                "Previous value: NZD {:.2}, Current value: NZD {:.2}, Difference: NZD {:.2}",
                                prev_value, current_value, difference
                            );
                        } else {
                            // Notify the first portfolio value
                            Notification::new()
                                .summary("Sharesies: Portfolio Value")
                                .body(&format!(
                                    "The initial portfolio value is {}",
                                    formatted_value
                                ))
                                .icon(icon)
                                .show()
                                .unwrap();
                            println!("Initial portfolio value: NZD {:.2}", current_value);
                        }

                        previous_portfolio_value = Some(current_value);
                    }
                }
                Err(err) => {
                    println!("Failed to retrieve portfolio: {}", err);
                    if err.to_string().contains("403 Forbidden") {
                        println!("Re-authenticating due to 403 Forbidden error...");
                        break; // Break the inner loop to re-authenticate
                    }
                }
            }

            // Sleep for 10 minutes before checking again
            sleep(Duration::from_secs(600)).await;
        }
    }
}
