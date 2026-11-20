use axum::{Json, extract::State, http::StatusCode};
use tracing::{debug, error, info};
use validator::Validate;

use crate::{
    auth::{generate_refresh_token, generate_token, hash_password},
    http::schemas::{EndUserData, LoginResponse, RegisterEndUserRequest},
    state::AppState,
};

pub async fn register_end_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterEndUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    info!("register_end_user handler started...");

    // Validate input data
    payload.user.validate().map_err(|error| {
        error!("Error while validating end user data: {}", error);
        StatusCode::BAD_REQUEST
    })?;

    // Check if user with a given username already exists
    if state
        .end_user_repository
        .find_by_username(&payload.user.username)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by username from the database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some()
    {
        debug!(
            "End user with the username {} already exists",
            &payload.user.username
        );
        return Err(StatusCode::CONFLICT);
    }

    // Check if user with a given email already exists
    if state
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
        .is_some()
    {
        debug!(
            "End user with the email {} already exists",
            &payload.user.email
        );
        return Err(StatusCode::CONFLICT);
    }

    // Hash the password
    let password_hash = hash_password(&payload.user.password).map_err(|error| {
        error!("Error while hashing a password: {}", error);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Create user in the database
    let end_user = state
        .end_user_repository
        .create(&payload.user.username, &payload.user.email, &password_hash)
        .await
        .map_err(|error| {
            error!("Error while crearting a new user: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Generate JWT token
    let access_token = generate_token(
        &end_user.id,
        &state.config_manager.jwt_secret,
        state.config_manager.jwt_max_duration_minutes,
    )
    .map_err(|error| {
        error!("Error while generating an access token: {}", error);
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

    // Build response
    let response = LoginResponse {
        user: EndUserData::from_end_user(end_user),
        access_token,
        refresh_token,
    };

    info!("End user successfully registered");

    Ok(Json(response))
}
