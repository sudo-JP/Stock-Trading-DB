use crate::models::trades::*; 
use crate::repositories::prelude_repo::*;
use std::collections::VecDeque;
use std::cmp;

pub struct TradeRepository {
    pool: PgPool 
}


impl TradeRepository {
    async fn create(&self, trade: &Trade) -> Result<Trade, Error> {
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
        let mut realized_pnl: RealizedPnl = RealizedPnl {
            instrument_id: instrument_id.clone(), 
            total_pnl: 0.0, 
            total_commission: 0.0, 
            net_pnl: 0.0, 
            trade_count: 0
        };
        
        let mut count = 0; 
        for trade in trades.iter() {
            match trade.trade_type {
                "BUY" => queue.push_back(trade.clone()), 
                "SELL" => { 
                    let result = pnl_sell(&trade, &queue)?;
                    realized_pnl.total_pnl += result.total_pnl; 
                    realized_pnl.net_pnl += result.net_pnl; 
                    realized_pnl.total_commission += result.total_commission;
                },
                _ => println!("Invalid trade type"),
            }

            count += 1; 
        }

        realized_pnl.trade_count = count;

        Ok(realized_pnl)
    }
}


fn pnl_sell(trade: &Trade, queue: &mut VecDeque<Trade>) -> Result<PnlResult, Error> {
    let mut remaining_quantity = trade.quantity;
    let mut result: PnlResult = PnlResult {
        total_pnl: 0,
        net_pnl: 0,
        total_commission: 0
    };

    while remaining_quantity > 0 && !queue.is_empty() {
        let mut node = queue.pop_front().unwrap();

        // pnl calculated by (sold price - buy price) * min(sold quantity, buy quantity)
        let matched_quantity = cmp::min(node.quantity, remaining_quantity);
        let pnl = (trade.price - node.price) * matched_quantity;

        // Remaining sell 
        remaining_quantity -= matched_quantity; 

        // Total commission
        let commission = (node.commission * matched_quantity / node.quantity) + (trade.commission * matched_quantity/ trade.quantity);

        node.quantity -= matched_quantity; 

        // update return  
        result.net_pnl += pnl - commission; 
        result.total_pnl += pnl;
        result.total_commission += commission;

        // Append the node back to the front of the queue 
        if node.quantity > 0 {
            queue.push_front(node);
        }
    }

    Ok(result)

}
