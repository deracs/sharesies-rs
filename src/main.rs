use sdk::{
    application::use_cases::check_portfolio_notification::check_portfolio_notification,
    domain::repositories::storage_repository::StorageRepository,
    infrastructure::storage::db::{
        inmemory_db::InMemoryRepository, sqlite_db::SQLiteRepository,
        surreal_db::SurrealDBRepository,
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("println")).init();

    let db_type = std::env::var("DB_TYPE").unwrap_or_else(|_| "inmemory".to_string());

    let repo: Box<dyn StorageRepository> = match db_type.as_str() {
        "sqlite" => {
            println!("Using SQLite as the storage backend");
            Box::new(SQLiteRepository::new())
        }
        "surreal" => {
            println!("Using SurrealDB as the storage backend");
            Box::new(SurrealDBRepository::new().await)
        }
        _ => {
            println!("Using InMemory storage as the storage backend");
            Box::new(InMemoryRepository::new())
        }
    };

    check_portfolio_notification(repo).await;
}
