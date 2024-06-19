use async_trait::async_trait;

use crate::domain::{
    entities::{
        login::{LoginRequest, LoginResponse},
        raikaia_token::RakiaiaToken,
        token::Token,
    },
    errors::SharesiesError,
};

#[async_trait]
pub trait AuthRepository {
    async fn login(&self, user: &LoginRequest) -> Result<LoginResponse, SharesiesError>;
    async fn get_rakiaia_token(&self, acting_as_id: &str) -> Result<RakiaiaToken, SharesiesError>;
    async fn authenticate(&self, user: &LoginRequest) -> Result<Token, SharesiesError>;
}
