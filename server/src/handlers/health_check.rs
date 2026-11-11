use axum::{Json, extract::State};
use serde_json::{Value, json};
use tracing::info;

use crate::state::AppState;

/*
 * "State(state): State<AppState>" is Axum’s state extraction mechanism.
 * Axum sees we want AppState and automatically provides it from what we passed to ".with_state()" in main.rs.
 * "state" now contains our shared database connection pool.
 */
pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    info!("health_check handler started...");

    // calling "sqlx::query("SELECT 1").execute(&state.db).await" is a database test using shared pool
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => Json(json!({
            "status": 200,
            "database": "connected",
            "message": "Server is running"
        })),
        Err(error) => {
            eprintln!("Database error: {}", error);
            Json(json!({
                "status": 500,
                "database": "disconnected",
                "message": "Server is not running",
                "error": error.to_string()
            }))
        }
    }
}
