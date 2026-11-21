use axum::{Json, extract::State, http::StatusCode};
use tracing::{debug, error, info};

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

    // Step 2: Check if token has expired
    if refresh_token.is_expired() {
        // Token is expired, delete it and reject
        debug!(
            "The refresh token is expired: id={}. Rejecting refresh token request",
            refresh_token.id
        );

        state
            .refresh_token_repository
            .delete_token(&refresh_token.token)
            .await
            .map_err(|error| {
                error!(
                    "Error while deleting a refresh token from database: {}",
                    error
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        return Err(StatusCode::UNAUTHORIZED);
    }

    // Step 3: REUSE DETECTION - Check if token was already used
    if refresh_token.is_used {
        // SECURITY BREACH DETECTED!
        // Someone is trying to use an old token
        // This means the token has likely been stolen

        error!("TOKEN REUSE DETECTED!");
        error!("Token: {}", &payload.refresh_token);
        error!("User ID: {}", refresh_token.end_user_id);
        error!("Originally used at: {:?}", refresh_token.used_at);

        // Nuclear option: Delete ALL user's refresh tokens
        // Force them to login again
        state
            .refresh_token_repository
            .delete_all_end_user_tokens(refresh_token.end_user_id)
            .await
            .map_err(|error| {
                error!(
                    "Error while deleting all refresh tokens from database: {}",
                    error
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        return Err(StatusCode::UNAUTHORIZED);
    }

    // Step 4: Mark the old token as used (consumed)
    state
        .refresh_token_repository
        .mark_token_as_used(&refresh_token.token)
        .await
        .map_err(|error| {
            error!("Error while marking refresh token as used: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Step 5: Generate NEW refresh token with rotation
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

    // Step 6: Generate new access token
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

    // Step 7: Return BOTH tokens
    Ok(Json(RefreshTokenResponse {
        access_token,
        refresh_token: new_refresh_token,
    }))
}
