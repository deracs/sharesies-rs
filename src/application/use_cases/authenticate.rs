use crate::domain::entities::login::LoginRequest;
use crate::domain::entities::token::Token;
use crate::domain::errors::SharesiesError;
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::infrastructure::storage::token_storage::TokenStorage;

pub struct AuthenticateUseCase<R: AuthRepository> {
    repository: R,
    token_storage: TokenStorage,
}

impl<R: AuthRepository> AuthenticateUseCase<R> {
    pub fn new(repository: R, token_storage: TokenStorage) -> Self {
        Self {
            repository,
            token_storage,
        }
    }

    pub async fn execute(&self, user: &LoginRequest) -> Result<Token, SharesiesError> {
        let login = self.repository.login(user).await?;
        self.token_storage.store_login(login.clone());

        let rakiaia_token = self.repository.get_rakiaia_token(&login.user.id).await?;

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
