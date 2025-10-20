use chrono::prelude::{DateTime, Utc}; 

#[derive(Debug, sqlx::FromRow)]
pub struct Trade {
    pub trade_id: i32, 
    pub instrument_id: i32, 
    pub trade_type: String,  // Buy or Sell
    pub time: DateTime<Utc>, 
    pub price: f32, 
    pub quantity: f32, 
    pub commission: f32 
}

#[derive(Debug, sqlx::FromRow)]
pub struct RealizedPnl {
    pub instrument_id: i32, 
    pub total_pnl: f32, 
    pub total_commission: f32, 
    pub net_pnl: f32,
    pub trade_count: i32
}

#[derive(Debug, sqlx::FromRow)]
pub struct PnlResult {
    pub total_pnl: f32,
    pub net_pnl: f32,
    pub total_commission: f32, 
}
