use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::database::password_reset_token::models::PasswordResetToken;

// The trait bound "Send" means the repository can be moved between threads.
// The trait bound "Sync" means multiple threads can share references to the repository.
#[async_trait]
pub trait PasswordResetTokenRepositoryTrait: Send + Sync {
    async fn create_token(
        &self,
        end_user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<PasswordResetToken, sqlx::Error>;

    async fn find_by_token(&self, token: &str) -> Result<Option<PasswordResetToken>, sqlx::Error>;

    async fn delete_token(&self, token: &str) -> Result<(), sqlx::Error>;

    /*
     * When a user successfully resets their password, we invalidate ALL their reset tokens.
     * If someone requested multiple resets, only one should work.
     */
    async fn delete_all_user_tokens(&self, user_id: Uuid) -> Result<(), sqlx::Error>;
}
