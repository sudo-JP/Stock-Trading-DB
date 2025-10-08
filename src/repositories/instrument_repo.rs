use crate::models::Instrument; 

pub struct InstrumentRepository {
    pool: PgPool 
}

impl InstrumentRepository {
    pub async fn find_by_symbol(&self, symbol: &str) -> Result<Instrument, Error> {
        let instrument = sqlx::query_as!(
            Instrument, 
            "SELECT * FROM instruments WHERE symbol = $1;", 
            symbol 
            )
            .fetch_one(&self.pool)
            .await?; 

        Ok(instrument)   
    }

    pub async fn create(&self, instrument: &Instrument) -> Result<Instrument, Error> {
        let result = sqlx::query_as!(
            Instrument, 
            r#"INSERT INTO instruments (symbol, name, instr_type, currency, exchange, multiplier, min_tick) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;"#
            
            instrument.symbol,
            instrument.name,
            instrument.instr_type,
            instrument.currency,
            instrument.exchange,
            instrument.multiplier,
            instrument.min_tick
            )
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn list_all(&self) -> Result<Vec<Instrument>, Error> {
        let all_instr = sqlx::query_as!(
            Instrument, 
            "SELECT * FROM instruments;"
            )
            .fetch_all(&self.pool)
            .await?;

        Ok(all_instr)
    }
}
