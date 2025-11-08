use crate::models::Instrument; 
use crate::repositories::prelude_repo::*;


pub struct InstrumentRepository {
    pool: sqlx::PgPool 
}

impl InstrumentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_symbol(&self, symbol: &str) -> Result<Instrument, sqlx::Error> {
        let instrument = sqlx::query_as::<sqlx::Postgres, Instrument>( 
            "SELECT * FROM instruments WHERE symbol = $1;"
            )
            .bind(symbol)
            .fetch_one(&self.pool)
            .await?; 

        Ok(instrument)   
    }

    pub async fn upsert(&self, instrument: &Instrument) -> Result<bool, sqlx::Error> {
        let result = crate::sql_insert!(UPSERT, "instruments", instrument, 
            "instrument_id", instrument_id, symbol, name, instrument_class,
            exchange, status, tradeable, marginable, shortable,
            fractionable, easy_to_borrow, maintenance_margin_requirement, 
            margin_requirement_long, margin_requirement_short)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list_all(&self) -> Result<Vec<Instrument>, sqlx::Error> {
        let all_instr = sqlx::query_as::<sqlx::Postgres, Instrument>(
            "SELECT * FROM instruments;"
            )
            .fetch_all(&self.pool)
            .await?;

        Ok(all_instr)
    }

    pub async fn delete_by_symbol(&self, symbol: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM instruments WHERE symbol = $1;")
        .bind(symbol)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
