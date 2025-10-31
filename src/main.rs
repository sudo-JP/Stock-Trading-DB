use networking::CppTCPServer; 
use stock_trading_db::*;
use crate::models::Instrument;
use std::thread;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server = CppTCPServer::new()?;
    let handshake = server.handshake()?;
    let handle = thread::spawn(move || {
        for _ in 1..handshake.thread_count {

        }
    });

    handle.join().unwrap(); 

    Ok(())
}
