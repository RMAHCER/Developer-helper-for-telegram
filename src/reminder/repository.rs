// Reminder repository

use crate::db::models::{NewReminder, Reminder};
use crate::error::{not_found, Result};
use crate::shared::types::{DbId, Timestamp};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ReminderRepository {
    pool: PgPool,
}

impl ReminderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Создать новое напоминание
    pub async fn create(&self, new_reminder: NewReminder) -> Result<Reminder> {
        let reminder = sqlx::query_as::<_, Reminder>(
            r#"
            INSERT INTO reminders (user_id, todo_id, remind_at, message, is_recurring, recurrence_pattern)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(new_reminder.user_id)
        .bind(new_reminder.todo_id)
        .bind(new_reminder.remind_at)
        .bind(&new_reminder.message)
        .bind(new_reminder.is_recurring)
        .bind(&new_reminder.recurrence_pattern)
        .fetch_one(&self.pool)
        .await?;

        tracing::debug!("Created reminder {} for user {}", reminder.id, reminder.user_id);
        Ok(reminder)
    }

    /// Найти напоминание по ID
    pub async fn find_by_id(&self, id: DbId) -> Result<Reminder> {
        let reminder = sqlx::query_as::<_, Reminder>("SELECT * FROM reminders WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| not_found(format!("Reminder {} not found", id)))?;

        Ok(reminder)
    }

    /// Получить все неотправленные напоминания, которые пора отправить
    pub async fn get_pending_reminders(&self, before: Timestamp) -> Result<Vec<Reminder>> {
        let reminders = sqlx::query_as::<_, Reminder>(
            r#"
            SELECT * FROM reminders
            WHERE is_sent = FALSE AND remind_at <= $1
            ORDER BY remind_at ASC
            LIMIT 100
            "#,
        )
        .bind(before)
        .fetch_all(&self.pool)
        .await?;

        tracing::debug!("Found {} pending reminders", reminders.len());
        Ok(reminders)
    }

    /// Отметить напоминание как отправленное
    pub async fn mark_as_sent(&self, id: DbId) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE reminders
            SET is_sent = TRUE, sent_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        tracing::debug!("Marked reminder {} as sent", id);
        Ok(())
    }

    /// Получить все напоминания пользователя
    pub async fn find_by_user(&self, user_id: DbId) -> Result<Vec<Reminder>> {
        let reminders = sqlx::query_as::<_, Reminder>(
            r#"
            SELECT * FROM reminders
            WHERE user_id = $1 AND is_sent = FALSE
            ORDER BY remind_at ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    /// Удалить напоминание
    pub async fn delete(&self, id: DbId) -> Result<()> {
        let result = sqlx::query("DELETE FROM reminders WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(not_found(format!("Reminder {} not found", id)));
        }

        tracing::debug!("Deleted reminder {}", id);
        Ok(())
    }
}
