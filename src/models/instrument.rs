#[derive(Debug, sqlx::FromRow)]
pub struct Instrument {
    pub instrument_id: String, 
    pub symbol: String, 
    pub name: String, 
    pub instrument_class: String, 
    pub exchange: String, 
    pub status: String, 
    
    pub tradeable: bool,
    pub marginable: bool,
    pub shortable: bool, 
    pub fractionable: bool,
    pub easy_to_borrow: bool, 

    // Addition 
    pub maintenance_margin_requirement: f64, 
    pub margin_requirement_long: f64,
    pub margin_requirement_short: f64
}
