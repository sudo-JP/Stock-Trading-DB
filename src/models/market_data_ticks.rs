use chrono::prelude::{DateTime, Utc}; 

pub struct MarketDataTick {
    markt_data_tick_id: i32,
    time: DateTime<Utc>, 
    instrument_id: i32, 
    bid_price: f32, 
    bid_size: i32, 
    ask_price: f32, 
    last_price: f32, 
    volume: i32
}
