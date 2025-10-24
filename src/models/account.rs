use crate::models::prelude_model::*;

#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    pub account_id: String, 
    pub currency: String, 
    pub buying_power: f64, 
    pub cash: f64, 
    pub portfolio_value: f64, 
    pub equity: f64, 

    pub unrealized_pl: f64, 
    pub realized_pl: f64, 

    pub status: String,
    pub last_update: DateTime<Utc>, 
}
