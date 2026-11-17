use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use tracing::{error, info, warn};
use validator::Validate;

use crate::{
    http::{
        handlers::auth::helpers::generate_verification_token,
        schemas::{ForgotPasswordRequest, ForgotPasswordResponse},
    },
    state::AppState,
};

// Handler for "Forgot Password" - generates and emails reset token
pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<Json<ForgotPasswordResponse>, StatusCode> {
    info!("forgot_password handler started ...");

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

    info!("Password successfully reset.");

    Ok(Json(ForgotPasswordResponse {
        message: "If that email exists, a password reset token has been sent.".to_string(),
        token: Some(reset_token),
    }))
}
