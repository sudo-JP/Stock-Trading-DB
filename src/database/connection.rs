use sqlx::PgPool;
use dotenvy;

fn format_env() -> String {
    dotenvy::dotenv().ok(); 
    let e: &str = "Missing .env";
    let url: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var("POSTGRES_USER").expect(&e),
        std::env::var("POSTGRES_PASSWORD").expect(&e), 
        std::env::var("POSTGRES_HOST").expect(&e),      
        std::env::var("POSTGRES_PORT").expect(&e),
        std::env::var("POSTGRES_DB").expect(&e)
    );
    url
}

async fn schema_path(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./src/models/migrations/").run(&*pool).await?;
    Ok(())
}

pub struct Database {
    pub pool: PgPool 
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let url = format_env(); 
        let pool = sqlx::PgPool::connect(&url)
        .await
        .expect("Failed to connect to database");
        schema_path(&pool).await?; 

        Ok(Self { pool })
    }
}

