use crate::domain::entities::portfolio::PortfolioRecord;
use crate::domain::repositories::storage_repository::StorageRepository;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

pub struct InMemoryRepository {
    data: Arc<AsyncMutex<HashMap<String, PortfolioRecord>>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageRepository for InMemoryRepository {
    async fn save_portfolio(&self, record: &PortfolioRecord) -> Result<(), String> {
        let mut data = self.data.lock().await;
        data.insert(record.timestamp.clone(), record.clone());
        Ok(())
    }
}
