use std::sync::{Arc, Mutex};

use log::error;

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
        if let Ok(mut guard) = self.token.lock() {
            *guard = Some(token);
        } else {
            // handle the error, e.g., log it or return an error
            error!("Failed to lock token mutex");
        }
    }

    pub fn store_login(&self, login: LoginResponse) {
        if let Ok(mut guard) = self.login.lock() {
            *guard = Some(login);
        } else {
            // handle the error, e.g., log it or return an error
            error!("Failed to lock login mutex");
        }
    }

    pub fn get_token(&self) -> Option<Token> {
        match self.token.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => {
                // handle the error, e.g., log it or return None
                error!("Failed to lock token mutex");
                None
            }
        }
    }

    pub fn get_login(&self) -> Option<LoginResponse> {
        match self.login.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => {
                // handle the error, e.g., log it or return None
                error!("Failed to lock login mutex");
                None
            }
        }
    }
}

impl Default for TokenStorage {
    fn default() -> Self {
        Self::new()
    }
}
