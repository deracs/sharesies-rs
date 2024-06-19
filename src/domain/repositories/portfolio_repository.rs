use crate::domain::entities::portfolio::CurrentPortfolio;
use async_trait::async_trait;

#[async_trait]
pub trait PortfolioRepository {
    async fn get_portfolio(&self, portfolio_id: &str) -> Result<CurrentPortfolio, String>;
}
