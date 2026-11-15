use axum::{Router, routing::post};

use crate::{http::handlers::register_end_user, state::AppState};

pub fn auth_router() -> Router<AppState> {
    Router::new().route("/register", post(register_end_user))
}
