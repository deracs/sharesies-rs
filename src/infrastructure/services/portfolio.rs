use crate::domain::entities::api_endpoint::ApiEndpoint;
use crate::domain::entities::portfolio::CurrentPortfolio;
use crate::domain::repositories::portfolio_repository::PortfolioRepository;
use async_trait::async_trait;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::from_str;

use super::api_service::ApiService;

#[async_trait]
impl PortfolioRepository for ApiService {
    async fn get_portfolio(&self) -> Result<CurrentPortfolio, String> {
        if let Some(login_token) = self.token_storage.get_login() {
            if login_token.user.portfolio_id.is_empty() {
                return Err("Portfolio ID not found".to_string());
            }

            let url = format!(
                "{}/{}",
                ApiEndpoint::Portfolio.url(),
                login_token.user.portfolio_id
            );
            info!("Calling: {}", url);
            let mut headers = HeaderMap::new();
            if let Some(token) = self.token_storage.get_token() {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.raikaia_identity_token))
                        .unwrap(),
                );
            }
            let response = self.get(&url, Some(headers)).await?;
            info!("Response: {}", response);
            let portfolio: CurrentPortfolio = from_str(&response).map_err(|e| e.to_string())?;
            Ok(portfolio)
        } else {
            return Err("User not found".to_string());
        }
    }
}
