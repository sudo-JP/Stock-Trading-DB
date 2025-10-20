use stock_trading_db::*;
use crate::models::instruments::Instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database = Database::new().await?; 
    println!("Connected to database!");

    /*pub instrument_id: i32, 
    pub symbol: String, 
    pub name: String, 
    pub instr_type: String, 
    pub currency: String, 
    pub exchange: String, 
    pub multiplier: f32, 
    pub min_tick: Option<f32>*/

    // Test the connection with timeout

    /*let instr = Instrument {
        instrument_id: 0, 
    symbol: "NVDA".to_string(),
    name: "Nvidia".to_string(),
    instr_type: "market".to_string(),
    currency: "CAD".to_string(),
    exchange: "TSX".to_string(),
    multiplier: 1.0,
    min_tick: Some(1.0),
};
    let instr_repo = InstrumentRepository::new(database.pool);
    let sample: Instrument = instr_repo.create(&instr).await?;

    println!("Sample info {}", sample.symbol);*/

    let instr_repo = InstrumentRepository::new(database.pool);
    let symbol = String::from("NVDA");
    // Try to query 
    let queried: Instrument = instr_repo.find_by_symbol(&symbol).await?; 
    println!("What {}", queried.symbol);

    Ok(())
}
