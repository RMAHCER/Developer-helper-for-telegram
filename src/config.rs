// Configuration - application configuration management
//
// Configuration is loaded from:
// 1. config/default.toml file (defaults)
// 2. Environment variables (.env file)
// 3. System environment variables (priority)

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Telegram bot configuration
    pub telegram: TelegramConfig,

    /// Database configuration
    pub database: DatabaseConfig,

    /// Application configuration
    pub app: AppConfig,

    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Telegram bot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    /// Bot token from @BotFather
    pub bot_token: String,

    /// Maximum number of concurrent handlers
    #[serde(default = "default_max_handlers")]
    pub max_handlers: usize,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL (example: postgres://user:pass@localhost/db)
    pub url: String,

    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Minimum number of connections in the pool
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// Automatic migration execution on startup
    #[serde(default = "default_auto_migrate")]
    pub auto_migrate: bool,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name
    #[serde(default = "default_app_name")]
    pub name: String,

    /// Environment (development, production)
    #[serde(default = "default_environment")]
    pub environment: String,

    /// Port for health checks (optional)
    #[serde(default)]
    pub port: Option<u16>,

    /// Directory for temporary files
    #[serde(default = "default_temp_dir")]
    pub temp_dir: String,

    /// Directory for converted files
    #[serde(default = "default_output_dir")]
    pub output_dir: String,

    /// Maximum uploaded file size (in bytes)
    #[serde(default = "default_max_file_size")]
    pub max_file_size: usize,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Logging level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log format (json, pretty)
    #[serde(default = "default_log_format")]
    pub format: String,
}

// Default values
fn default_max_handlers() -> usize { 100 }
fn default_max_connections() -> u32 { 10 }
fn default_min_connections() -> u32 { 2 }
fn default_auto_migrate() -> bool { true }
fn default_app_name() -> String { "telegram-multitool-bot".to_string() }
fn default_environment() -> String { "development".to_string() }
fn default_temp_dir() -> String { "./tmp".to_string() }
fn default_output_dir() -> String { "./converted".to_string() }
fn default_max_file_size() -> usize { 20 * 1024 * 1024 } // 20 MB
fn default_log_level() -> String { "info".to_string() }
fn default_log_format() -> String { "pretty".to_string() }

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Load .env file (if exists)
        dotenv::dotenv().ok();

        let telegram = TelegramConfig {
            bot_token: env::var("BOT_TOKEN")
                .map_err(|_| AppError::Config("BOT_TOKEN not set".to_string()))?,
            max_handlers: env::var("MAX_HANDLERS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(default_max_handlers),
        };

        let database = DatabaseConfig {
            url: env::var("DATABASE_URL")
                .map_err(|_| AppError::Config("DATABASE_URL not set".to_string()))?,
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(default_max_connections),
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(default_min_connections),
            auto_migrate: env::var("DB_AUTO_MIGRATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(default_auto_migrate),
        };

        let app = AppConfig {
            name: env::var("APP_NAME").unwrap_or_else(|_| default_app_name()),
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| default_environment()),
            port: env::var("PORT").ok().and_then(|v| v.parse().ok()),
            temp_dir: env::var("TEMP_DIR").unwrap_or_else(|_| default_temp_dir()),
            output_dir: env::var("OUTPUT_DIR").unwrap_or_else(|_| default_output_dir()),
            max_file_size: env::var("MAX_FILE_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(default_max_file_size),
        };

        let logging = LoggingConfig {
            level: env::var("LOG_LEVEL").unwrap_or_else(|_| default_log_level()),
            format: env::var("LOG_FORMAT").unwrap_or_else(|_| default_log_format()),
        };

        Ok(Config {
            telegram,
            database,
            app,
            logging,
        })
    }

    /// Check if we are in production environment
    pub fn is_production(&self) -> bool {
        self.app.environment == "production"
    }

    /// Check if we are in development environment
    pub fn is_development(&self) -> bool {
        self.app.environment == "development"
    }
}
