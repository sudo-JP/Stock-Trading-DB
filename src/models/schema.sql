CREATE TABLE IF NOT EXISTS instruments (
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    instr_type TEXT NOT NULL, 
    currency TEXT DEFAULT 'CAD', 
    exchange TEXT NOT NULL, 
    multiplier REAL DEFAULT 1, 
    min_tick REAL
); 

CREATE TABLE IF NOT EXISTS market_data_ticks (
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    symbol TEXT NOT NULL, 
    bid_price REAL NOT NULL, 
    bid_size INT NOT NULL, 
    ask_price REAL NOT NULL, 
    last_price REAL NOT NULL, 
    volume INT NOT NULL 
);


CREATE TABLE IF NOT EXISTS trading_transactions (
    symbol TEXT NOT NULL, 
    transaction_id INT
);


CREATE TABLE IF NOT EXISTS trading_sessions (
    session_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP 
);

-- Trading positions: where am at after selling/buying
CREATE TABLE IF NOT EXISTS positions (
    symbol TEXT NOT NULL, 
    quantity REAL NOT NULL, 
    average_cost REAL NOT NULL, 
    market_value REAL NOT NULL, 
    unrealized_pnl REAL NOT NULL 
); 

CREATE TABLE IF NOT EXISTS trades (
    trade_id INT NOT NULL, 
    trade_type TEXT NOT NULL,
    symbol TEXT NOT NULL, 
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    price REAL NOT NULL, 
    quantity REAL NOT NULL, -- because you can buy 0.4 stock
    comission REAL NOT NULL
); 

-- ML 
CREATE TABLE IF NOT EXISTS model_predicitons (
    prediction_id INT NOT NULL, 
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    symbol TEXT NOT NULL, 
    prediction_value REAL NOT NULL, 
    confidence REAL NOT NULL 
); 


CREATE TABLE IF NOT EXISTS technical_indicators (
    symbol TEXT NOT NULL, 
    time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, 
    sma_20 REAL NOT NULL,  -- Simple moving average over 20 period
    ema_12 REAL NOT NULL, -- exponential moving aerage 

    -- Momentum indicators 
    rst_14 REAL NOT NULL,
    macd REAL NOT NULL,

    -- Volatility indicators 
    bollinger_upper REAL NOT NULL, 
    bollinger_lower REAL NOT NULL, 
    atr_14 REAL NOT NULL 
);
