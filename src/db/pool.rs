// Database pool - управление пулом соединений с PostgreSQL
//
// Использует SQLx для асинхронной работы с БД
// Пул создаётся один раз при старте приложения

use crate::config::DatabaseConfig;
use crate::error::{AppError, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Создание пула соединений с базой данных
///
/// # Arguments
/// * `config` - конфигурация базы данных
///
/// # Returns
/// Пул соединений или ошибка
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    tracing::info!("Creating database pool...");
    tracing::debug!("Database URL: {}", mask_db_url(&config.url));

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600)) // 10 минут
        .max_lifetime(Duration::from_secs(1800)) // 30 минут
        .connect(&config.url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to database: {}", e);
            AppError::Database(e)
        })?;

    tracing::info!(
        "Database pool created successfully (max: {}, min: {})",
        config.max_connections,
        config.min_connections
    );

    Ok(pool)
}

/// Проверка соединения с базой данных
pub async fn check_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Database health check failed: {}", e);
            AppError::Database(e)
        })?;

    tracing::debug!("Database health check passed");
    Ok(())
}

/// Маскирование пароля в URL базы данных для логирования
fn mask_db_url(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            let mut masked = url.to_string();
            masked.replace_range(colon_pos + 1..at_pos, "****");
            return masked;
        }
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_db_url() {
        let url = "postgres://user:password@localhost/db";
        let masked = mask_db_url(url);
        assert!(masked.contains("****"));
        assert!(!masked.contains("password"));
    }
}
