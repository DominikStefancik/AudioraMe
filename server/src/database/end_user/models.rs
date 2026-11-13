use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// The "FromRow" derive macro lets SQLx automatically convert database rows into the EndUser struct
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct EndUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
