use axum::{Router, routing::get};

use crate::{
    http::handlers::{auth_router, health_check, users_router},
    state::AppState,
};

pub fn create_app_router(app_state: AppState) -> Router {
    Router::new()
        // Healthcheck endpoint
        .route("/health", get(health_check))
        .nest("/api/auth", auth_router())
        .nest("/api/users", users_router())
        .with_state(app_state)
}
