// Database migrations - SQL migration execution
//
// SQLx supports embedded migrations via sqlx::migrate!()
// Migrations run automatically on startup (if enabled in config)

use crate::error::{AppError, Result};
use sqlx::PgPool;

/// Run database migrations
///
/// Migrations are located in ./migrations/ folder
/// SQLx automatically tracks which migrations have been applied
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    tracing::info!("Running database migrations...");

    // Embedded migrations (compiled into binary)
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            tracing::error!("Migration failed: {}", e);
            AppError::Database(e.into())
        })?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}

/// Check migration status (for debugging)
pub async fn check_migrations(pool: &PgPool) -> Result<()> {
    tracing::debug!("Checking migration status...");

    // Check that _sqlx_migrations table exists
    let result = sqlx::query(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = '_sqlx_migrations'
        )"
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(_) => {
            tracing::debug!("Migration tracking table exists");
            Ok(())
        }
        Err(e) => {
            tracing::warn!("Migration tracking table not found: {}", e);
            Err(AppError::Database(e))
        }
    }
}
