use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::end_user::{models::EndUser, traits::EndUserRepositoryTrait};

#[derive(Clone)]
pub struct EndUserRepository {
    db: PgPool,
}

impl EndUserRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl EndUserRepositoryTrait for EndUserRepository {
    async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<EndUser, sqlx::Error> {
        let user = sqlx::query_as::<_, EndUser>(
            r#"
            INSERT INTO end_user (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, bio, image, created_at, updated_at
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    async fn update(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        bio: Option<&str>,
        image: Option<&str>,
    ) -> Result<Option<EndUser>, sqlx::Error> {
        // COALESCE is PostgreSQL’s function
        // If we pass Some("new_value"), it updates that field. If we pass None, it keeps the existing value.
        let user = sqlx::query_as::<_, EndUser>(
            r#"
            UPDATE end_user
            SET username = COALESCE($2, username),
                email = COALESCE($3, email),
                bio = COALESCE($4, bio),
                image = COALESCE($5, image)
            WHERE id = $1
            RETURNING id, username, email, password_hash, bio, image, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(bio)
        .bind(image)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<EndUser>, sqlx::Error> {
        let user = sqlx::query_as::<_, EndUser>(
            r#"
            SELECT id, username, email, password_hash, bio, image, created_at, updated_at
            FROM end_user
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<EndUser>, sqlx::Error> {
        let user = sqlx::query_as::<_, EndUser>(
            r#"
            SELECT id, username, email, password_hash, bio, image, created_at, updated_at
            FROM end_user
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<EndUser>, sqlx::Error> {
        let user = sqlx::query_as::<_, EndUser>(
            r#"
            SELECT id, username, email, password_hash, bio, image, created_at, updated_at
            FROM end_user
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn update_password(
        &self,
        user_id: Uuid,
        new_password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE end_user
            SET password_hash = $2
            WHERE id = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(user_id)
        .bind(new_password_hash)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
