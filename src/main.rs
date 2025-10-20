use stock_trading_db::*;
use crate::models::instruments::Instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::new().await?; 
    println!("Connected to database!");


    let instr_repo = InstrumentRepository::new(database.pool);
    let symbol = String::from("NVDA");
    // Try to query 
    let queried: Instrument = instr_repo.find_by_symbol(&symbol).await?; 
    println!("What {}", queried.symbol);

    Ok(())
}
