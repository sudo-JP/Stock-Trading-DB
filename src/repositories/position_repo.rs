use crate::models::positions::*; 
use crate::repositories::prelude_repo::*;

pub struct PositionRepository {
    pool: PgPool 
}


impl PositionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn get_position_by_instrument(&self, instrument_id: i32) -> Result<Position, sqlx::Error> {
        let position = sqlx::query_as::<sqlx::Postgres, Position>(
            "SELECT * FROM positions WHERE instrument_id = $1;" 
            )
            .bind(instrument_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(position)
    }


    // Archive the position by setting the quantity to 0 
    async fn close_position(&self, instrument_id: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE positions SET quantity = 0 WHERE positions.instrument_id = $1;"
            )
            .bind(instrument_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn update_position(&self, update: PositionUpdate) -> Result<Position, Error> {
        let position = sqlx::query_as::<sqlx::Postgres, Position>(
            "SELECT * FROM positions WHERE instrument_id = $1;")
            .bind(update.instrument_id)
            .fetch_one(&self.pool)
            .await?;
        

        let avg_cost = ((position.quantity * position.average_cost) + (update.quantity_change * update.trade_price) + update.commission) / (position.quantity + update.quantity_change);

        let updated: Position = Position {
            position_id: position.position_id,
            instrument_id: position.instrument_id, 
            quantity: position.quantity + update.quantity_change, 

            average_cost: avg_cost,

            unrealized_pnl: 0.0 // Change later when we have socket to calculate: Market Value - Total Cost 
        };
        // Use cached price for PnL calculation

        sqlx::query(
            "UPDATE positions SET quantity = $1, average_cost = $2, unrealized_pnl = $3 WHERE instrument_id = $4;")
            .bind(updated.quantity)
            .bind(updated.average_cost)
            .bind(updated.unrealized_pnl)
            .bind(updated.instrument_id)
            .execute(&self.pool)
            .await?;
    
        Ok(updated)
    }

}
