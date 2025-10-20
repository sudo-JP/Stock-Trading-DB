use dotenvy;
use std::{
    io::{BufRead, BufReader, Error},
    net::{TcpListener, TcpStream},
};

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
    listener: TcpListener,
}

fn handle_connection(stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    println!("Data: {}", line);
}

impl TCPServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let network = format_env();
        let listener = TcpListener::bind(network)?;
        Ok(Self { listener })
    }

    pub fn receive_data(&self) -> Result<(), Error> {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }

        Ok(())
    }
}
