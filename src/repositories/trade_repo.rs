use crate::models::{Trade, RealizedPnl}; 
use std::collections::VecDeque;

pub struct TradeRepository {
    pool: PgPool 
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
            .fetch_one(&self.pool)
            .await?;

        Ok(result)
    }

    async fn get_trade_by_id(&self, trade_id: i32) -> Result<Trade, Error> {
        let trade = sqlx::query_as!(
            Trade, 
            "SELECT * FROM trades WHERE trade_id = $1;", 
            trade_id
            )
            .fetch_one(&self.pool)
            .await?;
        Ok(trade)
    }

    async fn get_trades_by_instrument(&self, instrument_id: i32, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Result <Vec<Trade>, Error> {
        let trades = sqlx::query_as!(
            Trade, 
            "SELECT * FROM trades WHERE trades.instrument_id = $1 AND trades.time >= $2 AND trades.time <= $3;", 
            instrument_id, 
            start_time, 
            end_time
            )
            .fetch_all(&self.pool)
            .await?;
        Ok(trades)
    }

    async fn get_recent_trades(&self, limit: usize) -> Result<Vec<Trade>, Error> {
        let trades = sqlx::query_as!(
            Trade, 
            "SELECT * FROM trades ORDER BY trades.time DESC LIMIT $1;", 
            limit
            )
            .fetch_all(&self.pool)
            .await?; 
        Ok(trades)
    }

    async fn calculate_realized_pnl(&self, instrument_id: i32) -> Result<RealizedPnl, Error> {
        let trades = sqlx::query_as!(
            Trade, 
            "SELECT * FROM trades WHERE trades.instrument_id = $1;",
            instrument_id
            )
            .fetch_all(&self.pool)
            .await?;
        
        let mut queue: VecDeque<Trade> = VecDeque::new(); 

        for trade in trades.iter() {
            match trade.trade_type {
                &"BUY" => queue.push_back(trade), 
                &"SELL" => queue.pop_front(), 
                _ => println!("Invalid trade type"),
            }
        }

        Ok(pnl)
    }
}
