use crate::infrastructure::storage::token_storage::TokenStorage;
use reqwest::cookie::Jar;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ApiService {
    client: Client,
    cookie_store: Arc<Jar>,
    pub token_storage: TokenStorage,
}

impl ApiService {
    pub fn new(token_storage: TokenStorage) -> Self {
        let cookie_store = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .build()
            .unwrap();

        Self {
            client,
            cookie_store,
            token_storage,
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, String> {
        let request = self.client.get(url);

        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(text)
        } else {
            Err(format!("Failed to call API: {}", status))
        }
    }

    pub async fn post<T: Serialize>(&self, url: &str, body: &T) -> Result<String, String> {
        let request = self.client.post(url).json(body);

        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(text)
        } else {
            Err(format!("Failed to call API: {}", status))
        }
    }
}

impl Default for ApiService {
    fn default() -> Self {
        Self::new(TokenStorage::new())
    }
}
