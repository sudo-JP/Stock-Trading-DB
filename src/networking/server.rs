use dotenvy;
use std::net::TcpListener;

fn format_env() -> String {
    dotenvy::dotenv().ok(); 
    let e: &str = "Missing .env";
    let network: String = format!(
        "{}:{}",
        std::env::var("TCP_HOST").expect(&e),
        std::env::var("TCP_PORT").expect(&e), 
    );
    network 
}

pub struct TCPServer {
    listener: TcpListener
} 

impl TCPServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let network = format_env();
        let listener = TcpListener::bind(network)?;

        Ok(Self { listener })
    }
    
}


