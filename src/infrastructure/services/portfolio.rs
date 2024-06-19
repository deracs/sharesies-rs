use crate::domain::entities::api_endpoint::ApiEndpoint;
use crate::domain::entities::portfolio::CurrentPortfolio;
use crate::domain::errors::SharesiesError;
use crate::domain::repositories::portfolio_repository::PortfolioRepository;
use async_trait::async_trait;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::StatusCode;
use serde_json::from_str;

use super::api_service::ApiService;

#[async_trait]
impl PortfolioRepository for ApiService {
    async fn get_portfolio(&self) -> Result<CurrentPortfolio, SharesiesError> {
        if let Some(login_token) = self.token_storage.get_login() {
            if login_token.user.portfolio_id.is_empty() {
                return Err(SharesiesError::PortfolioRetrievalFailed(
                    "Portfolio ID not found".to_string(),
                ));
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
                        .map_err(|e| SharesiesError::HttpError(e.to_string()))?,
                );
            }
            let response = self.get(&url, Some(headers)).await?;

            info!("Response: {:?}", response);

            match response.status() {
                StatusCode::OK => {
                    let portfolio: CurrentPortfolio = from_str(
                        &response
                            .text()
                            .await
                            .map_err(|e| SharesiesError::HttpError(e.to_string()))?,
                    )
                    .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
                    Ok(portfolio)
                }
                StatusCode::NOT_FOUND => {
                    Err(SharesiesError::NotFound("Portfolio not found".to_string()))
                }
                StatusCode::FORBIDDEN => Err(SharesiesError::Forbidden(
                    "Access to portfolio is forbidden".to_string(),
                )),
                _ => Err(SharesiesError::HttpError(format!(
                    "Unexpected HTTP error: {}",
                    response.status()
                ))),
            }
        } else {
            Err(SharesiesError::PortfolioRetrievalFailed(
                "User not found".to_string(),
            ))
        }
    }
}
