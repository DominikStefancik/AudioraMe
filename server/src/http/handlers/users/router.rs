use axum::{Router, routing::get};

use crate::{
    http::handlers::users::{get_current_end_user, get_end_user::get_end_user, update_end_user},
    state::AppState,
};

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/current", get(get_current_end_user))
        // we can chain different handlers if the path is the same for them and don't have to call
        // a seperate "route()" method for it
        .route("/user/{id}", get(get_end_user).put(update_end_user))
}
