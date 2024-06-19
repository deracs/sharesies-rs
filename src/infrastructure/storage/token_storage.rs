use std::sync::{Arc, Mutex};

use crate::domain::entities::{login::LoginResponse, token::Token};

#[derive(Debug, Clone)]
pub struct TokenStorage {
    token: Arc<Mutex<Option<Token>>>,
    login: Arc<Mutex<Option<LoginResponse>>>,
}

impl TokenStorage {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
            login: Arc::new(Mutex::new(None)),
        }
    }

    pub fn store_token(&self, token: Token) {
        let mut guard = self.token.lock().unwrap();
        *guard = Some(token);
    }

    pub fn store_login(&self, login: LoginResponse) {
        let mut guard = self.login.lock().unwrap();
        *guard = Some(login);
    }

    pub fn get_token(&self) -> Option<Token> {
        let guard = self.token.lock().unwrap();
        guard.clone()
    }

    pub fn get_login(&self) -> Option<LoginResponse> {
        let guard = self.login.lock().unwrap();
        guard.clone()
    }
}

impl Default for TokenStorage {
    fn default() -> Self {
        Self::new()
    }
}
