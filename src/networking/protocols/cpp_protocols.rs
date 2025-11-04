use std::io::{Cursor, Read};
pub use chrono::prelude::{DateTime, Utc};
use chrono::TimeZone; 
use crate::models::{Account, Position, Order, Instrument};

use anyhow::{Result, bail};
use byteorder::{LittleEndian, ReadBytesExt};

#[repr(u32)]
pub enum SQLCommand {
    INSERT, 
    SELECT, 
    UPDATE, 
    DELETE, 
    UNKNOWN
}

#[repr(u32)]
pub enum SQLTable {
    ACCOUNT, 
    ORDER, 
    POSITION, 
    ASSET,
    UNKNOWN,
}

pub struct Handshake {
    pub thread_count: u32,
    pub port_range: u32 
}

pub enum Event {
    HANDSHAKE(Handshake), 
    SHUTDOWN, 
    ERROR(anyhow::Error)
}

#[repr(u32)]
pub enum MessageType {
    HANDSHAKE, 
    SHUTDOWN, 
    DB, 
    UNKNOWN 
}

#[repr(C, packed)]
pub struct CppBinaryMessage {
    pub msg_type: MessageType, 
    pub table: SQLTable, 
    pub sql_command: SQLCommand,
    pub data_size: u32
} 


// Deserialize 
pub fn deserialize_header_cpp(header: &[u8]) -> Result<CppBinaryMessage> {
    // Binary length checking 
    if header.len() != size_of::<CppBinaryMessage>() {
        bail!("Failed to deserialize header, header size mismatch");
    }

    let mut reader = Cursor::new(header); 
    let msg_type = reader.read_u32::<LittleEndian>()?; 
    let table = reader.read_u32::<LittleEndian>()?;
    let sql = reader.read_u32::<LittleEndian>()?;
    let data_size = reader.read_u32::<LittleEndian>()?; 

    Ok(CppBinaryMessage {
        msg_type: match msg_type {
            1 => MessageType::HANDSHAKE, 
            2 => MessageType::SHUTDOWN, 
            3 => MessageType::DB, 
            _ => MessageType::UNKNOWN
        }, 
        table: match table {
            1 => SQLTable::ACCOUNT, 
            2 => SQLTable::ORDER, 
            3 => SQLTable::POSITION, 
            4 => SQLTable::POSITION,
            _ => SQLTable::UNKNOWN 
        }, 
        sql_command: match sql {
            1 => SQLCommand::INSERT, 
            2 => SQLCommand::SELECT,
            3 => SQLCommand::UPDATE, 
            4 => SQLCommand::DELETE, 
            _ => SQLCommand::UNKNOWN
        }, 
        data_size: data_size})
}


fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim_end_matches('\0').to_string()
}

fn i64_to_nano(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_nanos(timestamp)
}


pub fn deserialize_account(packet: &[u8]) -> Result<Account> {
    let mut reader = Cursor::new(packet); 
    let mut account_id = [0u8; 64];
    reader.read_exact(&mut account_id)?;

    let mut currency = [0u8; 4]; 
    reader.read_exact(&mut currency)?;

    let buying_power = reader.read_f64::<LittleEndian>()?;  

    let cash = reader.read_f64::<LittleEndian>()?;
    let portfolio = reader.read_f64::<LittleEndian>()?; 
    let equity = reader.read_f64::<LittleEndian>()?;
    let unrealized = reader.read_f64::<LittleEndian>()?;
    let real = reader.read_f64::<LittleEndian>()?; 

    let status = reader.read_i32::<LittleEndian>()?;
    let last_upd = reader.read_i64::<LittleEndian>()?;


    Ok(Account{
        account_id: bytes_to_string(&account_id),
        currency: bytes_to_string(&currency),
        buying_power: buying_power, 
        cash: cash, 
        portfolio_value: portfolio, 
        equity: equity, 
        unrealized_pl: unrealized, 
        realized_pl: real, 
        status: match status {
            1 => "ACTIVE".to_owned(),
            2 => "INACTIVE".to_owned(), 
            _ => "UNKNOWN".to_owned()
        }, 
        last_update: i64_to_nano(last_upd)
    })
}


fn deserialize_order(packet: &[u8]) -> Result<Order> {
    let mut reader = Cursor::new(packet); 
    let mut id = [0u8; 64];
    reader.read_exact(&mut id)?;

    let mut client_order_id = [0u8; 64]; 
    reader.read_exact(&mut client_order_id)?; 

    let created_at = reader.read_i64::<LittleEndian>()?;
    let updated_at = reader.read_i64::<LittleEndian>()?;
    let submitted_at = reader.read_i64::<LittleEndian>()?;
    let filled_at = reader.read_i64::<LittleEndian>()?;

    let mut symbol = [0u8; 16]; 
    reader.read_exact(&mut symbol)?;

    let mut side = [0u8; 8]; 
    reader.read_exact(&mut side)?; 

    let mut type_order = [0u8; 16]; 
    reader.read_exact(&mut type_order)?;


    let mut time_in_force = [0u8; 8]; 
    reader.read_exact(&mut time_in_force)?; 
    let filled_qty = reader.read_u32::<LittleEndian>()?;
    let filled_avg_price = reader.read_f32::<LittleEndian>()?;

    Ok(Order{
        order_id: bytes_to_string(&id), 
        client_order_id: bytes_to_string(&client_order_id), 
        created_at: i64_to_nano(created_at), 
        updated_at: i64_to_nano(updated_at), 
        submitted_at: i64_to_nano(submitted_at), 
        filled_at: i64_to_nano(filled_at), 
        symbol: bytes_to_string(&symbol),
        side: bytes_to_string(&side), 
        type_order: bytes_to_string(&type_order), 
        time_in_force: bytes_to_string(&time_in_force), 
        filled_qty: filled_qty as i32,
        filled_avg_price: filled_avg_price
    })
}

fn deserialize_position(packet: &[u8]) -> Result<Position> {
    let mut reader = Cursor::new(packet); 
    let mut asset_id = [0u8; 64];
    reader.read_exact(&mut asset_id)?;

    let mut symbol = [0u8; 16]; 
    reader.read_exact(&mut symbol)?;

    let mut exchange = [0u8; 16];
    reader.read_exact(&mut exchange)?; 

    let mut asset_class = [0u8; 16]; 
    reader.read_exact(&mut asset_class)?; 

    let qty = reader.read_u32::<LittleEndian>()?;
    let avg_entry_price = reader.read_f64::<LittleEndian>()?;

    let mut side = [0u8; 8]; 
    reader.read_exact(&mut side)?; 

    let market_value = reader.read_f64::<LittleEndian>()?;
    let cost_basis = reader.read_f64::<LittleEndian>()?;

    let unrealized_pl = reader.read_f64::<LittleEndian>()?;
    let unrealized_plpc = reader.read_f64::<LittleEndian>()?;

    let unrealized_intraday_pl = reader.read_f64::<LittleEndian>()?;
    let unrealized_intraday_plpc = reader.read_f64::<LittleEndian>()?;
    let current_price = reader.read_f64::<LittleEndian>()?;
    let lastday_price = reader.read_f64::<LittleEndian>()?;
    let change_today = reader.read_f64::<LittleEndian>()?;


    Ok(Position{
        instrument_id: bytes_to_string(&asset_id), 
        symbol: bytes_to_string(&symbol), 
        exchange: bytes_to_string(&exchange), 
        instr_class: bytes_to_string(&asset_class), 
        qty: qty as i32, 
        avg_entry_price: avg_entry_price,
        side: bytes_to_string(&side), 
        cost_basis: cost_basis,
        average_cost: avg_entry_price, // About the same, will change it 
        market_value: market_value, 
        unrealized_pl: unrealized_pl, 
        unrealized_plpc: unrealized_plpc, 
        unrealized_intraday_pl: unrealized_intraday_pl,
        unrealized_intraday_plpc: unrealized_intraday_plpc,
        current_price: current_price, 
        lastday_price: lastday_price,
        change_today: change_today
    })

}


pub fn deserialize_asset(packet: &[u8]) -> Result<Instrument> {
    let mut reader = Cursor::new(packet); 
    let mut id = [0u8; 64];
    reader.read_exact(&mut id)?;

    let mut asset_class = [0u8; 16]; 
    reader.read_exact(&mut asset_class)?; 

    let mut exchange = [0u8; 16];
    reader.read_exact(&mut exchange)?; 

    let mut symbol = [0u8; 16]; 
    reader.read_exact(&mut symbol)?;

    let mut name = [0u8; 32];
    reader.read_exact(&mut name)?;

    let status = reader.read_u32::<LittleEndian>()?;
    let tradeable = reader.read_u8()? != 0;
    let marginable = reader.read_u8()? != 0;
    let shortable = reader.read_u8()? != 0;
    let easy_to_borrow = reader.read_u8()? != 0;
    let fractionable = reader.read_u8()? != 0;

    Ok(Instrument{
        instrument_id: bytes_to_string(&id), 
        instrument_class: bytes_to_string(&asset_class), 
        exchange: bytes_to_string(&exchange), 
        symbol: bytes_to_string(&symbol), 
        name: bytes_to_string(&name),
        status: match status {
            1 => "ACTIVE".to_owned(), 
            2 => "INACTIVE".to_owned(), 
            3 => "PENDING".to_owned(), 
            4 => "SUSPENDED".to_owned(), 
            5 => "CLOSED".to_owned(), 
            6 => "DELISTED".to_owned(), 
            7 => "MAINTENANCE".to_owned(),
            _ => "UNKNOWN".to_owned(),
        }, 
        tradeable: tradeable, 
        marginable: marginable, 
        shortable: shortable, 
        easy_to_borrow: easy_to_borrow, 
        fractionable: fractionable, 
        currency: "USD".to_owned(),
        instr_type: bytes_to_string(&asset_class), 
        multiplier: 1.0, 
        min_tick: None, 

    })
    
}


pub fn craft_handshake(packet: &[u8]) -> Result<Handshake> {
    let mut reader = Cursor::new(packet); 
    let threads = reader.read_u32::<LittleEndian>()?;
    let ports = reader.read_u32::<LittleEndian>()?;
    Ok(Handshake{
        thread_count: threads,
        port_range: ports
    })
}
