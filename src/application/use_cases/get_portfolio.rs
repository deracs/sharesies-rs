use crate::domain::entities::portfolio::CurrentPortfolio;
use crate::domain::repositories::portfolio_repository::PortfolioRepository;

pub struct GetPortfolioUseCase<R: PortfolioRepository> {
    pub repository: R,
}

impl<R: PortfolioRepository> GetPortfolioUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, portfolio_id: &str) -> Result<CurrentPortfolio, String> {
        self.repository.get_portfolio(portfolio_id).await
    }
}
