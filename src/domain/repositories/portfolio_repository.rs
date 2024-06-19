use crate::domain::entities::portfolio::CurrentPortfolio;
use async_trait::async_trait;

#[async_trait]
pub trait PortfolioRepository {
    async fn get_portfolio(&self) -> Result<CurrentPortfolio, String>;
}
