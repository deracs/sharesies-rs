use crate::domain::entities::portfolio::PortfolioRecord;
use async_trait::async_trait;

#[async_trait]
pub trait StorageRepository {
    async fn save_portfolio(&self, record: &PortfolioRecord) -> Result<(), String>;
}
