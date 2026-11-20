use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, info};

use crate::{
    auth::{generate_refresh_token, generate_token},
    http::schemas::{RefreshTokenRequest, RefreshTokenResponse},
    state::AppState,
};

pub async fn refresh_access_token(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, StatusCode> {
    info!("refresh_access_token handler started...");

    // Step 1: Look up the refresh token in the database
    let refresh_token = state
        .refresh_token_repository
        .find_by_token(&payload.refresh_token)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching a refresh token from database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        // This is a security check, the refresh token must exist in our database.
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Step 2: Generate NEW refresh token with rotation
    let new_refresh_token = generate_refresh_token();

    state
        .refresh_token_repository
        .create_token(refresh_token.end_user_id, &new_refresh_token)
        .await
        .map_err(|error| {
            error!("Error while creating new refresh token: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Update last_used_at timestamp
    state
        .refresh_token_repository
        .update_last_used(&refresh_token.token)
        .await
        .map_err(|error| {
            error!("Error while updating refresh token last_used_at: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Step 3: Generate new access token
    let access_token = generate_token(
        &refresh_token.end_user_id,
        &state.config_manager.jwt_secret,
        state.config_manager.jwt_max_duration_minutes,
    )
    .map_err(|error| {
        error!("Error while generating a token: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    info!("Access token successfully refreshed.");

    // Step 4: Return BOTH tokens
    Ok(Json(RefreshTokenResponse {
        access_token,
        refresh_token: new_refresh_token,
    }))
}
