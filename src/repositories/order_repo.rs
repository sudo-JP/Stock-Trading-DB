use crate::models::Order;
use crate::repositories::prelude_repo::*;

struct OrderRepository {
    pool: PgPool
}

pub struct Order {
    pub id: String, 
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
impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, order: &Order) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("
            INSERT INTO orders (id, client_order_id, created_at, updated_at, submitted_at, 
            filled_at, symbol, side, type_order, time_in_force, filled_qty, filled_avg_price) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
            ")
            .bind(&order.id)
            .bind(&order.client_order_id)
            .bind(&order.created_at)
            .bind(&order.updated_at)
            .bind(&order.updated_at)
            .bind(&order.submitted_at)
            .bind(&order.filled_at)
            .bind(&order.symbol)
            .bind(&order.side)
            .bind(&order.type_order)
            .bind(&order.time_in_force)
            .bind(order.filled_qty)
            .bind(order.filled_avg_price)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
