use axum::{Router, routing::post};

use crate::{
    http::handlers::{
        auth::password_reset::{forgot_password, reset_password},
        register_end_user,
    },
    state::AppState,
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_end_user))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}
