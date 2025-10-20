use crate::models::instruments::Instrument; 
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

    pub async fn create(&self, instrument: &Instrument) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"INSERT INTO instruments (symbol, name, instr_type, currency, exchange, multiplier, min_tick) VALUES ($1, $2, $3, $4, $5, $6, $7);"#
            )
            .bind(&instrument.symbol)
            .bind(&instrument.name)
            .bind(&instrument.instr_type)
            .bind(&instrument.currency)
            .bind(&instrument.exchange)
            .bind(&instrument.multiplier)
            .bind(&instrument.min_tick)
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
