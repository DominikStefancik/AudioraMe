use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::http::schemas::{EndUserData, validate_username};

#[derive(Debug, Deserialize)]
pub struct RegisterEndUserRequest {
    pub user: RegisterEndUserData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterEndUserData {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Username must be between 3 and 255 characters"
    ))]
    #[validate(custom(function = "validate_username"))]
    pub username: String,

    #[validate(
        length(min = 1, message = "Email must be provided"),
        email(message = "Invalid email format")
    )]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 64,
        message = "Password must be between 8 and 64 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginEndUserRequest {
    pub user: LoginEndUserData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginEndUserData {
    #[validate(
        length(min = 1, message = "Email must be provided"),
        email(message = "Invalid email format")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "Password must be provided"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: EndUserData,
    // token is always included (client needs it for future requests)
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthenticatedEndUserResponse {
    pub user: EndUserData,
}
