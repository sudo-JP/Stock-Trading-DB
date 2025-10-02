use chrono::prelude::{DateTime, Utc}; 

pub struct Trades {
    trade_id: i32, 
    instrument_id: i32, 
    trade_type: String, 
    time: DateTime<Utc>, 
    price: f32, 
    quantity: f32, 
    commission: f32 
}
