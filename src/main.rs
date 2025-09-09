mod application;
mod domain;
mod infrastructure;
mod presentation;
mod shared;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use application::state::AppState;
use presentation::routes::create_app_router;
use shared::config::Config;
use shared::errors::AppError;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_learning_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().map_err(|e| AppError::Internal(e.to_string()))?;

    // Create AppState from configuration
    let app_state = AppState::from_config(&config).await?;

    // Build application using modular router
    let app = create_app_router(app_state);

    // Start server
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await?;

    tracing::info!(
        "Server starting on {}:{}",
        config.server_host,
        config.server_port
    );

    axum::serve(listener, app).await?;

    Ok(())
}
