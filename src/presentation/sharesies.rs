use crate::application::use_cases::authenticate::AuthenticateUseCase;
use crate::application::use_cases::get_instruments::GetInstrumentsUseCase;
use crate::application::use_cases::get_portfolio::GetPortfolioUseCase;
use crate::application::use_cases::login::LoginUseCase;
use crate::infrastructure::services::api_service::ApiService;
use crate::infrastructure::storage::token_storage::TokenStorage;

pub struct Sharesies {
    pub login_use_case: LoginUseCase<ApiService>,
    pub authenticate_use_case: AuthenticateUseCase<ApiService>,
    pub get_portfolio_use_case: GetPortfolioUseCase<ApiService>,
    pub get_instruments_use_case: GetInstrumentsUseCase<ApiService>,
}

impl Sharesies {
    pub fn new() -> Self {
        let token_storage = TokenStorage::new();
        let api_service = ApiService::new(token_storage.clone());

        let login_use_case = LoginUseCase::new(api_service.clone(), token_storage.clone());
        let authenticate_use_case =
            AuthenticateUseCase::new(api_service.clone(), token_storage.clone());
        let get_portfolio_use_case = GetPortfolioUseCase::new(api_service.clone());
        let get_instruments_use_case = GetInstrumentsUseCase::new(api_service.clone());

        Self {
            login_use_case,
            authenticate_use_case,
            get_portfolio_use_case,
            get_instruments_use_case,
        }
    }
}

impl Default for Sharesies {
    fn default() -> Self {
        Self::new()
    }
}
