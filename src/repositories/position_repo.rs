use crate::models::{Position, PositionUpdate};

pub struct PositionRepository {
    pool: PgPool 
}


pub struct Position {
    position_id: i32, 
    instrument_id: i32, 
    quantity: f32, 
    average_cost: f32, 
    unrealized_pnl: f32
}
impl PositionRepository {
    /*async fn create_posititon(&self, position: &Position) -> Result<>{
        let position = sqlx::query_as!(

            )
            .fetch_one(&self.pool) 
            .await?;

        Ok(position)
    }*/

    async fn get_position_by_instrument(&self, instrument_id: i32) -> Result<Position, Error> {
        let position = sqlx::query_as!(
            Position, 
            "SELECT * FROM positions WHERE instrument_id = $1;", 
            instrument_id
            )
            .fetch_one(&self.pool)
            .await?;

        Ok(position)
    }


    // Archive the position by setting the quantity to 0 
    async fn close_position(&self, instrument_id: i32) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE positions SET quantity = 0 WHERE positions.instrument_id = $1;", 
            instrument_id
            )
            .execute(&self.pool)
            .await?
    }

    async fn update_position(&self, update: PositionUpdate) -> Result<Position, Error> {
        let mut position = sqlx::query_as!(
            Position, 
            "SELECT * FROM positions WHERE instrument_id = $1;",
            update.instrument_id
            )
            .fetch_one(&self.pool)
            .await?;
        

        let avg_cost = ((position.quantity * position.average_cost) + (update.quantity_change * update.trade_price) + update.commission) / (position.quantity + update.quantity);

        let updated: Position = Position {
            position_id: position.position_id,
            instrument_id: position.instrument_id, 
            quantity: position.quantity + update.quantity_change, 

            average_cost: avg_cost,

            unrealized_pnl: 0 // Change later when we have socket to calculate: Market Value - Total Cost 
        };
        // Use cached price for PnL calculation

        sqlx::query!(
            "UPDATE positions SET quantity = $1, average_cost = $2, unrealized_pnl = $3 WHERE instrument_id = $4;", 
            updated.quantity, 
            updated.average_cost, 
            updated.unrealized_pnl, 
            updated.instrument_id
            )
            .execute(&self.pool)
            .await?;
    
        Ok(updated)
    }

}
