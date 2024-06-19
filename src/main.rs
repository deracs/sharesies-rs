use sdk::{
    application::use_cases::check_portfolio_notification::CheckPortfolioNotificationUseCase,
    domain::repositories::storage_repository::StorageRepository,
    infrastructure::storage::db::{
        inmemory_db::InMemoryRepository, sqlite_db::SQLiteRepository,
        surreal_db::SurrealDBRepository,
    },
    presentation::sharesies::Sharesies,
};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let db_type = std::env::var("DB_TYPE").unwrap_or_else(|_| "inmemory".to_string());

    let repo: Arc<Mutex<dyn StorageRepository>> = match db_type.as_str() {
        "sqlite" => {
            println!("Using SQLite as the storage backend");
            Arc::new(Mutex::new(SQLiteRepository::new()))
        }
        "surreal" => {
            println!("Using SurrealDB as the storage backend");
            Arc::new(Mutex::new(SurrealDBRepository::new().await))
        }
        _ => {
            println!("Using InMemory storage as the storage backend");
            Arc::new(Mutex::new(InMemoryRepository::new()))
        }
    };

    let sdk = Sharesies::new();
    let check_portfolio_use_case = CheckPortfolioNotificationUseCase::new(
        Arc::clone(&repo),
        sdk,
        env::var("EMAIL").expect("EMAIL must be set"),
        env::var("PASSWORD").expect("PASSWORD must be set"),
    );

    // Execute the use case
    check_portfolio_use_case.execute().await;
}
