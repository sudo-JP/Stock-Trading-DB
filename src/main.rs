use networking::server; 
use stock_trading_db::*;
use crate::models::instruments::Instrument;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let database = Database::new().await?;
    println!("Connected to database!");


    let instr_repo = InstrumentRepository::new(database.pool);
    let symbol = String::from("NVDA");
    // Try to query 
    let queried: Instrument = instr_repo.find_by_symbol(&symbol).await?; 
    println!("What {}", queried.symbol);
    // Test the connection with timeout
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        sqlx::query("SELECT 1").execute(&database.pool)
    ).await {
        Ok(Ok(_)) => println!("Database query works!"),
        Ok(Err(e)) => println!("Database error: {}", e),
        Err(_) => println!("Database timeout - check connection string"),
    }*/

    let server = TCPServer::new()?;
    server.receive_data()?;

    Ok(())
}
