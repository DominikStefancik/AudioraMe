use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use tracing::{debug, error, info, warn};
use validator::Validate;

use crate::{
    auth::hash_password,
    http::{
        handlers::auth::helpers::generate_verification_token,
        schemas::{
            ForgotPasswordRequest, ForgotPasswordResponse, ResetPasswordRequest,
            ResetPasswordResponse,
        },
    },
    state::AppState,
};

// Handler for "Forgot Password" - generates reset token
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ForgotPasswordResponse>, StatusCode> {
    info!("forgot_password handler started...");

    // Validate input data
    payload.validate().map_err(|error| {
        error!("Error while validating forgot password data: {}", error);
        StatusCode::BAD_REQUEST
    })?;

    // Check if user with a given email exists
    let user = state
        .end_user_repository
        .find_by_email(&payload.email)
        .await
        .map_err(|error| {
            error!("Error while fetching user by email: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // SECURITY: Always return success even if email doesn't exist
    // This prevents attackers from discovering which emails are registered
    if user.is_none() {
        warn!("User with the email doesn't exists: {}", &payload.email);

        return Ok(Json(ForgotPasswordResponse {
            message: "If that email exists, a password reset token has been sent.".to_string(),
            token: None,
        }));
    }

    let user = user.unwrap();

    // Generate reset token
    let reset_token = generate_verification_token();
    let expires_at = Utc::now() + Duration::hours(1); // only 1 hour for expiration

    // Save token to database, so we can verify it later
    state
        .password_reset_token_repository
        .create_token(user.id, &reset_token, expires_at)
        .await
        .map_err(|error| {
            error!("Failed to create a reset token: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Password reset token successfully created.");

    Ok(Json(ForgotPasswordResponse {
        message: "If that email exists, a password reset token has been sent.".to_string(),
        token: Some(reset_token),
    }))
}

// Handler for actually resetting the password
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<Json<ResetPasswordResponse>, StatusCode> {
    info!("reset_password handler started...");

    // Validate new password
    payload.validate().map_err(|error| {
        error!("Error while validating reset password data: {}", error);
        StatusCode::BAD_REQUEST
    })?;

    // Look up the token in the database
    let reset_token = state
        .password_reset_token_repository
        .find_by_token(&payload.token)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching a reset password token from database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    if reset_token.is_expired() {
        debug!(
            "The token is expired. Deleting it from database: {}",
            &payload.token
        );

        // Clean up expired token
        state
            .password_reset_token_repository
            .delete_token(&payload.token)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Err(StatusCode::GONE);
    }

    // Hash the password
    let new_password_hash = hash_password(&payload.new_password).map_err(|error| {
        error!("Error while hashing a password: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Update user password
    state
        .end_user_repository
        .update_password(reset_token.end_user_id, &new_password_hash)
        .await
        .map_err(|error| {
            error!("Error while updating user password_hash: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Delete ALL password reset tokens for this user (invalidate any other pending requests)
    state
        .password_reset_token_repository
        .delete_all_user_tokens(reset_token.end_user_id)
        .await
        .map_err(|error| {
            error!("Error while deleting all reset passwords: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!("Password has been successfully reset.");

    Ok(Json(ResetPasswordResponse {
        message: "Password has been successfully reset! You can now login with your new password."
            .to_string(),
    }))
}
