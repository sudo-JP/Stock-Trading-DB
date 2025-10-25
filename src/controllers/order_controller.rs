use crate::models::Order;
use anyhow::{Result, anyhow};
use crate::repositories::OrderRepository;
use crate::controllers::*;
use crate::protocols::{CppBinaryMessage, SQLCommand};
use sqlx::Error;
use chrono::Utc;

struct OrderController {
    repo: OrderRepository 
}

pub fn validate_order(order: &Order) -> Result<(), Error> {
    if order.order_id.is_empty() {
        return Err(Error::Protocol("Order id is empty".into()));
    }

    if order.symbol.is_empty() {
        return Err(Error::Protocol("Order symbol is empty".into()));
    }

    if order.side != "buy" && order.side != "sell" {
        return Err(Error::Protocol("Order side must be 'buy' or 'sell'".into()));
    }

    if order.type_order.is_empty() {
        return Err(Error::Protocol("Order type is empty".into()));
    }

    if order.filled_qty < 0 {
        return Err(Error::Protocol("Filled quantity cannot be negative".into()));
    }

    if order.filled_avg_price < 0.0 {
        return Err(Error::Protocol("Filled average price cannot be negative".into()));
    }

    let now = Utc::now();
    if order.created_at > now {
        return Err(Error::Protocol("Created_at cannot be in the future".into()));
    }
    if order.submitted_at > now {
        return Err(Error::Protocol("Submitted_at cannot be in the future".into()));
    }
    if order.filled_at > now {
        return Err(Error::Protocol("Filled_at cannot be in the future".into()));
    }

    Ok(())
}

impl CppController<Order, CppResult<Order>> for OrderController {
    async fn handle_operation(&self, bn: CppBinaryMessage, model: Order) -> Result<CppResult<Order>, Error> {
        validate_order(&model)?; 

        match bn.sql_command {
            SQLCommand::INSERT => {
                self.repo.insert(&model).await?;
            } 
            SQLCommand::UPDATE => {
                self.repo.update(&model).await?;
            }
            SQLCommand::DELETE => {
                self.repo.delete_order_id(&model.order_id).await?;
            }
            SQLCommand::SELECT => {
                return Ok(CppResult::VALUE(self.repo.get_order_by_id(&model.order_id).await?));
            }
                _ => { return Err(sqlx::Error::Protocol("Unknown SQL command".into())); }
        }; 

        Ok(CppResult::SUCCESS)
    }
}
