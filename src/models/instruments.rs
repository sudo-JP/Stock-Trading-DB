#[derive(Debug, sqlx::FromRow)]
pub struct Instrument {
    instrument_id: i32, 
    symbol: String, 
    name: String, 
    instr_type: String, 
    currency: String, 
    exchange: String, 
    multiplier: f32, 
    min_tick: Option<f32>
}
