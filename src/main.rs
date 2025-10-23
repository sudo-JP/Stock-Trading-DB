use networking::CppTCPServer; 
use stock_trading_db::*;
use crate::models::instruments::Instrument;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server = CppTCPServer::new()?;
    server.receive_data()?;

    Ok(())
}
