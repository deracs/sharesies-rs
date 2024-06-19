use async_trait::async_trait;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

use crate::domain::entities::portfolio::PortfolioRecord;
use crate::domain::repositories::storage_repository::StorageRepository;

pub async fn get_surreal_db_client() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    db
}

pub struct SurrealDBRepository {
    db: Surreal<Client>,
}

impl SurrealDBRepository {
    pub async fn new() -> Self {
        Self {
            db: get_surreal_db_client().await,
        }
    }
}

#[async_trait]
impl StorageRepository for SurrealDBRepository {
    async fn save_portfolio(&self, record: &PortfolioRecord) -> Result<(), String> {
        self.db
            .create(("portfolio", record.timestamp.as_str()))
            .content(&record)
            .await
            .map(|_: Option<PortfolioRecord>| ())
            .map_err(|e| e.to_string())
    }
}
