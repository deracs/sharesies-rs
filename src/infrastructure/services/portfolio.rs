use crate::domain::entities::api_endpoint::ApiEndpoint;
use crate::domain::entities::portfolio::CurrentPortfolio;
use crate::domain::repositories::portfolio_repository::PortfolioRepository;
use async_trait::async_trait;
use serde_json::from_str;

use super::api_service::ApiService;

#[async_trait]
impl PortfolioRepository for ApiService {
    async fn get_portfolio(&self, portfolio_id: &str) -> Result<CurrentPortfolio, String> {
        let url = format!("{}/{}", ApiEndpoint::Portfolio.url(), portfolio_id);

        let response = self.get(&url).await?;
        let portfolio: CurrentPortfolio = from_str(&response).map_err(|e| e.to_string())?;
        Ok(portfolio)
    }
}
