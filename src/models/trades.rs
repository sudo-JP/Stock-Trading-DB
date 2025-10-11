use chrono::prelude::{DateTime, Utc}; 

pub struct Trade {
    trade_id: i32, 
    instrument_id: i32, 
    trade_type: String,  // Buy or Sell
    time: DateTime<Utc>, 
    price: f32, 
    quantity: f32, 
    commission: f32 
}

pub struct RealizedPnl {
    instrument_id: i32, 
    total_pnl: f32, 
    total_commission: f32, 
    net_pnl: f32,
    trade_count: i32
}

pub struct PnlResult {
    total_pnl: f32,
    net_pnl: f32,
    total_commission: f32, 
}
