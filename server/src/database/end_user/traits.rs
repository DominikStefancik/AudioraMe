use async_trait::async_trait;
use uuid::Uuid;

use crate::database::end_user::models::EndUser;

// The trait bound "Send" means the repository can be moved between threads.
// The trait bound "Sync" means multiple threads can share references to the repository.
#[async_trait]
pub trait EndUserRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<EndUser, sqlx::Error>;

    async fn update(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        bio: Option<&str>,
        image: Option<&str>,
    ) -> Result<Option<EndUser>, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<EndUser>, sqlx::Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<EndUser>, sqlx::Error>;

    async fn find_by_username(&self, username: &str) -> Result<Option<EndUser>, sqlx::Error>;

    async fn update_password(
        &self,
        user_id: Uuid,
        new_password_hash: &str,
    ) -> Result<(), sqlx::Error>;
}
