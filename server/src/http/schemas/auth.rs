use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::EndUser;

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
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 64,
        message = "Password must be between 8 and 64 characters"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: EndUserData,
}

#[derive(Debug, Serialize)]
pub struct EndUserData {
    pub username: String,
    pub email: String,
    pub bio: String,
    pub image: Option<String>,
}

impl EndUserData {
    pub fn from_end_user(end_user: EndUser) -> Self {
        Self {
            username: end_user.username,
            email: end_user.email,
            bio: end_user.bio.unwrap_or_default(), // Empty string if None
            image: end_user.image,
        }
    }
}
