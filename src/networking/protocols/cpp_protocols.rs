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

