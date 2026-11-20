use axum::{Json, extract::State, http::StatusCode};
use tracing::{debug, error, info};
use validator::Validate;

use crate::{
    auth::{generate_refresh_token, generate_token, verify_password},
    http::schemas::{EndUserData, LoginEndUserRequest, LoginResponse},
    state::AppState,
};

pub async fn login_end_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginEndUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    info!("login_end_user handler started...");

    // Validate input data
    payload.user.validate().map_err(|error| {
        error!("Error while validating end user data: {}", error);
        StatusCode::BAD_REQUEST
    })?;

    let end_user = state
        .end_user_repository
        .find_by_email(&payload.user.email)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by email from the database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify the password
    let is_password_valid = verify_password(&payload.user.password, &end_user.password_hash)
        .map_err(|error| {
            error!("Error while verifying password: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !is_password_valid {
        debug!("Password for the user is invalid: id={}", end_user.id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT token
    // Note: Always generate a fresh token on login (better security than reusing old tokens).
    let access_token = generate_token(
        &end_user.id,
        &state.config_manager.jwt_secret,
        state.config_manager.jwt_max_duration_minutes,
    )
    .map_err(|error| {
        error!("Error while generating a token: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate refresh token (UUID, no expiration)
    let refresh_token = generate_refresh_token();

    state
        .refresh_token_repository
        .create_token(end_user.id, &refresh_token)
        .await
        .map_err(|error| {
            error!("Error while saving refresh token: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Build response with access token
    let reponse = LoginResponse {
        user: EndUserData::from_end_user(end_user),
        access_token,
        refresh_token,
    };

    info!("End user successfully logged in.");

    Ok(Json(reponse))
}
