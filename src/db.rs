use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
   
    let pool = PgPoolOptions::new()
        .max_connections(5) // Un bon point de départ, à ajuster selon la charge
        .connect(database_url)
        .await;

    //pour debogage
    match &pool {
        Ok(_) => println!("✅ Database connection pool created successfully."),
        Err(e) => eprintln!("🔥 Failed to create database connection pool: {}", e),
    }

    pool
}