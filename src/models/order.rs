use crate::models::prelude_model::*; 

#[derive(Debug, sqlx::FromRow)]
pub struct Order {
    pub order_id: String, 
    pub client_order_id: String,
    pub created_at: DateTime<Utc>, 
    pub updated_at: DateTime<Utc>,
    pub submitted_at: DateTime<Utc>,
    pub filled_at: DateTime<Utc>,
    
    pub symbol: String,
    pub side: String, 
    pub type_order: String, 
    pub time_in_force: DateTime<Utc>, 

    pub filled_qty: i32, 
    pub filled_avg_price: f32 
}
