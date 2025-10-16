// Reminder notifier - отправка уведомлений пользователям

use crate::error::Result;
use crate::reminder::models::Reminder;
use teloxide::prelude::*;
use teloxide::types::ChatId;

/// Отправитель напоминаний
pub struct ReminderNotifier {
    bot: Bot,
}

impl ReminderNotifier {
    pub fn new(bot: Bot) -> Self {
        Self { bot }
    }

    /// Отправить напоминание пользователю
    pub async fn send_reminder(&self, reminder: &Reminder) -> Result<()> {
        let chat_id = ChatId(reminder.user_id as i64);

        let message = self.format_reminder_message(reminder);

        self.bot
            .send_message(chat_id, message)
            .await
            .map_err(|e| {
                tracing::error!("Failed to send reminder to user {}: {}", reminder.user_id, e);
                crate::error::AppError::Telegram(e.to_string())
            })?;

        Ok(())
    }

    /// Форматировать сообщение напоминания
    fn format_reminder_message(&self, reminder: &Reminder) -> String {
        let mut message = String::from("🔔 *Reminder!*\n\n");

        if let Some(ref msg) = reminder.message {
            message.push_str(msg);
        } else {
            message.push_str("You have a reminder!");
        }

        if reminder.todo_id.is_some() {
            message.push_str(&format!("\n\n📝 Related to task #{}", reminder.todo_id.unwrap()));
        }

        if reminder.is_recurring {
            message.push_str("\n\n🔄 This is a recurring reminder");
        }

        message
    }
}
