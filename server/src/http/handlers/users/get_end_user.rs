use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{debug, error, info};

use crate::{
    http::{
        middleware::RequiredAuth,
        schemas::{EndUserData, GetEndUserResponse, validate_uuid},
    },
    state::AppState,
};

pub async fn get_end_user(
    RequiredAuth(_): RequiredAuth,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<GetEndUserResponse>, StatusCode> {
    info!("get_end_user handler started...");

    // Validate url parameter (we are expecting UUID format)
    let id = match validate_uuid(&id) {
        Ok(uuid) => uuid,
        Err(error) => {
            error!("Error while validating end user id: {}", error);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let end_user = state
        .end_user_repository
        .find_by_id(id)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by id from the database: {}",
                error
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if end_user.is_none() {
        debug!("End user with the id {} doesn't exists", id.to_string());
        return Err(StatusCode::NOT_FOUND);
    }

    let end_user = end_user.unwrap();

    // Build response
    let end_user_data = EndUserData::from_end_user(end_user);
    let response = GetEndUserResponse {
        user: end_user_data,
    };

    info!("Getting end user was successful. Id = {}", id);

    Ok(Json(response))
}
