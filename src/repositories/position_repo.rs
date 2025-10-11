use crate::models::Position;

pub struct PositionRepository {
    pool: PgPool 
}


impl PositionRepository {
    async fn get_position_by_instrument(&self, instrument_id: i32) -> Result<Position, Error> {
        let position = sqlx::query_as!(
            Position, 
            "SELECT * FROM positions WHERE instrument_id = $1;"
            )
            .fetch_one(&self.pool)
            .await?;

        Ok(position)
    }

}
