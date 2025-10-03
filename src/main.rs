//use sqlx::PgPool;
use dotenvy;
mod database; 
//use sqlx::Connection; 
use crate::database::{Database}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok(); 

    let pg_user = std::env::var("POSTGRES_USER")?;
    let pg_pw = std::env::var("POSTGRES_PASSWORD")?;
    let pg_port = std::env::var("PORT")?;
    let pg_db = std::env::var("POSTGRES_DB")?;

    let url = format!("postgres://{}:{}@postgres:{}/{}", pg_user, pg_pw, pg_port, pg_db);
    let conn = sqlx::PgPool::connect(&url)
        .await
        .expect("Failed to connect to database");
    let database = Database::new(conn); 
    println!("Connected to database!");

    // Test the connection with timeout
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        sqlx::query("SELECT 1").execute(&database.pool)
    ).await {
        Ok(Ok(_)) => println!("Database query works!"),
        Ok(Err(e)) => println!("Database error: {}", e),
        Err(_) => println!("Database timeout - check connection string"),
    }

    Ok(())
}
