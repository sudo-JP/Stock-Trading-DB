CREATE TABLE IF NOT EXISTS instruments (
    instrument_id SERIAL PRIMARY KEY UNIQUE, 
    symbol TEXT NOT NULL UNIQUE, 
    name TEXT NOT NULL,
    instr_type TEXT NOT NULL, 
    currency TEXT DEFAULT 'CAD', 
    exchange TEXT NOT NULL, 
    multiplier REAL DEFAULT 1, 
    min_tick REAL
); 

CREATE TABLE IF NOT EXISTS market_data_ticks (
    markt_data_tick_id SERIAL PRIMARY KEY,
    time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, 
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE CASCADE, 
    bid_price DECIMAL NOT NULL, 
    bid_size INT NOT NULL, 
    ask_price DECIMAL NOT NULL, 
    last_price DECIMAL NOT NULL, 
    volume INT NOT NULL 
);


-- Trading positions: where am at after selling/buying
CREATE TABLE IF NOT EXISTS positions (
    position_id SERIAL PRIMARY KEY, 
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE RESTRICT, 
    quantity DECIMAL NOT NULL, 
    average_cost DECIMAL NOT NULL, 
    market_value DECIMAL NOT NULL, 
    unrealized_pnl DECIMAL NOT NULL 
); 

CREATE TABLE IF NOT EXISTS trades (
    trade_id SERIAL PRIMARY KEY,
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE RESTRICT, 
    trade_type TEXT NOT NULL, -- buy/sell
    time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, 
    price DECIMAL NOT NULL, 
    quantity DECIMAL NOT NULL, -- because you can buy 0.4 stock
    commission DECIMAL NOT NULL
); 

CREATE TABLE labeled_data (
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE CASCADE,
    label_id SERIAL PRIMARY KEY,
    time TIMESTAMPTZ,
    future_return_5min DECIMAL,
    future_return_1H DECIMAL,
    regime_label TEXT
);

CREATE TABLE feature_sets (
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE CASCADE,
    feat_id SERIAL PRIMARY KEY,
    time TIMESTAMPTZ,
    feature_vector JSONB,  -- Store all features together
    feature_version TEXT
);

-- ML 
CREATE TABLE IF NOT EXISTS model_predictions (
    prediction_id SERIAL PRIMARY KEY, 
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE CASCADE, 
    time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, 
    prediction_value REAL NOT NULL, 
    confidence REAL NOT NULL 
); 


CREATE TABLE IF NOT EXISTS technical_indicators (
    tech_ind_id SERIAL PRIMARY KEY,
    instrument_id INTEGER REFERENCES instruments(instrument_id) ON DELETE RESTRICT, 
    time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, 
    sma_20 REAL NOT NULL,  -- Simple moving average over 20 period
    ema_12 REAL NOT NULL, -- exponential moving aerage 

    -- Momentum indicators 
    rsi_14 REAL NOT NULL,
    macd REAL NOT NULL,

    -- Volatility indicators 
    bollinger_upper REAL NOT NULL, 
    bollinger_lower REAL NOT NULL, 
    atr_14 REAL NOT NULL 
);

CREATE TABLE models (
    model_id SERIAL PRIMARY KEY,
    model_name TEXT,
    model_version TEXT,
    hyperparameters JSONB,
    trained_at TIMESTAMPTZ
);


CREATE INDEX CONCURRENTLY idx_market_data_time_instrument ON market_data_ticks(time, instrument_id);

CREATE INDEX CONCURRENTLY idx_trades_time_instrument ON trades(time, instrument_id);

CREATE INDEX CONCURRENTLY idx_technical_indicators_time_instrument ON technical_indicators(time, instrument_id);

