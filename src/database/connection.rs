use sqlx::PgPool;
<<<<<<< HEAD
use crate::repositories::{InstrumentRepository, MarketDataRepository}; 

use std::error:Error;
use sqlx::PgPool; 
=======
/*use crate::repositories::{InstrumentRepository, MarketDataRepository}; */

//use std::error:Error;
>>>>>>> main

pub struct Database {
    pub pool: PgPool 
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
