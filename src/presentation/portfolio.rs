use crate::domain::{entities::portfolio::CurrentPortfolio, errors::SharesiesError};

use super::sharesies::Sharesies;

impl Sharesies {
    pub async fn get_portfolio(&self) -> Result<CurrentPortfolio, SharesiesError> {
        self.get_portfolio_use_case.execute().await
    }
}
