-- Orders 
CREATE TABLE IF NOT EXISTS orders (
    order_id TEXT PRIMARY KEY, 
    account_id TEXT NOT NULL, 

    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    submitted_at TIMESTAMPTZ, 
    filled_at TIMESTAMPTZ, 

    status TEXT NOT NULL, 
    instrument_id TEXT NOT NULL, 
    FOREIGN KEY (instrument_id) REFERENCES instruments(instrument_id), 

    symbol TEXT NOT NULL,
    side TEXT NOT NULL, 
    type_order TEXT NOT NULL, 
    time_in_force TEXT NOT NULL, 
    
    qty DECIMAL NOT NULL, 
    filled_qty DECIMAL NOT NULL, 
    filled_avg_price DECIMAL NOT NULL, 

    instrument_class TEXT NOT NULL,
    position_intent TEXT NOT NULL, 
    notional DECIMAL NOT NULL, 
    limit_price DECIMAL NOT NULL, 
    stop_price DECIMAL NOT NULL, 

    extended_hours BOOLEAN NOT NULL
); 
 
-- Positions
-- Change instrument_id type to TEXT if needed
ALTER TABLE positions
    ALTER COLUMN instrument_id TYPE TEXT USING instrument_id::TEXT;

-- Rename existing columns to match new names
ALTER TABLE positions
    RENAME COLUMN quantity TO qty,
    RENAME COLUMN average_cost TO avg_entry_price,
    RENAME COLUMN unrealized_pnl TO unrealized_pl;

-- Add missing columns
ALTER TABLE positions
    ADD COLUMN qty_available DECIMAL,
    ADD COLUMN cost_basis DECIMAL,
    ADD COLUMN unrealized_plpc DECIMAL,
    ADD COLUMN unrealized_intraday_pl DECIMAL,
    ADD COLUMN unrealized_intraday_plpc DECIMAL,
    ADD COLUMN current_price DECIMAL,
    ADD COLUMN lastday_price DECIMAL,
    ADD COLUMN change_today DECIMAL,
    ADD COLUMN symbol TEXT,
    ADD COLUMN exchange TEXT,
    ADD COLUMN instrument_class TEXT,
    ADD COLUMN side TEXT,
    ADD COLUMN instrument_marginable BOOLEAN,
    ADD COLUMN last_update TIMESTAMPTZ;

-- Account alter
ALTER TABLE account
    RENAME COLUMN account_id TO id,
    ADD COLUMN effective_buying_power DECIMAL,
    ADD COLUMN daytrading_buying_power DECIMAL,
    ADD COLUMN regt_buying_power DECIMAL,
    ADD COLUMN non_marginable_buying_power DECIMAL,
    ADD COLUMN last_equity DECIMAL,
    ADD COLUMN sma DECIMAL,
    ADD COLUMN position_market_value DECIMAL,
    ADD COLUMN long_market_value DECIMAL,
    ADD COLUMN short_market_value DECIMAL,
    ADD COLUMN crypto_status TEXT,
    ADD COLUMN balance_asof TIMESTAMPTZ,
    ADD COLUMN daytrade_count DECIMAL;


-- Instrument Alter 

-- Rename existing columns to match Rust struct
ALTER TABLE instruments
    RENAME COLUMN instr_type TO instrument_class;

-- Change instrument_id type to TEXT if needed
ALTER TABLE instruments
    ALTER COLUMN instrument_id TYPE TEXT USING instrument_id::TEXT;

-- Add new columns from Rust struct
ALTER TABLE instruments
    ADD COLUMN status TEXT,
    ADD COLUMN tradeable BOOLEAN,
    ADD COLUMN marginable BOOLEAN,
    ADD COLUMN shortable BOOLEAN,
    ADD COLUMN fractionable BOOLEAN,
    ADD COLUMN easy_to_borrow BOOLEAN,
    ADD COLUMN maintenance_margin_requirement DECIMAL,
    ADD COLUMN margin_requirement_long DECIMAL,
    ADD COLUMN margin_requirement_short DECIMAL;
