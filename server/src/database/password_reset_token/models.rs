use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// The "FromRow" derive macro lets SQLx automatically convert database rows into the PasswordResetToken struct
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub end_user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl PasswordResetToken {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
