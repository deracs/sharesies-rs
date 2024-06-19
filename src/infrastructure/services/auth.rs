use crate::domain::entities::api_endpoint::ApiEndpoint;
use crate::domain::entities::login::{LoginRequest, LoginResponse};
use crate::domain::entities::raikaia_token::RakiaiaToken;
use crate::domain::entities::token::Token;
use crate::domain::repositories::auth_repository::AuthRepository;
use async_trait::async_trait;
use serde_json::from_str;

use super::api_service::ApiService;

#[async_trait]
impl AuthRepository for ApiService {
    async fn login(&self, user: &LoginRequest) -> Result<LoginResponse, String> {
        let url = ApiEndpoint::IdentityLogin.url();
        let response = self.post(url, user).await?;
        let login: LoginResponse = from_str(&response).map_err(|e| e.to_string())?;
        Ok(login)
    }

    async fn get_rakiaia_token(&self, acting_as_id: &str) -> Result<RakiaiaToken, String> {
        let url = format!(
            "{}?acting_as_id={}",
            ApiEndpoint::IdentityRakaiaToken.url(),
            acting_as_id
        );

        let response = self.get(&url).await?;
        let rakiaia_token: RakiaiaToken = from_str(&response).map_err(|e| e.to_string())?;
        Ok(rakiaia_token)
    }

    async fn authenticate(&self, user: &LoginRequest) -> Result<Token, String> {
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
