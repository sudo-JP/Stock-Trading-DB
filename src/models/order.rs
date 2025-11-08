use crate::models::prelude_model::*; 

#[derive(Debug, sqlx::FromRow)]
pub struct Order {
    pub order_id: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: DateTime<Utc>,
    pub filled_at: DateTime<Utc>,
    pub status: String, 

    pub instrument_id: String,

    pub symbol: String,
    pub side: String,
    pub type_order: String,
    pub time_in_force: String,

    pub qty: f64,
    pub filled_qty: f64,
    pub filled_avg_price: f64,

    pub instrument_class: String,
    pub position_intent: String,
    pub notional: f64,
    pub limit_price: f64, 
    pub stop_price: f64,

    pub extended_hours: bool,
}
