use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};
use tracing::debug;

use crate::auth::AuthErrorMessage;

pub fn hash_password(password: &str) -> Result<String, AuthErrorMessage> {
    let params = Params::new(
        64 * 1024, // Memory cost (in KiB blocks)
        3,         // Time cost (iterations)
        1,         // Degree of parallelism (threads)
        None,      // Optional output length
    )
    .map_err(|error| AuthErrorMessage::HashingError(error.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let hashed_password = argon2
        .hash_password(password.as_bytes())
        .map_err(|error| AuthErrorMessage::HashingError(error.to_string()))?
        .to_string();

    Ok(hashed_password)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthErrorMessage> {
    let params = Params::new(
        64 * 1024, // Memory cost (in KiB blocks)
        3,         // Time cost (iterations)
        1,         // Degree of parallelism (threads)
        None,      // Optional output length
    )
    .map_err(|error| AuthErrorMessage::VerifyingHashError(error.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    match argon2.verify_password(password.as_bytes(), hash) {
        Ok(()) => Ok(true),
        Err(error) => {
            debug!(
                "{}",
                AuthErrorMessage::VerifyingHashError(error.to_string())
            );
            Ok(false)
        }
    }
}
