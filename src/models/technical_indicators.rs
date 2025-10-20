use crate::models::prelude_model::*;

#[derive(Debug, sqlx::FromRow)]
pub struct TechnicalIndicators {
    pub tech_ind_id: i32, 
    pub instrument_id: i32, 
    pub time: DateTime<Utc>, 
    pub sma_20: f32, 
    pub ema_12: f32, 
 
    pub rsi_14: f32, 
    pub macd: f32, 
 
    pub bollinger_upper: f32, 
    pub bollinger_lower: f32,
    pub atr_14: f32 
}
