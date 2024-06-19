pub enum ApiEndpoint {
    IdentityLogin,
    IdentityCheck,
    IdentityDistillToken,
    IdentityRakaiaToken,
    IdentityReauth,
    Instruments,
    CostBuy,
    CreateBuy,
    CostSell,
    CreateSell,
    Portfolio,
    InstrumentsUrlslug,
    FxRate,
    OrderHistory,
    RecentOrders,
}

impl ApiEndpoint {
    pub fn url(&self) -> &'static str {
        match self {
            ApiEndpoint::IdentityLogin => "https://app.sharesies.nz/api/identity/login",
            ApiEndpoint::IdentityCheck => "https://app.sharesies.nz/api/identity/check",
            ApiEndpoint::IdentityDistillToken => {
                "https://app.sharesies.nz/api/identity/distill-token"
            }
            ApiEndpoint::IdentityRakaiaToken => {
                "https://app.sharesies.nz/api/identity/rakaia-token"
            }
            ApiEndpoint::IdentityReauth => "https://app.sharesies.nz/api/identity/reauthenticate",
            ApiEndpoint::Instruments => "https://data.sharesies.nz/api/v1/instruments",
            ApiEndpoint::CostBuy => "https://app.sharesies.nz/api/order/cost-buy",
            ApiEndpoint::CreateBuy => "https://app.sharesies.nz/api/order/create-buy",
            ApiEndpoint::CostSell => "https://app.sharesies.nz/api/order/cost-sell",
            ApiEndpoint::CreateSell => "https://app.sharesies.nz/api/order/create-sell",
            ApiEndpoint::Portfolio => "https://portfolio.sharesies.nz/api/v1/portfolios",
            ApiEndpoint::InstrumentsUrlslug => {
                "https://data.sharesies.nz/api/v1/instruments/urlslug"
            }
            ApiEndpoint::FxRate => "https://app.sharesies.nz/api/fx/get-rate-v2",
            ApiEndpoint::OrderHistory => "https://app.sharesies.nz/api/accounting/order-history-v5",
            ApiEndpoint::RecentOrders => "https://app.sharesies.nz/api/accounting/recent-orders-v3",
        }
    }
}
