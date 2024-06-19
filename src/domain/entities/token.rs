use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub distill_token: String,
    pub distill_token_v2: String,
    pub rakaia_token: String,
    pub raikaia_identity_token: String,
}
