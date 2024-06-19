use serde::{Deserialize, Serialize};

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
