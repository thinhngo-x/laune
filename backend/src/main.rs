use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;

mod config;
mod db;
mod error;
mod feeds;
mod models;
mod routes;
mod summarizer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,laune_backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Load configuration
    let settings = config::Settings::new()?;
    
    // Initialize database connection pool
    let db_pool = db::initialize_db(&settings).await?;
    
    // Test the database connection
    if let Err(e) = db::check_connection(&db_pool).await {
        tracing::error!("Database connection check failed: {}", e);
        return Err(anyhow::anyhow!("Database connection failed"));
    }
    
    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api", routes::api_router())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        // Add the database connection pool to the application state
        .with_state(db_pool);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Basic health check endpoint
async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") })),
    )
}
