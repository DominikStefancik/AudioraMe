use axum::{Json, http::StatusCode};
use tracing::info;

use crate::http::{
    middleware::RequiredAuth,
    schemas::{AuthenticatedEndUserResponse, EndUserData},
};

pub async fn get_current_end_user(
    /*
     * Axum extractors are functions that run before your handler and extract data from the request.
     * (it runs the method "from_request_parts" from the "FromRequestParts")
     * Our auth extractors check for JWT tokens and give us the authenticated user.
     *
     * Here’s what happens behind the scenes:
     *  1. The extractor extracts the Authorization header Authorization: Token eyJ0eXAiOiJKV1Q...
     *  2. Then parse the token and remove “Token ” prefix and get the JWT string
     *  3. Validate the token and check signature with our secret key
     *  4. Get user ID and extract user ID from token claims
     *  5. Load user from database and find the user by ID
     *  6. If everything works, handler gets the User object
     *  7. If any step fails, the user gets a 401 Unauthorized response before our handler even runs.
     */
    RequiredAuth(user): RequiredAuth,
) -> Result<Json<AuthenticatedEndUserResponse>, StatusCode> {
    info!("get_current_end_user handler started...");

    // Build response
    let end_user_data = EndUserData::from_end_user(user);
    let reponse = AuthenticatedEndUserResponse {
        user: end_user_data,
    };

    info!("Getting current end user was successful.");

    Ok(Json(reponse))
}
