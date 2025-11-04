use dotenvy;
use usize;

use anyhow::{Result, anyhow};
use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Read}, mem::size_of, net::{TcpListener, TcpStream}, vec
}; 
use crate::{networking::protocols::cpp_protocols::*, protocols::CppBinaryMessage};



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


fn handle_stream(mut stream: TcpStream) -> Result<(CppBinaryMessage, Vec<u8>)> {
    // First read the header

    let header: usize = size_of::<CppBinaryMessage>(); 
    let mut buffer = vec![0u8; header]; // Filed out buffer with 0 for header 

    // Second, deserialize the header to find the remaining size 
    stream.read_exact(&mut buffer)?;
    let header = deserialize_header_cpp(&buffer)?;
    let data_size: usize = usize::try_from(header.data_size)?;


    // Get the data 
    let mut buffer = vec![0u8; data_size]; // Filed out buffer with 0 for body
    stream.read_exact(&mut buffer)?;
    
    //cpp_protocols::deserialize_account(&buffer)?;

    // Now, read number of data from stream
    //let body = cpp_protocols::deserialize_data_cpp(&header, &buffer)?;

    //let body_data = cpp_protocols::deserialize_data_cpp(&header, )
    
    Ok((header, buffer))
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

    pub fn receive_event(&self) -> Result<Event> {
        let stream = self.listener.accept().unwrap();
        let (header, payload) = handle_stream(stream.0)?;
        Ok(match header.msg_type {
            MessageType::HANDSHAKE => Event::HANDSHAKE(craft_handshake(&payload)?),
            MessageType::SHUTDOWN => Event::SHUTDOWN,
            _ => Event::ERROR(anyhow!("Not valid event"))
        })

    }
}
