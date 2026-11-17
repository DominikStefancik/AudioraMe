use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::password_reset_token::{
    models::PasswordResetToken, traits::PasswordResetTokenRepositoryTrait,
};

#[derive(Clone)]
pub struct PasswordResetTokenRepository {
    pub db: PgPool,
}

impl PasswordResetTokenRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PasswordResetTokenRepositoryTrait for PasswordResetTokenRepository {
    async fn create_token(
        &self,
        end_user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<PasswordResetToken, sqlx::Error> {
        let password_reset_token = sqlx::query_as::<_, PasswordResetToken>(
            r#"
            INSERT INTO password_reset_token (end_user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, end_user_id, token, created_at, expires_at
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(end_user_id)
        .bind(token)
        .bind(expires_at)
        .fetch_one(&self.db)
        .await?;

        Ok(password_reset_token)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<PasswordResetToken>, sqlx::Error> {
        let password_reset_token = sqlx::query_as::<_, PasswordResetToken>(
            r#"
            SELECT id, end_user_id, token, created_at, expires_at
            FROM password_reset_token
            WHERE token = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(token)
        .fetch_optional(&self.db)
        .await?;

        Ok(password_reset_token)
    }

    async fn delete_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM password_reset_token
            WHERE token = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(token)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn delete_all_user_tokens(&self, end_user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM password_reset_token
            WHERE end_user_id = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(end_user_id)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
