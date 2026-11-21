use regex::Regex;
use uuid::Uuid;
use validator::ValidationError;

pub fn validate_uuid(id: &str) -> Result<Uuid, ValidationError> {
    let uuid = Uuid::parse_str(id);

    if uuid.is_err() {
        return Err(ValidationError::new("User id must be in UUID format"));
    }

    Ok(uuid.unwrap())
}

pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    let regex =
        Regex::new(r"^[a-zA-Z0-9_]+$").expect("Incorrect regex expression for the username");

    if !regex.is_match(username) {
        return Err(ValidationError::new(
            "Username can only contain letters, numbers, and underscores",
        ));
    }

    Ok(())
}
