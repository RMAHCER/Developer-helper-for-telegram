// Database models - data structures for working with DB
//
// Models correspond to database tables
// Use derive(sqlx::FromRow) for automatic mapping

use crate::shared::types::{ConversionStatus, DbId, Priority, TelegramUserId, Timestamp, TodoStatus};
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// User model
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

/// Data for creating a new user
#[derive(Debug, Clone)]
pub struct NewUser {
    pub telegram_id: TelegramUserId,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub language_code: String,
}

impl User {
    /// Update last activity time
    pub fn touch(&mut self) {
        self.last_active_at = Utc::now();
    }
}

/// Task model (Todo)
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

/// Data for creating a new task
#[derive(Debug, Clone)]
pub struct NewTodo {
    pub user_id: DbId,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
}

/// Data for updating a task
#[derive(Debug, Clone, Default)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TodoStatus>,
    pub priority: Option<Priority>,
}

impl Todo {
    /// Check if task is completed
    pub fn is_completed(&self) -> bool {
        self.status == TodoStatus::Completed
    }

    /// Get emoji for status
    pub fn status_emoji(&self) -> &'static str {
        match self.status {
            TodoStatus::Pending => "⏳",
            TodoStatus::InProgress => "🔄",
            TodoStatus::Completed => "✅",
            TodoStatus::Cancelled => "❌",
        }
    }

    /// Get emoji for priority
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

/// Reminder model
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

/// Data for creating a new reminder
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
    /// Check if it's time to send the reminder
    pub fn should_send(&self) -> bool {
        !self.is_sent && self.remind_at <= Utc::now()
    }
}

/// File conversion model
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

/// Data for creating a new conversion
#[derive(Debug, Clone)]
pub struct NewFileConversion {
    pub user_id: DbId,
    pub source_file_id: String,
    pub source_format: String,
    pub target_format: String,
}

impl FileConversion {
    /// Get status as enum
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
