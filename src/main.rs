//mod database;
//use crate::database::Database;

mod networking;
use crate::networking::TCPServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let database = Database::new().await?;
    println!("Connected to database!");

    // Test the connection with timeout
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        sqlx::query("SELECT 1").execute(&database.pool)
    ).await {
        Ok(Ok(_)) => println!("Database query works!"),
        Ok(Err(e)) => println!("Database error: {}", e),
        Err(_) => println!("Database timeout - check connection string"),
    }*/

    let server = TCPServer::new()?;
    server.receive_data()?;

    Ok(())
}
