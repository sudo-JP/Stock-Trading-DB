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

    // ID and currency 
    let mut account_id = [0u8; 64];
    reader.read_exact(&mut account_id)?;

    let mut currency = [0u8; 4]; 
    reader.read_exact(&mut currency)?;

    // Owned Numeric 
    let cash = reader.read_f64::<LittleEndian>()?;
    let buying_power = reader.read_f64::<LittleEndian>()?;  
    let equity = reader.read_f64::<LittleEndian>()?;
    let portfolio = reader.read_f64::<LittleEndian>()?; 

    // Futures 
    let eff_buying_power = reader.read_f64::<LittleEndian>()?; 
    let daytrading_buying_power = reader.read_f64::<LittleEndian>()?; 
    let regt = reader.read_f64::<LittleEndian>()?; 
    let non_marg = reader.read_f64::<LittleEndian>()?; 
    let last_equity = reader.read_f64::<LittleEndian>()?; 
    let sma = reader.read_f64::<LittleEndian>()?; 
    let pos_mrk = reader.read_f64::<LittleEndian>()?; 
    let long_mrk = reader.read_f64::<LittleEndian>()?; 
    let short_mrk = reader.read_f64::<LittleEndian>()?; 

    // Metadata 
    let mut status = [0u8; 16]; 
    reader.read_exact(&mut status); 

    let mut cryp_stat = [0u8; 16]; 
    reader.read_exact(&mut cryp_stat); 

    let bal_asof = i64_to_nano(reader.read_i64::<LittleEndian>()?);
    let day_cnt = reader.read_f64::<LittleEndian>()?; 


    Ok(Account{
        id: bytes_to_string(&account_id),
        currency: bytes_to_string(&currency),

        // Owned numeric 
        cash: cash, 
        buying_power: buying_power, 
        equity: equity, 
        portfolio_value: portfolio, 

        // Futures 
        effective_buying_power: eff_buying_power, 
        daytrading_buying_power: daytrading_buying_power,
        regt_buying_power: regt, 
        non_marginable_buying_power: non_marg, 
        last_equity: last_equity, 
        sma: sma, 
        position_market_value: pos_mrk, 
        long_market_value: long_mrk, 
        short_market_value: short_mrk,

        // Metadata 
        status: bytes_to_string(&status),
        crypto_status: bytes_to_string(&cryp_stat), 
        balance_asof: bal_asof, 
        daytrade_count: day_cnt
    })
}


fn deserialize_order(packet: &[u8]) -> Result<Order> {
    let mut reader = Cursor::new(packet); 
    let mut id = [0u8; 64];
    reader.read_exact(&mut id)?;

    let created_at = reader.read_i64::<LittleEndian>()?;
    let updated_at = reader.read_i64::<LittleEndian>()?;
    let submitted_at = reader.read_i64::<LittleEndian>()?;
    let filled_at = reader.read_i64::<LittleEndian>()?;

    let mut status = [0u8; 16]; 
    reader.read_exact(&mut status)?;

    let mut asset_id = [0u8; 64]; 
    reader.read_exact(&mut asset_id)?;

    let mut symbol = [0u8; 16]; 
    reader.read_exact(&mut symbol)?;

    let mut side = [0u8; 8]; 
    reader.read_exact(&mut side)?; 

    let mut type_order = [0u8; 16]; 
    reader.read_exact(&mut type_order)?;


    let mut time_in_force = [0u8; 8]; 
    reader.read_exact(&mut time_in_force)?; 

    let qty = reader.read_f64::<LittleEndian>()?;
    let filled_qty = reader.read_f64::<LittleEndian>()?;
    let filled_avg_price = reader.read_f64::<LittleEndian>()?;

    let mut asset_class = [0u8; 16]; 
    reader.read_exact(&mut asset_class)?; 

    let mut position_intent = [0u8; 16]; 
    reader.read_exact(&mut position_intent)?; 

    let notional = reader.read_f64::<LittleEndian>()?;
    let limit_price = reader.read_f64::<LittleEndian>()?;
    let stop_price = reader.read_f64::<LittleEndian>()?;
    let extended_hours = reader.read_u8()? != 0;

    Ok(Order{
        order_id: bytes_to_string(&id), 

        created_at: i64_to_nano(created_at), 
        updated_at: i64_to_nano(updated_at), 
        submitted_at: i64_to_nano(submitted_at), 
        filled_at: i64_to_nano(filled_at), 
        status: bytes_to_string(&status), 

        instrument_id: bytes_to_string(&asset_id), 
        symbol: bytes_to_string(&symbol),
        side: bytes_to_string(&side), 
        type_order: bytes_to_string(&type_order), 
        time_in_force: bytes_to_string(&time_in_force), 

        qty: qty,
        filled_qty: filled_qty,
        filled_avg_price: filled_avg_price,

        instrument_class: bytes_to_string(&asset_class), 
        position_intent: bytes_to_string(&position_intent), 
        notional: notional, 

        limit_price: limit_price, 
        stop_price: stop_price, 
        extended_hours: extended_hours
    })
}

fn deserialize_position(packet: &[u8]) -> Result<Position> {
    let mut reader = Cursor::new(packet); 

    let mut asset_id = [0u8; 64];
    reader.read_exact(&mut asset_id)?;

    let mut symbol = [0u8; 16]; 
    reader.read_exact(&mut symbol)?;

    let mut exchange = [0u8; 8];
    reader.read_exact(&mut exchange)?; 

    let mut asset_class = [0u8; 16]; 
    reader.read_exact(&mut asset_class)?; 

    let mut side = [0u8; 8]; 
    reader.read_exact(&mut side)?; 

    let qty = reader.read_f64::<LittleEndian>()?;
    let qty_available = reader.read_f64::<LittleEndian>()?;

    let avg_entry_price = reader.read_f64::<LittleEndian>()?;
    let market_value = reader.read_f64::<LittleEndian>()?;
    let cost_basis = reader.read_f64::<LittleEndian>()?;

    let unrealized_pl = reader.read_f64::<LittleEndian>()?;
    let unrealized_plpc = reader.read_f64::<LittleEndian>()?;
    let unrealized_intraday_pl = reader.read_f64::<LittleEndian>()?;
    let unrealized_intraday_plpc = reader.read_f64::<LittleEndian>()?;

    let current_price = reader.read_f64::<LittleEndian>()?;
    let lastday_price = reader.read_f64::<LittleEndian>()?;
    let change_today = reader.read_f64::<LittleEndian>()?;

    let asset_marginable = reader.read_u8()? != 0;
    let last_update = i64_to_nano(reader.read_i64::<LittleEndian>()?);

    Ok(Position{
        instrument_id: bytes_to_string(&asset_id), 
        symbol: bytes_to_string(&symbol), 
        exchange: bytes_to_string(&exchange), 
        instrument_class: bytes_to_string(&asset_class), 
        side: bytes_to_string(&side), 
    

        qty: qty, 
        qty_available: qty_available, 

        avg_entry_price: avg_entry_price,
        market_value: market_value, 
        cost_basis: cost_basis,
        
        unrealized_pl: unrealized_pl, 
        unrealized_plpc: unrealized_plpc, 
        unrealized_intraday_pl: unrealized_intraday_pl,
        unrealized_intraday_plpc: unrealized_intraday_plpc,

        current_price: current_price, 
        lastday_price: lastday_price,
        change_today: change_today,

        instrument_marginable: asset_marginable, 
        last_update: last_update
    })

}


pub fn deserialize_asset(packet: &[u8]) -> Result<Instrument> {
    let mut reader = Cursor::new(packet); 
    let mut id = [0u8; 64];
    reader.read_exact(&mut id)?;

    let mut symbol= [0u8; 16]; 
    reader.read_exact(&mut symbol)?; 

    let mut name = [0u8; 64];
    reader.read_exact(&mut name)?;

    let mut asset_class = [0u8; 16];
    reader.read_exact(&mut asset_class)?;

    let mut exchange = [0u8; 16];
    reader.read_exact(&mut exchange)?; 

    let mut status = [0u8; 8];
    reader.read_exact(&mut status)?; 

    let tradeable = reader.read_u8()? != 0;
    let marginable = reader.read_u8()? != 0;
    let shortable = reader.read_u8()? != 0;
    let fractionable = reader.read_u8()? != 0;
    let easy_to_borrow = reader.read_u8()? != 0;

    let maintenance_margin_requirement = reader.read_f64::<LittleEndian>()?;
    let margin_requirement_long = reader.read_f64::<LittleEndian>()?;
    let margin_requirement_short = reader.read_f64::<LittleEndian>()?;

    Ok(Instrument{
        instrument_id: bytes_to_string(&id), 
        symbol: bytes_to_string(&symbol), 
        name: bytes_to_string(&name),
        instrument_class: bytes_to_string(&asset_class),
        exchange: bytes_to_string(&exchange), 
        status: bytes_to_string(&status),

        tradeable: tradeable, 
        marginable: marginable, 
        shortable: shortable, 
        fractionable: fractionable, 
        easy_to_borrow: easy_to_borrow, 
        
        maintenance_margin_requirement: maintenance_margin_requirement,

        margin_requirement_long: margin_requirement_long,
        margin_requirement_short: margin_requirement_short
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
