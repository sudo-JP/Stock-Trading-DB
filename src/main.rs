use networking::CppTCPServer; 
use stock_trading_db::{protocols::cpp_protocols, *};
use crate::models::Instrument;
use std::{thread, sync::{Arc, atomic::{AtomicBool, Ordering}}};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server = CppTCPServer::new()?;
    let handshake = match server.receive_event()? {
        cpp_protocols::Event::HANDSHAKE(hs) => hs, 
        _ => panic!("Expected handshake but got another signal")
    };

    let connect = Arc::new(AtomicBool::new(true)); 
    let mut children = vec![];

    for _ in 1..handshake.thread_count {
        let connect = Arc::clone(&connect); 
        let child = thread::spawn(move || {
            while connect.load(Ordering::Relaxed) {

            }
        }); 
        children.push(child);
    }

    match server.receive_event()? {
        cpp_protocols::Event::SHUTDOWN => println!("Shutting down Rust system..."), 
        _ => panic!("Expected shutdown but got another signal")
    };
    connect.store(false, Ordering::Relaxed);

    for child in children {
        child.join().unwrap();
    }

    Ok(())
}
