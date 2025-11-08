use crate::models::Position;
use crate::repositories::prelude_repo::*;

pub struct PositionRepository {
    pool: PgPool 
}


impl PositionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn upsert(&self, pos: &Position) -> Result<bool, sqlx::Error> {
        let result = crate::sql_insert!(UPSERT, "positions", pos, 
            "instrument_id", instrument_id, symbol, exchange, 
            instrument_class, side, qty, qty_available, avg_entry_price,
            market_value, cost_basis, unrealized_pl, unrealized_plpc,
            unrealized_intraday_pl, unrealized_intraday_plpc,
            current_price, lastday_price, change_today, instrument_marginable,
            last_update)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_position_by_instrument(&self, instrument_id: &str) -> Result<Position, sqlx::Error> {
        let position = sqlx::query_as::<sqlx::Postgres, Position>(
            "SELECT * FROM positions WHERE instrument_id = $1;" 
            )
            .bind(instrument_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(position)
    }


    pub async fn delete_by_instr_id(&self, instrument_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM positions WHERE instrument_id = $1;")
            .bind(instrument_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
