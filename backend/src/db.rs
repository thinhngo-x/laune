use crate::config::Settings;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

// Type alias for database connection pool
pub type DbPool = Pool<Postgres>;

// Initialize database connection
pub async fn initialize_db(settings: &Settings) -> Result<DbPool> {
    info!(
        "Connecting to PostgreSQL at {}:{}",
        settings.database.host, settings.database.port
    );

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&settings.database.connection_string())
        .await?;

    info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database initialized successfully");
    Ok(pool)
}

// Function to check database connection
pub async fn check_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
