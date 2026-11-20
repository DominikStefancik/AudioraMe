use axum::{Router, routing::post};

use crate::{
    http::handlers::{
        auth::{
            password_reset::{forgot_password, reset_password},
            refresh_token::refresh_access_token,
        },
        login_end_user, register_end_user,
    },
    state::AppState,
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_end_user))
        .route("/login", post(login_end_user))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/refresh-token", post(refresh_access_token))
}
