use sqlx::PgPool;
/*use crate::repositories::{InstrumentRepository, MarketDataRepository}; */

//use std::error:Error;

pub struct Database {
    pub pool: PgPool 
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
