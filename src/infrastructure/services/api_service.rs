use crate::domain::errors::SharesiesError;
use crate::infrastructure::storage::token_storage::TokenStorage;
use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ApiService {
    pub client: Client,
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

    pub async fn get(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
    ) -> Result<reqwest::Response, SharesiesError> {
        let mut request = self.client.get(url);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = request
            .send()
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
        Ok(response)
    }

    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        body: &T,
        headers: Option<HeaderMap>,
    ) -> Result<reqwest::Response, SharesiesError> {
        let mut request = self.client.post(url).json(body);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = request
            .send()
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
        Ok(response)
    }
}

impl Default for ApiService {
    fn default() -> Self {
        Self::new(TokenStorage::new())
    }
}
