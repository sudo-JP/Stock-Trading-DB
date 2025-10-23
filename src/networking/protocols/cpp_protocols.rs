use std::io::{Cursor, Read};

use anyhow::{Result, bail};
use byteorder::{LittleEndian, ReadBytesExt};

#[repr(C, packed)]
pub struct BinaryMessage {
    pub sql_command: u32,
    pub table: u32, 
    pub timestamp: u64,
    pub data_size: u32
} 

#[repr(u32)]
enum SQLCommand {
    INSERT = 1, 
    SELECT = 2, 
    UPDATE = 3, 
    DELETE = 4
}

#[repr(u32)]
enum SQLTable {
    ACCOUNT = 1, 
    ORDER = 2, 
    POSITION = 3, 
    ASSET = 4
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
    id: [u8; 64],
    client_order_id: [u8; 64],
    created_at: u64, 
    updated_at: u64,
    submitted_at: u64,
    filled_at: u64,
    
    symbol: [u8; 16],
    side: [u8; 8],
    type_order: [u8; 16],
    time_in_force: u64, 

    filled_qty: u32,
    filled_avg_price: f32 
}

#[repr(C, packed)]
struct AssetBinaryPayload {
    id: [u8; 64], 
    asset_class: [u8; 16],
    exchange: [u8; 16],
    symbol: [u8; 16],
    name: [u8; 32],
    status: u32, 
    tradeable: u8,
    marginable: u8,
    shortable: u8, 
    easy_to_borrow: u8, 
    fractionable: u8
}

#[repr(C, packed)]
struct PositionBinaryPayload {
    asset_id: [u8; 64],
    symbol: [u8; 16],
    exchange: [u8; 16], 
    asset_class: [u8; 16],

    qty: u32,
    avg_entry_price: f64, 
    
    side: [u8; 8], 
    market_value: f64,
    cost_basis: f64, 

    
    unrealized_pl: f64,
    unrealized_plpc: f64,
    unrealized_intraday_pl: f64,
    unrealized_intraday_plpc: f64,
    current_price: f64,
    lastday_price: f64,
    change_today: f64,
}

#[repr(C, packed)]
pub struct AccountBinaryPayload {
    pub account_id: [u8; 64],
    pub currency: [u8; 4], 
    
    pub buying_power: f64,
    pub cash: f64,
    pub portfolio_value: f64, 
    pub equity: f64,

    // Performance
    pub unrealized_pl: f64,
    pub realized_pl: f64,

    pub status: i32, 
    pub last_update: i64
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
        sql_command: sql, 
        table: table, 
        timestamp: timestamp, 
        data_size: data_size} )
}

impl SQLTable {
    pub fn from_u32(num: u32) -> Option<SQLTable> {
        match num {
            1 => Some(SQLTable::ACCOUNT), 
            2 => Some(SQLTable::ORDER), 
            3 => Some(SQLTable::POSITION), 
            4 => Some(SQLTable::ASSET),
            _ => None
        }

    }
}

// compile safety for usize


pub fn deserialize_account(packet: &[u8]) -> Result<AccountBinaryPayload> {
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


    Ok(AccountBinaryPayload {
        account_id: account_id,
        currency: currency, 
        buying_power: buying_power, 
        cash: cash, 
        portfolio_value: portfolio, 
        equity: equity, 
        unrealized_pl: unrealized, 
        realized_pl: real, 
        status: status, 
        last_update: last_upd
    })
}

fn deserialize_order(packet: &[u8]) -> Result<PositionBinaryPayload> {

    bail!("")
}

fn deserialize_position(packet: &[u8]) -> Result<AccountBinaryPayload> {

    bail!("")
}

struct AssetBinaryPayload {
    id: [u8; 64], 
    asset_class: [u8; 16],
    exchange: [u8; 16],
    symbol: [u8; 16],
    name: [u8; 32],
    status: u32, 
    tradeable: u8,
    marginable: u8,
    shortable: u8, 
    easy_to_borrow: u8, 
    fractionable: u8
}

pub fn deserialize_asset(packet: &[u8]) -> Result<AssetBinaryPayload> {
    let mut reader = Cursor::new(packet); 
    let id = [0u8; 64]

    //let mut account_id = [0u8; 64];
    //reader.read_exact(&mut account_id)?;
    let asset_class = read_bin_arr<16>(&mut reader)?;
    let exchange = read_bin_arr<16>(&mut reader)?; 
    let symbol = read_bin_arr<16>(&mut reader)?; 
    let name = read_bin_arr<32>(&mut reader)?; 
    let status = reader.read_u32::<LittleEndian>()?;
    let tradeable = reader.read_u8::<LittleEndian>()?;
    let marginable = reader.read_u8::<LittleEndian>()?;
    let shortable = reader.read_u8::<LittleEndian>()?;
    let easy_to_borrow = reader.read_u8::<LittleEndian>()?;
    let fractionable = reader.read_u8::<LittleEndian>()?;

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

