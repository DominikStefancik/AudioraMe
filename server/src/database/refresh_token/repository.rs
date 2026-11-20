use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::database::refresh_token::{RefreshTokenRepositoryTrait, models::RefreshToken};

#[derive(Clone)]
pub struct RefreshTokenRepository {
    pub db: PgPool,
}

impl RefreshTokenRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RefreshTokenRepositoryTrait for RefreshTokenRepository {
    async fn create_token(
        &self,
        end_user_id: Uuid,
        token: &str,
    ) -> Result<RefreshToken, sqlx::Error> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_token (end_user_id, token)
            VALUES ($1, $2)
            RETURNING id, end_user_id, token, created_at, last_used_at
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(end_user_id)
        .bind(token)
        .fetch_one(&self.db)
        .await?;

        Ok(refresh_token)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<RefreshToken>, sqlx::Error> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT id, end_user_id, token, created_at, last_used_at
            FROM refresh_token
            WHERE token = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(token)
        .fetch_optional(&self.db)
        .await?;

        Ok(refresh_token)
    }

    /*
     * Every time someone uses their refresh token to get a new access token, we’ll update this timestamp.
     * This creates an audit trail that helps us detect suspicious activity,
     * like a token being used from multiple locations simultaneously.
     */
    async fn update_last_used(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE refresh_token
            SET last_used_at = $2
            WHERE token = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(token)
        .bind(Utc::now())
        .fetch_optional(&self.db)
        .await?;

        Ok(())
    }

    async fn delete_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM refresh_token
            WHERE token = $1
            "#,
        )
        // The "bind()" method safely inserts our parameters, preventing SQL injection attacks.
        .bind(token)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn delete_all_end_user_tokens(&self, end_user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM refresh_token
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
