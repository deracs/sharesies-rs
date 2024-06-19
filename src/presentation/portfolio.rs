use crate::domain::entities::portfolio::CurrentPortfolio;

use super::sharesies::Sharesies;

impl Sharesies {
    pub async fn get_portfolio(&self) -> Result<CurrentPortfolio, String> {
        self.get_portfolio_use_case.execute().await
    }
}
