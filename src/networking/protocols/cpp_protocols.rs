use std::io::{Cursor, Read};
pub use chrono::prelude::{DateTime, Utc};
use chrono::TimeZone; 
use crate::models::{Account, Position, Trade, Instrument};

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

#[repr(C, packed)]
pub struct BinaryMessage {
    pub sql_command: SQLCommand,
    pub table: SQLTable, 
    pub timestamp: u64,
    pub data_size: u32
} 


#[repr(u32)]
enum BinaryStatus {
    UNKNOWN = 0,
    ACTIVE = 1,
    INACTIVE = 2, 
    PENDING = 3,
    SUSPENDED = 4,
    CLOSED = 5,
    DELISTED = 6,
    MAINTENANCE = 7,
}


#[repr(C, packed)]
struct OrderBinaryPayload {
    pub id: [u8; 64],
    pub client_order_id: [u8; 64],
    pub created_at: u64, 
    pub updated_at: u64,
    pub submitted_at: u64,
    pub filled_at: u64,
    
    pub symbol: [u8; 16],
    pub side: [u8; 8],
    pub type_order: [u8; 16],
    pub time_in_force: u64, 

    pub filled_qty: u32,
    pub filled_avg_price: f32 
}

#[repr(C, packed)]
struct AssetBinaryPayload {
    pub id: [u8; 64], 
    pub asset_class: [u8; 16],
    pub exchange: [u8; 16],
    pub symbol: [u8; 16],
    pub name: [u8; 32],
    pub status: u32, 
    pub tradeable: u8,
    pub marginable: u8,
    pub shortable: u8, 
    pub easy_to_borrow: u8, 
    pub fractionable: u8
}

#[repr(C, packed)]
struct PositionBinaryPayload {
    pub asset_id: [u8; 64],
    pub symbol: [u8; 16],
    pub exchange: [u8; 16], 
    pub asset_class: [u8; 16],

    pub qty: u32,
    pub avg_entry_price: f64, 
    
    pub side: [u8; 8], 
    pub market_value: f64,
    pub cost_basis: f64, 

    
    pub unrealized_pl: f64,
    pub unrealized_plpc: f64,
    pub unrealized_intraday_pl: f64,
    pub unrealized_intraday_plpc: f64,
    pub current_price: f64,
    pub lastday_price: f64,
    pub change_today: f64,
}


// Deserialize 
pub fn deserialize_header_cpp(header: &[u8]) -> Result<BinaryMessage> {
    // Binary length checking 
    if header.len() != size_of::<BinaryMessage>() {
        bail!("Failed to deserialize header, header size mismatch");
    }

    let mut reader = Cursor::new(header); 
    let sql = reader.read_u32::<LittleEndian>()?;
    let table = reader.read_u32::<LittleEndian>()?;
    let timestamp = reader.read_u64::<LittleEndian>()?;
    let data_size = reader.read_u32::<LittleEndian>()?; 

    Ok(BinaryMessage {
        sql_command: match sql {
            1 => SQLCommand::INSERT, 
            2 => SQLCommand::SELECT,
            3 => SQLCommand::UPDATE, 
            4 => SQLCommand::DELETE, 
            _ => SQLCommand::UNKNOWN
        }, 
        table: match table {
            1 => SQLTable::ACCOUNT, 
            2 => SQLTable::ORDER, 
            3 => SQLTable::POSITION, 
            4 => SQLTable::POSITION,
            _ => SQLTable::UNKNOWN 
        }, 
        timestamp: timestamp, 
        data_size: data_size} )
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


fn deserialize_order(packet: &[u8]) -> Result<OrderBinaryPayload> {
    let mut reader = Cursor::new(packet); 
    let mut id = [0u8; 64];
    reader.read_exact(&mut id)?;

    let mut client_order_id = [0u8; 64]; 
    reader.read_exact(&mut client_order_id)?; 

    let created_at = reader.read_u64::<LittleEndian>()?;
    let updated_at = reader.read_u64::<LittleEndian>()?;
    let submitted_at = reader.read_u64::<LittleEndian>()?;
    let filled_at = reader.read_u64::<LittleEndian>()?;

    let mut symbol = [0u8; 16]; 
    reader.read_exact(&mut symbol)?;

    let mut side = [0u8; 8]; 
    reader.read_exact(&mut side)?; 

    let mut type_order = [0u8; 16]; 
    reader.read_exact(&mut type_order)?;

    let time_in_force = reader.read_u64::<LittleEndian>()?;
    let filled_qty = reader.read_u32::<LittleEndian>()?;
    let filled_avg_price = reader.read_f32::<LittleEndian>()?;

    Ok(OrderBinaryPayload{
        id: id, 
        client_order_id: client_order_id, 
        created_at: created_at, 
        updated_at: updated_at, 
        submitted_at: submitted_at, 
        filled_at: filled_at, 
        symbol: symbol, 
        side: side, 
        type_order: type_order, 
        time_in_force: time_in_force, 
        filled_qty: filled_qty, 
        filled_avg_price: filled_avg_price
    })
}

fn deserialize_position(packet: &[u8]) -> Result<PositionBinaryPayload> {
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

    Ok(PositionBinaryPayload{
        asset_id: asset_id, 
        symbol: symbol, 
        exchange: exchange, 
        asset_class: asset_class, 
        qty: qty, 
        avg_entry_price: avg_entry_price,
        side: side, 
        market_value: market_value, 
        cost_basis: cost_basis, 
        unrealized_pl: unrealized_pl, 
        unrealized_plpc: unrealized_plpc, 
        unrealized_intraday_pl: unrealized_intraday_pl,
        unrealized_intraday_plpc: unrealized_intraday_plpc,
        current_price: current_price, 
        lastday_price: lastday_price, 
        change_today: change_today
    })

}


pub fn deserialize_asset(packet: &[u8]) -> Result<AssetBinaryPayload> {
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
    let tradeable = reader.read_u8()?;
    let marginable = reader.read_u8()?;
    let shortable = reader.read_u8()?;
    let easy_to_borrow = reader.read_u8()?;
    let fractionable = reader.read_u8()?;

    Ok(AssetBinaryPayload{
        id: id, 
        asset_class: asset_class, 
        exchange: exchange, 
        symbol: symbol, 
        name: name, 
        status: status, 
        tradeable: tradeable, 
        marginable: marginable, 
        shortable: shortable, 
        easy_to_borrow: easy_to_borrow, 
        fractionable: fractionable, 
    })
    
}

