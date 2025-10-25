#[derive(Debug, sqlx::FromRow)]
pub struct Instrument {
    pub instrument_id: String, 
    pub instrument_class: String, 
    pub symbol: String, 
    pub name: String, 
    pub status: String, 
    
    pub tradeable: bool,
    pub marginable: bool,
    pub shortable: bool, 
    pub easy_to_borrow: bool, 
    pub fractionable: bool,

    // Addition 
    pub instr_type: String, 
    pub currency: String, 
    pub exchange: String, 
    pub multiplier: f32, 
    pub min_tick: Option<f32>
}


