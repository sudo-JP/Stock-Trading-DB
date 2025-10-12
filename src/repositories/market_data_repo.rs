use crate::models::{MarketDataTick}; 

struct MarketDataRepository {
    pool: PgPool
}


impl MarketDataRepository {
    pub async fn insert_ticks(&self, ticks: &[MarketDataTick]) -> Result<(), Error> {
        let mut time_vec: Vec<datetime<utc>> = vec::new();
        let mut instr_vec: Vec<i32> = vec::new();
        let mut bid_price_vec: Vec<f32> = vec::new();
        let mut bid_size_vec: Vec<i32> = vec::new();
        let mut ask_price_vec: Vec<f32> = vec::new();
        let mut last_price_vec: Vec<f32> = vec::new();
        let mut volume_vec: Vec<i32> = vec::new();

        for tick in ticks {
            time_vec.push(tick.time);
            instr_vec.push(tick.instrument_id);
            bid_price_vec.push(tick.bid_price);
            bid_size_vec.push(tick.bid_size);
            ask_price_vec.push(tick.ask_price);
            last_price_vec.push(tick.last_price);
            volume_vec.push(tick.volume);
        }

        sqlx::query!("
        insert into market_data_ticks (time, instrument_id, bid_price, bid_size, ask_price, last_price, volume) select * from unnest(
            array[$1]::timestamptz[],
            array[$2]::integer[], 
            array[$3]::decimal[], 
            array[$4]::integer[],
            array[$5]::decimal[],
            array[$6]::decimal[],
            array[$7]::integer[]
        );", 
        time_vec, 
        instr_vec, 
        bid_price_vec, 
        bid_size_vec, 
        ask_price_vec, 
        last_price_vec, 
        volume_vec
        )
        .execute(&self.pool)
        .await?;

        Ok(())

    }

    /// Retrieve market data ticks within a time range for a specific instrument
    ///
    /// # Arguments
    /// * `instrument_id` - ID of the instrument
    /// * `start_time` - Start of time range (inclusive)
    /// * `end_time` - End of time range (inclusive)
    ///
    /// # Returns
    /// * `Ok(Vec<MarketDataTick>)` - Ticks within the specified range
    /// * `Err(Error)` - If query fails
    pub async fn get_ticks_by_time_range(
        &self,
        instrument_id: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<MarketDataTick>, Error>
    {

    }

    /// Generate OHLC bars from tick data for a given time interval
    ///
    /// # Arguments
    /// * `instrument_id` - ID of the instrument
    /// * `interval` - Time interval for bars ("1min", "5min", "1H", "1D")
    ///
    /// # Returns
    /// * `Ok(Vec<OhlcBar>)` - OHLC bars with open, high, low, close prices
    /// * `Err(Error)` - If query fails or no data available
    pub async fn get_ohlc_bars(
        &self,
        instrument_id: i32,
        interval: &str,
    ) -> Result<Vec<OhlcBar>, Error>
    {

    }
}
