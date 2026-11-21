use axum::{Json, extract::State, http::StatusCode};
use tracing::{debug, error, info};
use uuid::Uuid;
use validator::Validate;

use crate::{
    http::{
        handlers::helpers::{is_email_used, is_username_used},
        middleware::RequiredAuth,
        schemas::{EndUserData, UpdateEndUserData, UpdateEndUserRequest, UpdateEndUserResponse},
    },
    state::AppState,
};

pub async fn update_end_user(
    RequiredAuth(_): RequiredAuth,
    State(state): State<AppState>,
    Json(payload): Json<UpdateEndUserRequest>,
) -> Result<Json<UpdateEndUserResponse>, StatusCode> {
    info!("update_end_user handler started...");

    // Validate input data
    payload.user.validate().map_err(|error| {
        error!("Error while validating end user data: {}", error);
        StatusCode::BAD_REQUEST
    })?;

    let end_user_id = Uuid::parse_str(&payload.user.id).unwrap();
    let end_user = state
        .end_user_repository
        .find_by_id(end_user_id)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by id from the database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if end_user.is_none() {
        debug!(
            "End user with the id {} doesn't exists",
            payload.user.id.to_string()
        );
        return Err(StatusCode::NOT_FOUND);
    }

    let end_user = end_user.unwrap();

    // Check if another user with a given username already exists
    if let Some(payload_username) = &payload.user.username {
        let is_used = is_username_used(
            &state.end_user_repository,
            payload.user.username.as_deref().unwrap(),
        )
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by usernamae from the database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // the username might have been found in the database, but the username provided in the payload
        // can be the same, which means a user didn't change it
        if is_used && &end_user.username != payload_username {
            return Err(StatusCode::CONFLICT);
        }
    }

    // Check if another user with a given email already exists
    if let Some(payload_email) = &payload.user.email {
        let is_used = is_email_used(
            &state.end_user_repository,
            payload.user.email.as_deref().unwrap(),
        )
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by email from the database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // the email might have been found in the database, but the email provided in the payload
        // can be the same, which means a user didn't change it
        if is_used && &end_user.email != payload_email {
            return Err(StatusCode::CONFLICT);
        }
    }

    let UpdateEndUserData {
        id,
        username,
        email,
        bio,
        image,
    } = payload.user;

    let end_user = state
        .end_user_repository
        .update(
            end_user_id,
            username.as_deref(),
            email.as_deref(),
            bio.as_deref(),
            image.as_deref(),
        )
        .await
        .map_err(|error| {
            error!("Error while updating end user in the database: {}", error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Build response
    let end_user_data = EndUserData::from_end_user(end_user);
    let response = UpdateEndUserResponse {
        user: end_user_data,
    };

    info!("Updating end user was successful. Id = {}", id);

    Ok(Json(response))
}
