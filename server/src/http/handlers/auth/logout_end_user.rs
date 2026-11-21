use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, info};

use crate::{
    http::schemas::{LogoutRequest, LogoutResponse},
    state::AppState,
};

pub async fn logout_end_user(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, StatusCode> {
    info!("logout_end_user handler started...");

    // Simply delete the refresh token from database
    state
        .refresh_token_repository
        .delete_token(&payload.refresh_token)
        .await
        .map_err(|error| {
            error!("Error while deleting a refresh token: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("End user has been successfully logged out.");

    Ok(Json(LogoutResponse {
        message: "End user has been successfully logged out.".to_string(),
    }))
}
