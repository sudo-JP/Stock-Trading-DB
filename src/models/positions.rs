pub struct Position {
    position_id: i32, 
    instrument_id: i32, 
    quantity: f32, 
    average_cost: f32, 
    unrealized_pnl: f32
}

pub struct PositionUpdate {
    instrument_id: i32, 
    quantity_change: f32, 
    trade_price: f32, 
    commission: f32
}
