use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentPortfolio {
    pub uuid: String,
    pub currency: String,
    pub date: String,
    pub portfolio_value: f64,
    pub total_return: f64,
    pub simple_return: f64,
    pub cost_basis: f64,
    pub cost_basis_max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioRecord {
    pub id: String,
    pub value: f64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatedRecord {
    pub id: Thing,
}
