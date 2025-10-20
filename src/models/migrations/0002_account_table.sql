CREATE TABLE IF NOT EXISTS account (
    account_id TEXT PRIMARY KEY, 
    currency TEXT NOT NULL,
    
    buying_power DECIMAL NOT NULL, 
    cash DECIMAL NOT NULL, 
    portfolio_value DECIMAL NOT NULL, 
    equity DECIMAL NOT NULL,
    
    unrealized_pl DECIMAL NOT NULL,
    realized_pl DECIMAL NOT NULL, 

    status TEXT NOT NULL, 
    last_update TIMESTAMPTZ NOT NULL 
);
