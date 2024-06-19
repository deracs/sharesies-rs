use async_trait::async_trait;

use crate::domain::entities::{
    login::LoginRequest, login::LoginResponse, raikaia_token::RakiaiaToken, token::Token,
};

#[async_trait]
pub trait AuthRepository {
    async fn login(&self, user: &LoginRequest) -> Result<LoginResponse, String>;
    async fn get_rakiaia_token(&self, acting_as_id: &str) -> Result<RakiaiaToken, String>;
    async fn authenticate(&self, user: &LoginRequest) -> Result<Token, String>;
}
