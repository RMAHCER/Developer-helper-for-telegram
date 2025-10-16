// Reminder models

pub use crate::db::models::{NewReminder, Reminder};
use serde::{Deserialize, Serialize};

/// DTO for displaying reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderView {
    pub id: i32,
    pub message: String,
    pub remind_at: String,
    pub is_recurring: bool,
    pub todo_title: Option<String>,
}
