use sqlx::PgPool;
use std::env; 
use dotenvy::dotenvy;
use sqlx::Connection; 
use crate::database::{Database}; 

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); 

    let pg_user = std::env::var("POSTGRES_USER")?;
    let pg_pw = std::env::var("POSTGRES_PASSWORD")?;
    let pg_port = std::env::var("PORT")?;
    let pg_db = std::env::var("POSTGRES_DB")?;

    let url = format!("postgres://{}:{}@localhost:{}/{}", pg_user, pg_pw, pg_port, pg_db);
    let conn = sqlx::PgPool::connect(&url).await?;
    let database = Database::new(conn); 
    Ok(())
}
