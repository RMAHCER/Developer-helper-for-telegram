// Error handling - centralized error handling
//
// Use thiserror to create custom error types
// with automatic Error trait implementation

use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Telegram API errors
    #[error("Telegram error: {0}")]
    Telegram(String),

    /// Business logic errors
    #[error("Business logic error: {0}")]
    BusinessLogic(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// File processing errors
    #[error("File processing error: {0}")]
    FileProcessing(String),

    /// Scheduler errors
    #[error("Scheduler error: {0}")]
    Scheduler(String),

    /// Parse errors
    #[error("Parse error: {0}")]
    Parse(String),

    /// I/O errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result with our error type
pub type Result<T> = std::result::Result<T, AppError>;

/// Conversion from anyhow::Error for convenience
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// Conversion from teloxide::RequestError
impl From<teloxide::RequestError> for AppError {
    fn from(err: teloxide::RequestError) -> Self {
        AppError::Telegram(err.to_string())
    }
}

/// Helper for creating validation errors
pub fn validation_error(msg: impl Into<String>) -> AppError {
    AppError::Validation(msg.into())
}

/// Helper for creating "not found" errors
pub fn not_found(resource: impl Into<String>) -> AppError {
    AppError::NotFound(resource.into())
}
