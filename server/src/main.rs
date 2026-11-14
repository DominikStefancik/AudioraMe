use axum::{
    Router,
    routing::{get, post},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{
    config::ConfigManager,
    http::handlers::{health_check, register_end_user},
    state::AppState,
};

mod auth;
mod config;
mod database;
mod http;
mod state;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Install global tracing subscriber configured based on RUST_LOG env var
    tracing_subscriber::fmt()
        .with_env_filter(match EnvFilter::try_from_default_env() {
            Ok(filter) => filter,
            Err(error) => {
                eprintln!(
                    "Invalid or missing RUST_LOG ({error}), falling back to info logging level"
                );
                EnvFilter::new("info")
            }
        })
        .init();

    let config_manager = ConfigManager::init();
    let app_state = AppState::new(&config_manager.database_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        // Healthcheck endpoints
        .route("/health", get(health_check))
        .route("/api/users", post(register_end_user))
        .with_state(app_state);

    // The 0.0.0.0 address means the server accepts connections from any IP address, not just localhost.
    let address = format!("0.0.0.0:{}", config_manager.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    info!("Server is running on {}", address);
    axum::serve(listener, app).await.unwrap();
}
