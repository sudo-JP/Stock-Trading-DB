use crate::models::prelude_model::*;

#[derive(Debug, sqlx::FromRow)]
pub struct MarketDataTick {
    pub markt_data_tick_id: i32,
    pub time: DateTime<Utc>, 
    pub instrument_id: i32, 
    pub bid_price: f32, 
    pub bid_size: i32, 
    pub ask_price: f32, 
    pub last_price: f32, 
    pub volume: i32
}
