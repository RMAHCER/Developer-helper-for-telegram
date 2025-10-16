// Reminder service

use crate::db::models::NewReminder;
use crate::error::{validation_error, Result};
use crate::reminder::models::Reminder;
use crate::reminder::repository::ReminderRepository;
use crate::shared::types::DbId;
use crate::shared::utils::parse_relative_time;
use chrono::Utc;

#[derive(Clone)]
pub struct ReminderService {
    repo: ReminderRepository,
}

impl ReminderService {
    pub fn new(repo: ReminderRepository) -> Self {
        Self { repo }
    }

    /// Create reminder
    pub async fn create_reminder(
        &self,
        user_id: DbId,
        todo_id: Option<DbId>,
        time_input: &str,
        message: Option<String>,
    ) -> Result<Reminder> {
        // Parse time
        let remind_at = parse_relative_time(time_input)
            .ok_or_else(|| validation_error("Invalid time format. Use: 30m, 2h, 1d"))?;

        if remind_at <= Utc::now() {
            return Err(validation_error("Reminder time must be in the future"));
        }

        // Validation сообщения
        if let Some(ref msg) = message {
            if msg.len() > 500 {
                return Err(validation_error("Message is too long (max 500 chars)"));
            }
        }

        let new_reminder = NewReminder {
            user_id,
            todo_id,
            remind_at,
            message,
            is_recurring: false,
            recurrence_pattern: None,
        };

        self.repo.create(new_reminder).await
    }

    /// Get user reminders
    pub async fn get_user_reminders(&self, user_id: DbId) -> Result<Vec<Reminder>> {
        self.repo.find_by_user(user_id).await
    }

    /// Delete reminder
    pub async fn delete_reminder(&self, id: DbId) -> Result<()> {
        self.repo.delete(id).await
    }

    /// Get pending reminders for scheduler
    pub async fn get_pending_reminders(&self) -> Result<Vec<Reminder>> {
        self.repo.get_pending_reminders(Utc::now()).await
    }

    /// Mark reminder as sent
    pub async fn mark_sent(&self, id: DbId) -> Result<()> {
        self.repo.mark_as_sent(id).await
    }
}
