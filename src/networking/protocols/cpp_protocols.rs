use byteorder::{LittleEndian};

#[repr(C, packed)]
pub struct BinaryMessage {
    sql_command: u32,
    table: u32, 
    timestamp: u64,
    data_size: u32
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
    INSTRUMENT = 4
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
struct AccountBinaryPayload {
    account_id: [u8; 64],
    currency: [u8; 4], 
    
    buying_power: f64,
    cash: f64,
    portfolio_value: f64, 
    equity: f64,

    // Performance
    unrealized_pl: f64,
    realized_pl: f64,

    status: i32, 
    last_update: i64
}


// Deserialize 
pub unsafe fn deserialize_header_cpp(header: &[u8]) -> () {
    // Binary length checking 
    if header.len() != size_of()
    let ptr = header.as_ptr();
    let bin_read = ptr.read_unaligned(); 

    //bin_read
    
}

pub unsafe fn deserialize_data_cpp(header: &BinaryMessage, packet: &[u8]) -> () {
    let ptr = packet.as_ptr();
    let bin_read = ptr.read_unaligned(); 
    
}
