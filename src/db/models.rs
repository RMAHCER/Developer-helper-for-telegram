// Database models - структуры данных для работы с БД
//
// Модели соответствуют таблицам в базе данных
// Используют derive(sqlx::FromRow) для автоматического маппинга

use crate::shared::types::{ConversionStatus, DbId, Priority, TelegramUserId, Timestamp, TodoStatus};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Модель пользователя
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: DbId,
    pub telegram_id: TelegramUserId,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub language_code: String,
    pub created_at: Timestamp,
    pub last_active_at: Timestamp,
}

/// Данные для создания нового пользователя
#[derive(Debug, Clone)]
pub struct NewUser {
    pub telegram_id: TelegramUserId,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub language_code: String,
}

impl User {
    /// Обновить время последней активности
    pub fn touch(&mut self) {
        self.last_active_at = Utc::now();
    }
}

/// Модель задачи (Todo)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: DbId,
    pub user_id: DbId,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub priority: Priority,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

/// Данные для создания новой задачи
#[derive(Debug, Clone)]
pub struct NewTodo {
    pub user_id: DbId,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
}

/// Данные для обновления задачи
#[derive(Debug, Clone, Default)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TodoStatus>,
    pub priority: Option<Priority>,
}

impl Todo {
    /// Проверка, завершена ли задача
    pub fn is_completed(&self) -> bool {
        self.status == TodoStatus::Completed
    }

    /// Получить эмодзи для статуса
    pub fn status_emoji(&self) -> &'static str {
        match self.status {
            TodoStatus::Pending => "⏳",
            TodoStatus::InProgress => "🔄",
            TodoStatus::Completed => "✅",
            TodoStatus::Cancelled => "❌",
        }
    }

    /// Получить эмодзи для приоритета
    pub fn priority_emoji(&self) -> &'static str {
        match self.priority {
            1 => "🔴", // Highest
            2 => "🟠",
            3 => "🟡",
            4 => "🟢",
            5 => "⚪", // Lowest
            _ => "⚪",
        }
    }
}

/// Модель напоминания
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reminder {
    pub id: DbId,
    pub todo_id: Option<DbId>,
    pub user_id: DbId,
    pub remind_at: Timestamp,
    pub message: Option<String>,
    pub is_sent: bool,
    pub sent_at: Option<Timestamp>,
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>,
    pub created_at: Timestamp,
}

/// Данные для создания нового напоминания
#[derive(Debug, Clone)]
pub struct NewReminder {
    pub user_id: DbId,
    pub todo_id: Option<DbId>,
    pub remind_at: Timestamp,
    pub message: Option<String>,
    pub is_recurring: bool,
    pub recurrence_pattern: Option<String>,
}

impl Reminder {
    /// Проверка, пора ли отправлять напоминание
    pub fn should_send(&self) -> bool {
        !self.is_sent && self.remind_at <= Utc::now()
    }
}

/// Модель конвертации файла
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FileConversion {
    pub id: DbId,
    pub user_id: DbId,
    pub source_file_id: Option<String>,
    pub source_format: Option<String>,
    pub target_format: Option<String>,
    pub status: Option<String>,
    pub result_file_path: Option<String>,
    pub error_message: Option<String>,
    pub created_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

/// Данные для создания новой конвертации
#[derive(Debug, Clone)]
pub struct NewFileConversion {
    pub user_id: DbId,
    pub source_file_id: String,
    pub source_format: String,
    pub target_format: String,
}

impl FileConversion {
    /// Получить статус как enum
    pub fn get_status(&self) -> ConversionStatus {
        match self.status.as_deref() {
            Some("pending") => ConversionStatus::Pending,
            Some("processing") => ConversionStatus::Processing,
            Some("completed") => ConversionStatus::Completed,
            Some("failed") => ConversionStatus::Failed,
            _ => ConversionStatus::Pending,
        }
    }
}
