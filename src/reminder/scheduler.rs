// Reminder scheduler - reminder scheduler using Tokio tasks
//
// Architecture:
// 1. Background task checks for new reminders every 30 сек
// 2. For each pending reminder, a Tokio task with delay is created
// 3. When time comes, task sends notification
//
// Scales up to ~10K concurrent reminders

use crate::error::Result;
use crate::reminder::notifier::ReminderNotifier;
use crate::reminder::repository::ReminderRepository;
use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;
use teloxide::Bot;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

/// Reminder scheduler
pub struct ReminderScheduler {
    pool: PgPool,
    bot: Bot,
    scheduled_ids: Arc<Mutex<HashSet<i32>>>, // Track scheduled reminders
}

impl ReminderScheduler {
    pub fn new(pool: PgPool, bot: Bot) -> Self {
        Self {
            pool,
            bot,
            scheduled_ids: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Start scheduler (background task)
    pub async fn run(self) -> Result<()> {
        tracing::info!("Starting reminder scheduler...");

        loop {
            if let Err(e) = self.check_and_schedule().await {
                tracing::error!("Scheduler error: {}", e);
            }

            // Check every 30 секунд
            sleep(Duration::from_secs(30)).await;
        }
    }

    /// Check and schedule new reminders
    async fn check_and_schedule(&self) -> Result<()> {
        let repo = ReminderRepository::new(self.pool.clone());
        let reminders = repo.get_pending_reminders(Utc::now()).await?;

        let mut scheduled = self.scheduled_ids.lock().await;

        for reminder in reminders {
            // Skip already scheduled
            if scheduled.contains(&reminder.id) {
                continue;
            }

            // Check if it's time to send now
            let now = Utc::now();
            if reminder.remind_at <= now {
                self.send_reminder_now(reminder.clone()).await;
            } else {
                // Schedule for future
                self.schedule_reminder(reminder.clone()).await;
            }

            scheduled.insert(reminder.id);
        }

        tracing::debug!("Scheduler check completed. Total scheduled: {}", scheduled.len());

        // Clear old IDs (optional, to prevent infinite growth)
        if scheduled.len() > 100_000 {
            scheduled.clear();
            tracing::warn!("Cleared scheduled IDs cache (reached 100K)");
        }

        Ok(())
    }

    /// Schedule reminder for future
    async fn schedule_reminder(&self, reminder: crate::db::models::Reminder) {
        let bot = self.bot.clone();
        let pool = self.pool.clone();
        let scheduled_ids = Arc::clone(&self.scheduled_ids);

        tokio::spawn(async move {
            // Calculate wait time
            let now = Utc::now();
            let remind_at = reminder.remind_at;
            let delay = (remind_at - now).to_std().unwrap_or(Duration::from_secs(0));

            if delay.as_secs() > 0 {
                tracing::debug!(
                    "Scheduled reminder {} to be sent in {} seconds",
                    reminder.id,
                    delay.as_secs()
                );

                // Wait until needed time
                sleep(delay).await;
            }

            // Send reminder
            let notifier = ReminderNotifier::new(bot);
            if let Err(e) = notifier.send_reminder(&reminder).await {
                tracing::error!("Failed to send reminder {}: {}", reminder.id, e);
            } else {
                // Отмечаем как отправленное
                let repo = ReminderRepository::new(pool);
                if let Err(e) = repo.mark_as_sent(reminder.id).await {
                    tracing::error!("Failed to mark reminder {} as sent: {}", reminder.id, e);
                }

                // Remove from scheduled list
                let mut scheduled = scheduled_ids.lock().await;
                scheduled.remove(&reminder.id);

                tracing::info!("Reminder {} sent successfully", reminder.id);
            }
        });
    }

    /// Send reminder immediately
    async fn send_reminder_now(&self, reminder: crate::db::models::Reminder) {
        let bot = self.bot.clone();
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let notifier = ReminderNotifier::new(bot);
            if let Err(e) = notifier.send_reminder(&reminder).await {
                tracing::error!("Failed to send reminder {}: {}", reminder.id, e);
            } else {
                let repo = ReminderRepository::new(pool);
                if let Err(e) = repo.mark_as_sent(reminder.id).await {
                    tracing::error!("Failed to mark reminder {} as sent: {}", reminder.id, e);
                }
                tracing::info!("Reminder {} sent immediately", reminder.id);
            }
        });
    }
}
