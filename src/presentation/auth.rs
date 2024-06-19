use crate::domain::entities::login::LoginRequest;
use crate::domain::entities::token::Token;

use super::sharesies::Sharesies;

impl Sharesies {
    pub async fn login(&self, email: String, password: String) -> Result<(), String> {
        let user = LoginRequest {
            email,
            password,
            remember: true,
            mfa_token: None,
        };
        self.login_use_case.execute(&user).await
    }

    pub async fn authenticate(&self, email: String, password: String) -> Result<Token, String> {
        let user = LoginRequest {
            email,
            password,
            remember: true,
            mfa_token: None,
        };

        let token = self.authenticate_use_case.execute(&user).await?;

        Ok(token)
    }
}
