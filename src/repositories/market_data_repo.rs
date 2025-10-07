use crate::models::MarketDataTick; 

struct MarketDataRepository {
    pool: PgPool
}

impl MarketDataRepository {

    pub async fn insert_ticks(&self, ticks: &[MarketDataTick]) -> Result<(), Error> {
        let time: Vec<DateTime<Utc>> = ticks.iter().map(|tick| tick.time.clone()).collect();

        let instrument_id: Vec<i32> = ticks.iter().map(|tick| tick.instrument_id.clone()).collect();

        let bid_price: Vec<f32> = ticks.iter().map(|tick| tick.bid_price.clone()).collect();
        let bid_size: Vec<i32> = ticks.iter().map(|tick| tick.size.clone()).collect();
        let ticks = sqlx::query_as(
            r#"INSERT INTO market_data_tick
            (time, instrument_id, bid_price, bid_size, ask_price, last_price, volume)
            "#)
        .bind(time)
        .await?;

        Ok(ticks)
    }

    pub async fn get_ticks_by_time_range(
        &self,
        instrument_id: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>) -> Result<Vec<MarketDataTick>, Error> {

    }

    pub async fn get_ohlc_bars(
        &self,
        instrument_id: i32,
        interval: &str) -> Result<Vec<OhlcBar>, Error> {

    }
}
