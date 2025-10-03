use sqlx::PgPool;
use crate::repositories::{InstrumentRepository, MarketDataRepository}; 

use std::error:Error;
use sqlx::PgPool; 

pub struct Database {
    pool: PgPool 
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
