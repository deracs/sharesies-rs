use crate::domain::entities::login::LoginRequest;
use crate::domain::errors::SharesiesError;
use crate::domain::repositories::auth_repository::AuthRepository;
use crate::infrastructure::storage::token_storage::TokenStorage;

pub struct LoginUseCase<R: AuthRepository> {
    repository: R,
    token_storage: TokenStorage,
}

impl<R: AuthRepository> LoginUseCase<R> {
    pub fn new(repository: R, token_storage: TokenStorage) -> Self {
        Self {
            repository,
            token_storage,
        }
    }

    pub async fn execute(&self, user: &LoginRequest) -> Result<(), SharesiesError> {
        let login = self.repository.login(user).await?;
        self.token_storage.store_login(login);
        Ok(())
    }
}
