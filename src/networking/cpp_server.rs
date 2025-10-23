use dotenvy;
use usize;

use anyhow::{Result, bail};
use std::{
    io::{Read, Error},
    net::{TcpListener, TcpStream},
    mem::{size_of}
}; 
use crate::networking::protocols::cpp_protocols;



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

pub struct CppTCPServer {
    listener: TcpListener,
}

fn handle_stream(mut stream: TcpStream) -> Result <()> {
    // First read the header
    let header: usize = size_of::<cpp_protocols::BinaryMessage>(); 
    let mut buffer = vec![0u8; header]; // Filed out buffer with 0 for header 

    // Second, deserialize the header to find the remaining size 
    stream.read_exact(&mut buffer)?;
    let header = cpp_protocols::deserialize_header_cpp(&buffer)?;

    // Now, read number of data from stream
    let data_size: usize = usize::try_from(header.data_size)?;
    let mut buffer = vec![0u8; data_size]; // Filed out buffer with 0 for body
    //let body = cpp_protocols::deserialize_data_cpp(&header, &buffer)?;

    //let body_data = cpp_protocols::deserialize_data_cpp(&header, )
    
    Ok(())
}

impl CppTCPServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let network = format_env();
        let listener = TcpListener::bind(network)?;
        Ok(Self { listener })
    }

    pub fn receive_data(&self) -> Result<(), Error> {

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            match handle_stream(stream) {
                Err(e) => { eprint!("Error parsing stream {}", e); }
                _ => {}
            }
        }

        Ok(())
    }
}
