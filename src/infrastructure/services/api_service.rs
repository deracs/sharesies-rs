use crate::infrastructure::storage::token_storage::TokenStorage;
use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ApiService {
    client: Client,
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
            token_storage,
        }
    }

    pub fn with_token_storage(token_storage: TokenStorage) -> Self {
        Self::new(token_storage)
    }

    pub async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, String> {
        let request = self.client.get(url);

        let request = match headers {
            Some(headers) => {
                let mut request = request;
                for (key, value) in headers.iter() {
                    request = request.header(key, value);
                }
                request
            }
            None => request,
        };

        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let text = response.text().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            Ok(text)
        } else {
            Err(format!("Failed to call API: {}", status))
        }
    }

    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        body: &T,
        headers: Option<HeaderMap>,
    ) -> Result<String, String> {
        let request = self.client.post(url).json(body);
        let request = match headers {
            Some(headers) => {
                let mut request = request;
                for (key, value) in headers.iter() {
                    request = request.header(key, value);
                }
                request
            }
            None => request,
        };

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
