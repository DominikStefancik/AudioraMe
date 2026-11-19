use axum::{Router, routing::get};

use crate::{http::handlers::users::get_current_end_user, state::AppState};

pub fn users_router() -> Router<AppState> {
    Router::new().route("/current", get(get_current_end_user))
}
