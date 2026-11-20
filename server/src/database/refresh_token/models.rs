use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// The "FromRow" derive macro lets SQLx automatically convert database rows into the RefreshToken struct
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub end_user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}
