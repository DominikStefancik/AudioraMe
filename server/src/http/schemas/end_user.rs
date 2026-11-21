use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{database::EndUser, http::schemas::validate_username};

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

#[derive(Debug, Deserialize)]
pub struct UpdateEndUserRequest {
    pub user: UpdateEndUserData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEndUserData {
    #[validate(custom(function = "validate_username"))]
    pub username: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateEndUserResponse {
    pub user: EndUserData,
}
