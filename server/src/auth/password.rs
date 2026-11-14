use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn hash_password(password: &str) -> Result<String, String> {
    if password.is_empty() {
        return Err("Password cannot be empty".into());
    }

    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(format!(
            "Password must not be more than {} characters",
            MAX_PASSWORD_LENGTH
        ));
    }

    let params = Params::new(
        64 * 1024, // Memory cost (in KiB blocks)
        3,         // Time cost (iterations)
        1,         // Degree of parallelism (threads)
        None,      // Optional output length
    )
    .map_err(|error| format!("Error while hashing password: {}", error))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let hashed_password = argon2
        .hash_password(password.as_bytes())
        .map_err(|error| format!("Error while hashing password: {}", error))?
        .to_string();

    Ok(hashed_password)
}
