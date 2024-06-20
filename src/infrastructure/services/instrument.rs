use crate::domain::{
    entities::{
        api_endpoint::ApiEndpoint,
        instrument::{InstrumentRequest, InstrumentResponse},
    },
    errors::SharesiesError,
    repositories::instrument_repository::InstrumentRepository,
};
use async_trait::async_trait;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::from_str;

use super::api_service::ApiService;

#[async_trait]
impl InstrumentRepository for ApiService {
    async fn get_instruments(
        &self,
        data: InstrumentRequest,
    ) -> Result<InstrumentResponse, SharesiesError> {
        if let Some(login_token) = self.token_storage.get_login() {
            if login_token.distill_token.is_empty() {
                return Err(SharesiesError::InstrumentRetrievalFailed(
                    "Token ID not found".to_string(),
                ));
            }

            let url = ApiEndpoint::Instruments.url();
            info!("Calling: {}", url);

            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", login_token.distill_token))
                    .map_err(|e| SharesiesError::HttpError(e.to_string()))?,
            );

            let response = self
                .post(url, &data, Some(headers))
                .await
                .map_err(|e| SharesiesError::HttpError(e.to_string()))?;

            info!("Response: {:?}", response);

            let response_text = response
                .text()
                .await
                .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
            let instrument_response: InstrumentResponse =
                from_str(&response_text).map_err(|e| SharesiesError::HttpError(e.to_string()))?;
            Ok(instrument_response)
        } else {
            Err(SharesiesError::InstrumentRetrievalFailed(
                "User not found".to_string(),
            ))
        }
    }
}
