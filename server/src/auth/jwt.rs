use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // subject (end_user_id)
    pub exp: usize,  // expiration
    pub iat: usize,  // issued at
}

pub fn generate_token(end_user_id: &Uuid, secret: &str, duration: u16) -> Result<String, Error> {
    let now = Utc::now();
    let expiration = (now + Duration::minutes(duration as i64)).timestamp() as usize;
    let issued_at = now.timestamp() as usize;

    let token_claims = TokenClaims {
        sub: end_user_id.to_string(),
        exp: expiration,
        iat: issued_at,
    };

    encode(
        &Header::default(), // The default header uses HS256 algorithm
        &token_claims,
        &EncodingKey::from_secret(secret.as_ref()).unwrap(),
    )
}

pub fn validate_token(token: &str, secret: &str) -> Result<TokenClaims, Error> {
    decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()).unwrap(),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}
