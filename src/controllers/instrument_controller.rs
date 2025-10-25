use crate::models::Instrument;
use anyhow::Result;
use crate::repositories::InstrumentRepository;
use crate::controllers::*;
use crate::protocols::{CppBinaryMessage, SQLCommand};


fn validate_instrument(instr: &Instrument) -> Result<(), sqlx::Error> {
    if instr.symbol.is_empty() {
        return Err(sqlx::Error::Protocol("No symbol".into()));
    }
    else if instr.exchange.is_empty() {
        return Err(sqlx::Error::Protocol("No exchange".into()));
    }
    else if instr.multiplier <= 0.0 {
        return Err(sqlx::Error::Protocol("Negative multiplier".into()));
    }
    
    Ok(())

}

struct InstrumentController {
    repo: InstrumentRepository 
}

impl CppController<Instrument, CppResult<Instrument>> for InstrumentController {
    async fn handle_operation(&self, bn: CppBinaryMessage, model: Instrument) -> Result<CppResult<Instrument>, sqlx::Error> {
        validate_instrument(&model)?;

        match bn.sql_command {
            SQLCommand::INSERT | SQLCommand::UPDATE => {
                self.repo.upsert(&model).await?; 
            }
            SQLCommand::DELETE => {
                self.repo.delete_by_symbol(&model.symbol).await?;
            }
            SQLCommand::SELECT => {
                return Ok(CppResult::VALUE(self.repo.find_by_symbol(&model.symbol).await?))
            }
            _ => {
                return Err(sqlx::Error::Protocol("Unknown SQL command".into()));
            }
        }

        Ok(CppResult::SUCCESS)
    }

}
