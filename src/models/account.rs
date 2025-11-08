use crate::models::prelude_model::*;

#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    pub id: String,
    pub currency: String,

    // Owned numeric
    pub cash: f64,
    pub buying_power: f64,
    pub equity: f64,
    pub portfolio_value: f64,

    // Futures 
    pub effective_buying_power: f64,
    pub daytrading_buying_power: f64,
    pub regt_buying_power: f64,
    pub non_marginable_buying_power: f64,
    pub last_equity: f64,
    pub sma: f64,
    pub position_market_value: f64,
    pub long_market_value: f64,
    pub short_market_value: f64,


    // Metadata 
    pub status: String,
    pub crypto_status: String,
    pub balance_asof: DateTime<Utc>, 
    pub daytrade_count: f64,
}
