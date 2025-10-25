#[derive(Debug, sqlx::FromRow)]
pub struct Position {
    pub instrument_id: String, 
    pub symbol: String, 
    pub exchange: String, 
    pub instr_class: String, 

    pub qty: u32, 
    pub avg_entry_price: f64, 

    pub side: String, 
    pub market_value: f64, 

    pub average_cost: f64, 

    pub unrealized_pl: f64,
    pub unrealized_plpc: f64, 
    pub unrealized_intraday_pl: f64, 
    pub unrealized_intraday_plpc: f64, 
    
    pub current_price: f64, 
    pub lastday_price: f64,
    pub change_today: f64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PositionUpdate {
    pub instrument_id: i32, 
    pub quantity_change: f32, 
    pub trade_price: f32, 
    pub commission: f32
}
