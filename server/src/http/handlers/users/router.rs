use axum::{
    Router,
    routing::{get, put},
};

use crate::{
    http::handlers::users::{get_current_end_user, update_end_user},
    state::AppState,
};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/current", get(get_current_end_user))
        .route("/user", put(update_end_user))
}
