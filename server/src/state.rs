use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::{PgPool, Pool, Postgres};
use tracing::{debug, info};

use crate::database::{EndUserRepository, EndUserRepositoryTrait};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: Pool<Postgres>,
    /* "dyn" means “dynamic dispatch”
     * At compile time, the Rust compiler doesn't know which implementation of the
     * user repository will be used.
     */
    pub end_user_repository: Arc<dyn EndUserRepositoryTrait>,
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(database_url).await?;
        debug!("Connected to the database successfully");

        // Run migrations automatically
        sqlx::migrate!("../migrations").run(&db).await?;
        info!("Ran migrations successfully");

        /*
         * Axum clones AppState for every request, but with Arc,
         * all those clones point to the same repository instance in memory.
         * We’re not creating multiple database connections, we’re sharing one.
         */
        let end_user_repository: Arc<dyn EndUserRepositoryTrait> =
            Arc::new(EndUserRepository::new(db.clone()));

        Ok(AppState {
            db,
            end_user_repository,
        })
    }
}
