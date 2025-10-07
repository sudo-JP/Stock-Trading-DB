use crate::models::Trade; 

pub struct TradeRepository {
    pool: PgPool 
}

pub struct Trade {
    trade_id: i32, 
    instrument_id: i32, 
    trade_type: String, 
    time: DateTime<Utc>, 
    price: f32, 
    quantity: f32, 
    commission: f32 
}
impl TradeRepository {
    async fn create_trade(&self, trade: &Trade) -> Result<Trade, Error> {
        let result = sqlx::query_as!(
            Trade, 
            r#"INSERT INTO trades (instrument_id, trade_type, time, price, quantity, commission) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;"#
            
            trade.instrument_id,
            trade.trade_type,
            trade.time,
            trade.price,
            trade.quantity,
            trade.commission
            )
            .fetch_one(&self.pool).await?;

        Ok(result)
    }

    async fn get_trade_by_id(&self, trade_id: i32) -> Result<Trade, Error> {
        let trade = sqlx::query_as!(
            Trade, 
            "SELECT * FROM trades WHERE trade_id = $1;", 
            trade_id
            )
            .fetch_one(&self.pool).await?;
        Ok(trade)
    }

    async fn get_trades_by_instrument(&self, instrument_id: i32, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result <Vec<Trade>, Error> {

    }

    async fn get_recent_trades(&self, limit: usize) -> Result<Vec<Trade>, Error> {

    }

    async fn calculate_realized_pnl(&self, instrument_id: i32) -> Result<RealizedPnl, Error> {

    }
}
