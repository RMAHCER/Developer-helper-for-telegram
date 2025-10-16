// Error handling - централизованная обработка ошибок
//
// Используем thiserror для создания кастомных типов ошибок
// с автоматической реализацией trait Error

use thiserror::Error;

/// Основной тип ошибок приложения
#[derive(Error, Debug)]
pub enum AppError {
    /// Ошибки конфигурации
    #[error("Configuration error: {0}")]
    Config(String),

    /// Ошибки базы данных
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Ошибки Telegram API
    #[error("Telegram error: {0}")]
    Telegram(String),

    /// Ошибки бизнес-логики
    #[error("Business logic error: {0}")]
    BusinessLogic(String),

    /// Ошибки валидации
    #[error("Validation error: {0}")]
    Validation(String),

    /// Ошибки обработки файлов
    #[error("File processing error: {0}")]
    FileProcessing(String),

    /// Ошибки планировщика
    #[error("Scheduler error: {0}")]
    Scheduler(String),

    /// Ошибки парсинга
    #[error("Parse error: {0}")]
    Parse(String),

    /// Ошибки ввода-вывода
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Не найдено
    #[error("Not found: {0}")]
    NotFound(String),

    /// Общая ошибка
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Результат с нашим типом ошибки
pub type Result<T> = std::result::Result<T, AppError>;

/// Конвертация из anyhow::Error для удобства
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// Конвертация из teloxide::RequestError
impl From<teloxide::RequestError> for AppError {
    fn from(err: teloxide::RequestError) -> Self {
        AppError::Telegram(err.to_string())
    }
}

/// Хелпер для создания ошибок валидации
pub fn validation_error(msg: impl Into<String>) -> AppError {
    AppError::Validation(msg.into())
}

/// Хелпер для создания ошибок "не найдено"
pub fn not_found(resource: impl Into<String>) -> AppError {
    AppError::NotFound(resource.into())
}
