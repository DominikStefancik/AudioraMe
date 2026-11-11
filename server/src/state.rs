use axum::extract::FromRef;
use sqlx::{PgPool, Pool, Postgres};
use tracing::debug;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(database_url).await?;
        debug!("Connected to the database successfully");

        Ok(AppState { db })
    }
}
