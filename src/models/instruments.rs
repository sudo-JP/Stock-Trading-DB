#[derive(Debug, sqlx::FromRow)]
pub struct Instrument {
    pub instrument_id: i32, 
    pub symbol: String, 
    pub name: String, 
    pub instr_type: String, 
    pub currency: String, 
    pub exchange: String, 
    pub multiplier: f32, 
    pub min_tick: Option<f32>
}
