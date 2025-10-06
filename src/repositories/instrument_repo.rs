use crate::models::Instrument; 

pub struct InstrumentRepository {
    pool: PgPool 
}

impl InstrumentRepository {
    pub async fn find_by_symbol(&self, symbol: &str) -> Result<Instrument, Error> {
        let instrument = sqlx::query_as!(
            Instrument, 
            "SELECT * FROM instruments WHERE symbol = $1", 
            symbol 
            )
            .fetch_one(&self.pool)
            .await?; 

        Ok(instrument)   
    }

    /// Create a new instrument in the database
    ///
    /// # Arguments
    /// * `instrument` - The instrument data to insert
    ///
    /// # Returns
    /// * `Ok(Instrument)` - The created instrument with generated ID
    /// * `Err(Error)` - If insertion fails (e.g., duplicate symbol)
    ///
    /// # Example
    /// ```rust
    /// let instrument = Instrument { symbol: "AAPL", ... };
    /// let created = repo.create(&instrument).await?;
    /// ```
    pub async fn create(&self, instrument: &Instrument) -> Result<Instrument, Error> {
    }

    /// Retrieve all instruments from the database
    ///
    /// # Returns
    /// * `Ok(Vec<Instrument>)` - List of all instruments
    /// * `Err(Error)` - If query fails
    ///
    /// # Example
    /// ```rust
    /// let all_instruments = repo.list_all().await?;
    /// ```
    pub async fn list_all(&self) -> Result<Vec<Instrument>, Error> {

    }
}
