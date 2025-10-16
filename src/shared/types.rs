// Shared types - common data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Telegram user ID
pub type TelegramUserId = i64;

/// Database record ID
pub type DbId = i32;

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl ToString for TodoStatus {
    fn to_string(&self) -> String {
        match self {
            TodoStatus::Pending => "pending".to_string(),
            TodoStatus::InProgress => "in_progress".to_string(),
            TodoStatus::Completed => "completed".to_string(),
            TodoStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl std::str::FromStr for TodoStatus {
    type Err = crate::error::AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(TodoStatus::Pending),
            "in_progress" | "inprogress" => Ok(TodoStatus::InProgress),
            "completed" | "done" => Ok(TodoStatus::Completed),
            "cancelled" | "canceled" => Ok(TodoStatus::Cancelled),
            _ => Err(crate::error::validation_error(format!(
                "Invalid todo status: {}",
                s
            ))),
        }
    }
}

/// Task priority (1 - highest, 5 - lowest)
pub type Priority = i32;

/// Timestamp
pub type Timestamp = DateTime<Utc>;

/// File conversion status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ConversionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl ToString for ConversionStatus {
    fn to_string(&self) -> String {
        match self {
            ConversionStatus::Pending => "pending".to_string(),
            ConversionStatus::Processing => "processing".to_string(),
            ConversionStatus::Completed => "completed".to_string(),
            ConversionStatus::Failed => "failed".to_string(),
        }
    }
}

/// Reminder recurrence pattern
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrencePattern {
    Daily,
    Weekly,
    Monthly,
    Custom(String),
}

impl ToString for RecurrencePattern {
    fn to_string(&self) -> String {
        match self {
            RecurrencePattern::Daily => "daily".to_string(),
            RecurrencePattern::Weekly => "weekly".to_string(),
            RecurrencePattern::Monthly => "monthly".to_string(),
            RecurrencePattern::Custom(s) => s.clone(),
        }
    }
}

impl std::str::FromStr for RecurrencePattern {
    type Err = crate::error::AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" => Ok(RecurrencePattern::Daily),
            "weekly" => Ok(RecurrencePattern::Weekly),
            "monthly" => Ok(RecurrencePattern::Monthly),
            _ => Ok(RecurrencePattern::Custom(s.to_string())),
        }
    }
}
