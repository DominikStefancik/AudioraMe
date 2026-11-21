use async_trait::async_trait;
use uuid::Uuid;

use crate::database::refresh_token::models::RefreshToken;

// The trait bound "Send" means the repository can be moved between threads.
// The trait bound "Sync" means multiple threads can share references to the repository.
#[async_trait]
pub trait RefreshTokenRepositoryTrait: Send + Sync {
    async fn create_token(
        &self,
        end_user_id: Uuid,
        token: &str,
    ) -> Result<RefreshToken, sqlx::Error>;

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, sqlx::Error>;

    async fn update_last_used(&self, token: &str) -> Result<(), sqlx::Error>;

    async fn delete_token(&self, token: &str) -> Result<(), sqlx::Error>;

    async fn delete_all_end_user_tokens(&self, end_user_id: Uuid) -> Result<(), sqlx::Error>;

    async fn mark_token_as_used(&self, token: &str) -> Result<(), sqlx::Error>;
}
