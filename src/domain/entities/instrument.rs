use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InstrumentRequest {
    pub query: String,
    pub instruments: Vec<String>,
    pub trading_statuses: Vec<String>,
    pub sort: String,
    pub price_change_time: String,
    pub page: i32,
    pub per_page: i32,
}

impl InstrumentRequest {
    pub fn create(
        instruments: Vec<String>,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> InstrumentRequest {
        Self {
            query: "".to_string(),
            instruments,
            trading_statuses: vec![
                "active".to_string(),
                "halt".to_string(),
                "closeonly".to_string(),
                "notrade".to_string(),
            ],
            page: page.unwrap_or(1),
            per_page: per_page.unwrap_or(10),
            sort: "maintainIdOrder".to_string(),
            price_change_time: "1y".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentResponse {
    pub instruments: Vec<Instrument>,
    pub total: i32,
    #[serde(rename = "currentPage")]
    pub current_page: i32,
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: i32,
    #[serde(rename = "numberOfPages")]
    pub number_of_pages: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instrument {
    pub id: String,
    #[serde(rename = "annualisedReturnPercent")]
    pub annualised_return_percent: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "dominantColour")]
    pub dominant_colour: Option<String>,
    pub exchange: Option<String>,
    #[serde(rename = "exchangeCountry")]
    pub exchange_country: Option<String>,
    #[serde(rename = "grossDividendYieldPercent")]
    pub gross_dividend_yield_percent: Option<String>,
    #[serde(rename = "instrumentType")]
    pub instrument_type: Option<String>,
    #[serde(rename = "isAdr")]
    pub is_adr: Option<bool>,
    #[serde(rename = "isIlliquid")]
    pub is_illiquid: Option<bool>,
    #[serde(rename = "isUsPartnership")]
    pub is_us_partnership: Option<bool>,
    #[serde(rename = "isVolatile")]
    pub is_volatile: Option<bool>,
    #[serde(rename = "marketCap")]
    pub market_cap: Option<u64>,
    #[serde(rename = "marketLastCheck")]
    pub market_last_check: Option<String>,
    #[serde(rename = "marketPrice")]
    pub market_price: String,
    pub name: String,
    #[serde(rename = "peRatio")]
    pub pe_ratio: Option<String>,
    #[serde(rename = "riskRating")]
    pub risk_rating: Option<u32>,
    pub symbol: String,
    #[serde(rename = "tradingStatus")]
    pub trading_status: Option<String>,
    #[serde(rename = "urlSlug")]
    pub url_slug: Option<String>,
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,
    pub day_prices: Option<HashMap<String, f64>>,
    pub logos: Option<InstrumentLogo>,
    pub ceo: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstrumentLogo {
    pub wide: String,
    pub thumb: String,
    pub micro: String,
}
