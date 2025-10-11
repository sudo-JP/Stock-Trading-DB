use crate::models::Position;

pub struct PositionRepository {
    pool: PgPool 
}


impl PositionRepository {
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
            .await?;
        Ok(())
    }

}
