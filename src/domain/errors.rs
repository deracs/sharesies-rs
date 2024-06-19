use thiserror::Error;

#[derive(Error, Debug)]
pub enum SharesiesError {
    #[error("authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("failed to retrieve portfolio: {0}")]
    PortfolioRetrievalFailed(String),

    #[error("failed to retrieve instruments: {0}")]
    InstrumentRetrievalFailed(String),

    #[error("login failed: {0}")]
    LoginFailed(String),

    #[error("failed to retrieve Rakiaia token: {0}")]
    RakiaiaTokenRetrievalFailed(String),

    #[error("repository error: {0}")]
    RepositoryError(String),

    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("unknown error: {0}")]
    Unknown(String),
}
