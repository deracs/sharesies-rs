use crate::domain::entities::api_endpoint::ApiEndpoint;
use crate::domain::entities::login::{LoginRequest, LoginResponse};
use crate::domain::entities::raikaia_token::RakiaiaToken;
use crate::domain::entities::token::Token;
use crate::domain::errors::SharesiesError;
use crate::domain::repositories::auth_repository::AuthRepository;
use async_trait::async_trait;
use log::info;
use serde_json::from_str;

use super::api_service::ApiService;

#[async_trait]
impl AuthRepository for ApiService {
    async fn login(&self, user: &LoginRequest) -> Result<LoginResponse, SharesiesError> {
        let url = ApiEndpoint::IdentityLogin.url();
        let response = self
            .post(url, user, None)
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
        info!("Response: {:?}", response);

        let response_text = response
            .text()
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
        let login: LoginResponse =
            from_str(&response_text).map_err(|e| SharesiesError::LoginFailed(e.to_string()))?;
        Ok(login)
    }

    async fn get_rakiaia_token(&self, acting_as_id: &str) -> Result<RakiaiaToken, SharesiesError> {
        let url = format!(
            "{}?acting_as_id={}",
            ApiEndpoint::IdentityRakaiaToken.url(),
            acting_as_id
        );

        let response = self
            .get(&url, None)
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
        info!("Response: {:?}", response);

        let response_text = response
            .text()
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))?;
        let rakiaia_token: RakiaiaToken = from_str(&response_text)
            .map_err(|e| SharesiesError::RakiaiaTokenRetrievalFailed(e.to_string()))?;
        Ok(rakiaia_token)
    }

    async fn authenticate(&self, user: &LoginRequest) -> Result<Token, SharesiesError> {
        let login = self.login(user).await?;
        let rakiaia_token = self.get_rakiaia_token(&login.user.id).await?;

        let token = Token {
            distill_token: login.distill_token,
            distill_token_v2: login.distill_token_v2,
            rakaia_token: login.rakaia_token,
            raikaia_identity_token: rakiaia_token.token,
        };

        self.token_storage.store_token(token.clone());

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::storage::token_storage::TokenStorage;
    use mockito::Server;
    use std::fs::read_to_string;

    #[tokio::test]
    async fn test_login() {
        let mut server = Server::new_async().await;
        let login_response_body = read_to_string("tests/data/login_response.json")
            .expect("Failed to read login_response.json");
        let _m = server
            .mock("POST", "/identity/login")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&login_response_body)
            .create_async()
            .await;

        let token_storage = TokenStorage::new();
        let api_service = ApiService::new(token_storage);

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            remember: false,
            mfa_token: None,
        };

        // Use the mock server URL
        let url = &format!("{}/identity/login", &server.url());
        let result = api_service
            .post(url, &login_request, None)
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()));

        assert!(result.is_ok());

        let response = result.unwrap();
        let response_text = response
            .text()
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))
            .unwrap();
        let login_response: LoginResponse = from_str(&response_text)
            .map_err(|e| SharesiesError::LoginFailed(e.to_string()))
            .unwrap();

        assert_eq!(login_response.user.id, "1");
        assert_eq!(login_response.distill_token, "token123");
    }

    #[tokio::test]
    async fn test_get_rakiaia_token() {
        let mut server = Server::new_async().await;

        // Read the JSON response from a file
        let rakiaia_token_response_body = read_to_string("tests/data/rakiaia_token_response.json")
            .expect("Failed to read tests/data/rakiaia_token_response.json");

        let _m = server
            .mock("GET", "/identity/rakiaia_token?acting_as_id=1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&rakiaia_token_response_body)
            .create_async()
            .await;

        let token_storage = TokenStorage::new();
        let api_service = ApiService::new(token_storage);
        // Use the mock server URL
        let url = &format!("{}/identity/rakiaia_token?acting_as_id=1", &server.url());
        let result = api_service
            .get(url, None)
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()));

        assert!(result.is_ok());

        let response = result.unwrap();
        let response_text = response
            .text()
            .await
            .map_err(|e| SharesiesError::HttpError(e.to_string()))
            .unwrap();
        let rakiaia_token: RakiaiaToken = from_str(&response_text)
            .map_err(|e| SharesiesError::RakiaiaTokenRetrievalFailed(e.to_string()))
            .unwrap();

        assert_eq!(rakiaia_token.token, "rakiaia123");
    }
}
