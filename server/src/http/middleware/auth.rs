use axum::{
    extract::{FromRef, FromRequestParts},
    http::{HeaderMap, StatusCode, request::Parts},
};
use tracing::error;
use uuid::Uuid;

// This middleware automatically checks if requests have valid JWT tokens.

use crate::{auth::validate_token, database::EndUser, state::AppState};

// Extractor for protected routes - blocks requests without valid JWT tokens
pub struct RequiredAuth(pub EndUser);

// Extractor for optional auth - allows requests with or without tokens (extracts user if token present)
pub struct OptionalAuth(pub Option<EndUser>);

// The "FromRequestParts" is an Axum trait that lets you extract data from HTTP requests
impl<S> FromRequestParts<S> for RequiredAuth
where
    // this says “whatever type S is, we must be able to extract an AppState from it”
    // or in other words “the state type must contain (or be convertible to) our AppState”
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Extract authorization header
        let headers = &parts.headers;
        let token = extract_token_from_headers(headers).ok_or(StatusCode::UNAUTHORIZED)?;

        // Validate JWT token
        let jwt_secret = std::env::var("JWT_SECRET").map_err(|error| {
            error!(
                "Error while fetching JWT secret from the environment variable: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let claims = validate_token(&token, &jwt_secret).map_err(|error| {
            error!("JWT token invalid: {}", error);
            StatusCode::UNAUTHORIZED
        })?;

        // Get end user from the database
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;
        let user = app_state
            .end_user_repository
            .find_by_id(user_id)
            .await
            .map_err(|error| {
                error!(
                    "Error while fetching user by id from the database: {}",
                    error
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(RequiredAuth(user))
    }
}

impl<S> FromRequestParts<S> for OptionalAuth
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Try to extract Authorization header
        let headers = &parts.headers;
        let token = match extract_token_from_headers(headers) {
            Some(token) => token,
            None => return Ok(OptionalAuth(None)),
        };

        // Try to validate JWT token
        let jwt_secret = std::env::var("JWT_SECRET").map_err(|error| {
            error!(
                "Error while fetching JWT secret from the environment variable: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let claims = match validate_token(&token, &jwt_secret) {
            Ok(claims) => claims,
            Err(_) => return Ok(OptionalAuth(None)),
        };

        // Try to get user from the database
        let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => return Ok(OptionalAuth(None)),
        };
        let user = app_state
            .end_user_repository
            .find_by_id(user_id)
            .await
            .map_err(|error| {
                error!(
                    "Error while fetching user by id from the database: {}",
                    error
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        Ok(OptionalAuth(user))
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers.get("Authorization")?.to_str().ok()?;

    if auth_header.starts_with("Token ") {
        Some(auth_header[6..].to_string())
    } else {
        None
    }
}
