use crate::models::Order;
use crate::repositories::prelude_repo::*;
use crate::sql_col;

pub struct OrderRepository {
    pool: PgPool
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, order: &Order) -> Result<bool, sqlx::Error> {
        let result = crate::sql_insert!(INSERT, "orders", order,
            order_id, created_at, updated_at, submitted_at, filled_at,
            status, instrument_id, symbol, side, type_order, time_in_force,
            qty, filled_qty, filled_avg_price, instrument_class, 
            position_intent, notional, limit_price, stop_price,
            extended_hours)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /*pub async fn update(&self, order: &Order) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE orders SET
            client_order_id = $1,
            updated_at = $2,
            submitted_at = $3,
            filled_at = $4,
            symbol = $5,
            side = $6,
            type_order = $7,
            time_in_force = $8,
            filled_qty = $9,
            filled_avg_price = $10
            WHERE order_id = $11;
            ")
            .bind(&order.client_order_id)
            .bind(order.updated_at)
            .bind(order.submitted_at)
            .bind(order.filled_at)
            .bind(&order.symbol)
            .bind(&order.side)
            .bind(&order.type_order)
            .bind(&order.time_in_force)
            .bind(order.filled_qty)
            .bind(order.filled_avg_price)
            .bind(&order.order_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }*/

    pub async fn delete_order_id(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM orders WHERE order_id = $1")
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0) 
    }

    pub async fn get_order_by_id(&self, id: &str) -> Result<Order, sqlx::Error> {
        let result = sqlx::query_as::<sqlx::Postgres, Order>("
            SELECT * FROM orders WHERE order_id = $1
            ")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(result)
    } 
}
