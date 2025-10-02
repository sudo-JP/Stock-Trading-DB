use chrono::prelude::{DateTime, Utc}; 

pub struct TechnicalIndicators {
    tech_ind_id: i32, 
    instrument_id: i32, 
    time: DateTime<Utc>, 
    sma_20: f32, 
    ema_12: f32, 

    rsi_14: f32, 
    macd: f32, 

    bollinger_upper: f32, 
    bollinger_lower: f32,
    atr_14: f32 
}
