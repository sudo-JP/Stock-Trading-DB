CREATE TABLE IF NOT EXISTS orders (
    order_id TEXT PRIMARY KEY, 
    account_id TEXT NOT NULL, 
    FOREIGN KEY (account_id) REFERENCES account(account_id),
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    submitted_at TIMESTAMPTZ, 
    filled_at TIMESTAMPTZ, 

    symbol TEXT NOT NULL,
    side TEXT NOT NULL, 
    type_order TEXT NOT NULL, 
    time_in_force TEXT NOT NULL, 
    
    filled_qty INTEGER DEFAULT 0, 
    filled_avg_price DECIMAL DEFAULT 0 
); 
 
