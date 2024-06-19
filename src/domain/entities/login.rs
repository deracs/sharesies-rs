use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember: bool,
    pub mfa_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub distill_token: String,
    pub distill_token_v2: String,
    pub rakaia_token: String,
    pub user: User,
}
