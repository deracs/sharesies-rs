use crate::{
    domain::entities::portfolio::PortfolioRecord,
    domain::repositories::storage_repository::StorageRepository,
    presentation::sharesies::Sharesies,
};
use chrono::Utc;
use notify_rust::Notification;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct CheckPortfolioNotificationUseCase {
    pub repository: Arc<Mutex<dyn StorageRepository>>,
    pub sdk: Sharesies,
    pub email: String,
    pub password: String,
}

impl CheckPortfolioNotificationUseCase {
    pub fn new(
        repository: Arc<Mutex<dyn StorageRepository>>,
        sdk: Sharesies,
        email: String,
        password: String,
    ) -> Self {
        Self {
            repository,
            sdk,
            email,
            password,
        }
    }

    pub async fn execute(&self) {
        match self
            .sdk
            .authenticate(self.email.clone(), self.password.clone())
            .await
        {
            Ok(_token) => {
                let mut previous_portfolio_value: Option<f64> = None;
                loop {
                    println!("Checking portfolio value...");

                    match self.sdk.get_portfolio().await {
                        Ok(portfolio) => {
                            let current_value = portfolio.portfolio_value;
                            let formatted_value = format!("NZD {:.2}", current_value);
                            let timestamp = Utc::now().to_rfc3339();

                            // Save current portfolio value
                            let record = PortfolioRecord {
                                id: timestamp.clone(),
                                value: current_value,
                                timestamp: timestamp.clone(),
                            };

                            let save_result = {
                                let repo = self.repository.lock().await;
                                repo.save_portfolio(&record).await
                            };

                            if save_result.is_ok() {
                                if let Some(prev_value) = previous_portfolio_value {
                                    let difference = current_value - prev_value;
                                    if difference > 0.0 {
                                        self.send_notification(
                                            "Sharesies: Portfolio Increase",
                                            &format!(
                                                "Portfolio value has increased to {} (up by NZD {:.2})",
                                                formatted_value, difference
                                            ),
                                        );
                                    } else if difference < 0.0 {
                                        self.send_notification(
                                            "Sharesies: Portfolio Decrease",
                                            &format!(
                                                "Portfolio value has decreased to {} (down by NZD {:.2})",
                                                formatted_value,
                                                difference.abs()
                                            ),
                                        );
                                    }
                                    println!(
                                        "Previous value: NZD {:.2}, Current value: NZD {:.2}, Difference: NZD {:.2}",
                                        prev_value, current_value, difference
                                    );
                                } else {
                                    // Notify the first portfolio value
                                    self.send_notification(
                                        "Sharesies: Portfolio Value",
                                        &format!(
                                            "The initial portfolio value is {}",
                                            formatted_value
                                        ),
                                    );
                                    println!("Initial portfolio value: NZD {:.2}", current_value);
                                }

                                previous_portfolio_value = Some(current_value);
                            }
                        }
                        Err(err) => {
                            println!("Failed to retrieve portfolio: {}", err);
                            if err.to_string().contains("403 Forbidden") {
                                println!("Re-authenticating due to 403 Forbidden error...");
                                match self
                                    .sdk
                                    .authenticate(self.email.clone(), self.password.clone())
                                    .await
                                {
                                    Ok(_) => {
                                        println!("Re-authenticated successfully.");
                                    }
                                    Err(auth_err) => {
                                        println!("Re-authentication failed: {}", auth_err);
                                    }
                                }
                            }
                        }
                    }

                    // Sleep for 10 minutes before checking again
                    sleep(Duration::from_secs(600)).await;
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        };
    }

    fn send_notification(&self, summary: &str, body: &str) {
        Notification::new()
            .summary(summary)
            .body(body)
            .show()
            .unwrap();
    }
}
