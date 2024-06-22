use crate::domain::errors::SharesiesError;
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
        if let Err(err) = self
            .sdk
            .authenticate(self.email.clone(), self.password.clone())
            .await
        {
            eprintln!("Authentication failed: {}", err);
            return;
        }

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

                    if self.save_portfolio_record(&record).await {
                        if let Some(prev_value) = previous_portfolio_value {
                            let difference = current_value - prev_value;
                            self.notify_portfolio_change(current_value, difference);
                        } else {
                            self.send_notification(
                                "Sharesies: Portfolio Value",
                                &format!("The initial portfolio value is {}", formatted_value),
                            );
                            println!("Initial portfolio value: NZD {:.2}", current_value);
                        }

                        previous_portfolio_value = Some(current_value);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to retrieve portfolio: {}", err);
                    if let SharesiesError::Forbidden(_) = err {
                        println!("Re-authenticating due to 403 Forbidden error...");
                        self.send_notification(
                            "Sharesies: Re-authentication",
                            "Re-authenticating...",
                        );
                        if self.reauthenticate().await {
                            println!("Re-authenticated successfully.");
                            self.send_notification(
                                "Sharesies: Re-authentication",
                                "Re-authenticated successfully.",
                            );
                        } else {
                            eprintln!("Re-authentication failed.");
                            self.send_notification(
                                "Sharesies: Re-authentication",
                                "Re-authentication failed.",
                            );
                        }
                    }
                }
            }

            // Sleep for 5 minutes before checking again
            sleep(Duration::from_secs(300)).await;
        }
    }

    async fn save_portfolio_record(&self, record: &PortfolioRecord) -> bool {
        let repo = self.repository.lock().await;
        match repo.save_portfolio(record).await {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Failed to save portfolio record: {}", err);
                false
            }
        }
    }

    async fn reauthenticate(&self) -> bool {
        match self
            .sdk
            .authenticate(self.email.clone(), self.password.clone())
            .await
        {
            Ok(_) => true,
            Err(err) => {
                eprintln!("Re-authentication failed: {}", err);
                false
            }
        }
    }

    fn notify_portfolio_change(&self, current_value: f64, difference: f64) {
        let formatted_value = format!("NZD {:.2}", current_value);
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
            current_value - difference,
            current_value,
            difference
        );
    }

    fn send_notification(&self, summary: &str, body: &str) {
        if let Err(err) = Notification::new().summary(summary).body(body).show() {
            eprintln!("Failed to send notification: {}", err);
        }
    }
}
