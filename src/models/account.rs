#[derive(Debug, sqlx::FromRow)]
struct Account {
    pub account_id: String, 
    pub currency: String, 
    pub buying_power: f32, 
    pub cash: f32, 
    pub portfolio_value: f32, 
    pub equity: f32, 

    pub unrealized_pl: f32, 
    pub realized_pl: f32, 

    pub status: String,
    pub last_updated: DateTime<Utc>, 
}
