use crate::domain::entities::portfolio::PortfolioRecord;
use crate::domain::repositories::storage_repository::StorageRepository;
use async_trait::async_trait;
use rusqlite::{params, Connection, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SQLiteRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SQLiteRepository {
    pub fn new() -> Self {
        let conn = Connection::open("portfolio.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS portfolio (
                id TEXT PRIMARY KEY,
                value REAL NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl Default for SQLiteRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StorageRepository for SQLiteRepository {
    async fn save_portfolio(&self, record: &PortfolioRecord) -> Result<(), String> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO portfolio (id, value, timestamp) VALUES (?1, ?2, ?3)",
            params![record.id.to_string(), record.value, record.timestamp],
        )
        .map(|_| ())
        .map_err(|e| e.to_string())
    }
}
