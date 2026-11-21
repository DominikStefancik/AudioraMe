use std::sync::Arc;

use tracing::{debug, error};

use crate::database::EndUserRepositoryTrait;

pub async fn is_username_used(
    repository: &Arc<dyn EndUserRepositoryTrait>,
    username: &str,
) -> Result<bool, String> {
    if repository
        .find_by_username(username)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by username from the database: {}",
                error
            );
            "Error while fetching end user by username from the database".to_string()
        })?
        .is_some()
    {
        debug!("End user with the username {} already exists", username);
        return Ok(true);
    }

    Ok(false)
}

pub async fn is_email_used(
    repository: &Arc<dyn EndUserRepositoryTrait>,
    email: &str,
) -> Result<bool, String> {
    if repository
        .find_by_email(email)
        .await
        .map_err(|error| {
            error!(
                "Error while fetching end user by email from the database: {}",
                error
            );
            "Error while fetching end user by email from the database"
        })?
        .is_some()
    {
        debug!("End user with the email {} already exists", email);
        return Ok(true);
    }

    Ok(false)
}
