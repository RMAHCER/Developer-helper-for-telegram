// Bot module - обработка команд и взаимодействие с Telegram API
pub mod handlers;
pub mod commands;
pub mod callbacks;
pub mod keyboards;
pub mod state;

// Re-export для удобного использования
pub use handlers::schema;
pub use state::State;
