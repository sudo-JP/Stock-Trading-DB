
#[derive(Debug, sqlx::FromRow)]
pub struct Position {
    pub position_id: i32, 
    pub instrument_id: i32, 
    pub quantity: f32, 
    pub average_cost: f32, 
    pub unrealized_pnl: f32
}

#[derive(Debug, sqlx::FromRow)]
pub struct PositionUpdate {
    pub instrument_id: i32, 
    pub quantity_change: f32, 
    pub trade_price: f32, 
    pub commission: f32
}
