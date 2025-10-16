// Database migrations - запуск SQL миграций
//
// SQLx поддерживает встроенные миграции через sqlx::migrate!()
// Миграции запускаются автоматически при старте (если включено в конфиге)

use crate::error::{AppError, Result};
use sqlx::PgPool;

/// Запуск миграций базы данных
///
/// Миграции находятся в папке ./migrations/
/// SQLx автоматически отслеживает, какие миграции уже применены
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    tracing::info!("Running database migrations...");

    // Встроенные миграции (компилируются в бинарник)
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

/// Проверка состояния миграций (для отладки)
pub async fn check_migrations(pool: &PgPool) -> Result<()> {
    tracing::debug!("Checking migration status...");

    // Проверяем, что таблица _sqlx_migrations существует
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
