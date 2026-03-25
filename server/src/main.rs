use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use tracing::info;

#[tokio::main]
async fn main() {
    // Install global tracing subscriber configured based on RUST_LOG envvar
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // Healthcheck endpoints
        .route("/health", get(health_check));

    // The 0.0.0.0 address means the server accepts connections from any IP address, not just localhost.
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server is running on {}", address);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    info!("health_check handler started ...");

    Json(json!({
        "status": "200",
        "message": "Server is running"
    }))
}
