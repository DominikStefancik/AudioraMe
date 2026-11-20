use uuid::Uuid;

pub fn generate_refresh_token() -> String {
    // Unlike JWTs which contain encoded data and signatures, refresh tokens are just random strings.
    // Generate a random UUID and convert to string
    // This creates a unique, unpredictable token
    Uuid::new_v4().to_string()
}
