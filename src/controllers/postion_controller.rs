use crate::models::Position;
use anyhow::Result;
use crate::repositories::PositionRepository;
use crate::controllers::*;
use crate::protocols::{CppBinaryMessage, SQLCommand};


struct PositonController {
    repo: PositionRepository
}

fn validate_position(pos: &Position) -> Result<(), sqlx::Error> {
    if pos.symbol.is_empty() { return Err(sqlx::Error::Protocol("No symbol".into())); }
    if pos.exchange.is_empty() { return Err(sqlx::Error::Protocol("No exchange".into())); }
    if pos.qty < 0 { return Err(sqlx::Error::Protocol("Quantity cannot be negative".into())); }
    if pos.avg_entry_price < 0.0 { return Err(sqlx::Error::Protocol("Average entry price cannot be negative".into())); }

    Ok(())
}

impl CppController<Position, CppResult<Position>> for PositonController {
    async fn handle_operation(&self, bn: CppBinaryMessage, model: Position) -> Result<CppResult<Position>, sqlx::Error> {
        validate_position(&model)?;


        match bn.sql_command {
            SQLCommand::INSERT | SQLCommand::UPDATE => {
                self.repo.upsert(&model).await?;
            } 
            SQLCommand::DELETE => {
                self.repo.delete_by_instr_id(&model.instrument_id).await?;
            }
            SQLCommand::SELECT => {
                return Ok(CppResult::VALUE(self.repo.get_position_by_instrument(&model.instrument_id).await?));
            }
                _ => { return Err(sqlx::Error::Protocol("Unknown SQL command".into())); }
        }; 

        Ok(CppResult::SUCCESS)
    }
}
