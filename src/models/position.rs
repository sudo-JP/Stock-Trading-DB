use crate::models::prelude_model::*; 

#[derive(Debug, sqlx::FromRow)]
pub struct Position {
    pub instrument_id: String,
    pub symbol: String,
    pub exchange: String,
    pub instrument_class: String,
    pub side: String,

    pub qty: f64,
    pub qty_available: f64,
    pub avg_entry_price: f64,
    pub market_value: f64,
    pub cost_basis: f64,

    pub unrealized_pl: f64,
    pub unrealized_plpc: f64,
    pub unrealized_intraday_pl: f64,
    pub unrealized_intraday_plpc: f64,

    pub current_price: f64,
    pub lastday_price: f64,
    pub change_today: f64,

    pub instrument_marginable: bool,
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PositionUpdate {
    pub instrument_id: i32, 
    pub quantity_change: f32, 
    pub trade_price: f32, 
    pub commission: f32
}
