use crate::application::use_cases::authenticate::AuthenticateUseCase;
use crate::application::use_cases::get_portfolio::GetPortfolioUseCase;
use crate::application::use_cases::login::LoginUseCase;
use crate::domain::entities::login::LoginRequest;
use crate::domain::entities::portfolio::CurrentPortfolio;
use crate::domain::entities::token::Token;
use crate::infrastructure::services::api_service::ApiService;
use crate::infrastructure::storage::token_storage::TokenStorage;

pub struct SDK {
    login_use_case: LoginUseCase<ApiService>,
    authenticate_use_case: AuthenticateUseCase<ApiService>,
    get_portfolio_use_case: GetPortfolioUseCase<ApiService>,
}

impl SDK {
    pub fn new() -> Self {
        let token_storage = TokenStorage::new();
        let api_service = ApiService::new(token_storage.clone());

        let login_use_case = LoginUseCase::new(api_service.clone(), token_storage.clone());
        let authenticate_use_case =
            AuthenticateUseCase::new(api_service.clone(), token_storage.clone());
        let get_portfolio_use_case = GetPortfolioUseCase::new(api_service.clone());

        Self {
            login_use_case,
            authenticate_use_case,
            get_portfolio_use_case,
        }
    }

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

    pub async fn get_portfolio(&self) -> Result<CurrentPortfolio, String> {
        self.get_portfolio_use_case.execute().await
    }
}

impl Default for SDK {
    fn default() -> Self {
        Self::new()
    }
}
