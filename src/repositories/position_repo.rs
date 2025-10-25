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
        sqlx::query(
            "INSERT INTO positions (instrument_id, symbol, exchange, instr_class,
            qty, cost_basis, avg_entry_price, side, market_value, average_cost,
            unrealized_pl, unrealized_plpc, unrealized_intraday_pl, unrealized_intraday_plpc, 
            current_price, lastday_price, change_today) VALUES ($1, $2, $3, $4, $5, $6, $7, 
            $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) ON CONFLICT (account_id, instrument_id)
            qty = EXCLUDED.qty,
            avg_entry_price = EXCLUDED.avg_entry_price,
            market_value = EXCLUDED.market_value,
            unrealized_pl = EXCLUDED.unrealized_pl,
            current_price = EXCLUDED.current_price;"
            )
            .bind(&pos.instrument_id)
            .bind(&pos.symbol)
            .bind(&pos.exchange)
            .bind(&pos.instr_class)
            .bind(pos.qty)
            .bind(pos.cost_basis)
            .bind(pos.avg_entry_price)
            .bind(&pos.side)
            .bind(pos.market_value)
            .bind(pos.average_cost)
            .bind(pos.unrealized_pl)
            .bind(pos.unrealized_plpc)
            .bind(pos.unrealized_intraday_pl)
            .bind(pos.unrealized_intraday_plpc)
            .bind(pos.current_price)
            .bind(pos.lastday_price)
            .bind(pos.change_today)
            .execute(&self.pool)
            .await?;

        Ok(true)
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


    // Archive the position by setting the quantity to 0 
    pub async fn close_position(&self, instrument_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE positions SET quantity = 0 WHERE positions.instrument_id = $1;"
            )
            .bind(instrument_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_by_instr_id(&self, instrument_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM positions WHERE instrument_id = $1;")
            .bind(instrument_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
