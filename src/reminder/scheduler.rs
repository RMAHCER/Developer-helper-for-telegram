// Reminder scheduler - планировщик напоминаний на Tokio tasks
//
// Архитектура:
// 1. Background task проверяет новые напоминания каждые 30 сек
// 2. Для каждого pending напоминания создаётся Tokio task с delay
// 3. Когда время приходит, task отправляет уведомление
//
// Масштабируется до ~10K одновременных напоминаний

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

/// Планировщик напоминаний
pub struct ReminderScheduler {
    pool: PgPool,
    bot: Bot,
    scheduled_ids: Arc<Mutex<HashSet<i32>>>, // Отслеживаем запланированные напоминания
}

impl ReminderScheduler {
    pub fn new(pool: PgPool, bot: Bot) -> Self {
        Self {
            pool,
            bot,
            scheduled_ids: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Запустить планировщик (фоновая задача)
    pub async fn run(self) -> Result<()> {
        tracing::info!("Starting reminder scheduler...");

        loop {
            if let Err(e) = self.check_and_schedule().await {
                tracing::error!("Scheduler error: {}", e);
            }

            // Проверяем каждые 30 секунд
            sleep(Duration::from_secs(30)).await;
        }
    }

    /// Проверить и запланировать новые напоминания
    async fn check_and_schedule(&self) -> Result<()> {
        let repo = ReminderRepository::new(self.pool.clone());
        let reminders = repo.get_pending_reminders(Utc::now()).await?;

        let mut scheduled = self.scheduled_ids.lock().await;

        for reminder in reminders {
            // Пропускаем уже запланированные
            if scheduled.contains(&reminder.id) {
                continue;
            }

            // Проверяем, не пора ли отправлять уже сейчас
            let now = Utc::now();
            if reminder.remind_at <= now {
                self.send_reminder_now(reminder.clone()).await;
            } else {
                // Планируем на будущее
                self.schedule_reminder(reminder.clone()).await;
            }

            scheduled.insert(reminder.id);
        }

        tracing::debug!("Scheduler check completed. Total scheduled: {}", scheduled.len());

        // Очищаем старые IDs (опционально, чтобы не росло бесконечно)
        if scheduled.len() > 100_000 {
            scheduled.clear();
            tracing::warn!("Cleared scheduled IDs cache (reached 100K)");
        }

        Ok(())
    }

    /// Запланировать напоминание на будущее
    async fn schedule_reminder(&self, reminder: crate::db::models::Reminder) {
        let bot = self.bot.clone();
        let pool = self.pool.clone();
        let scheduled_ids = Arc::clone(&self.scheduled_ids);

        tokio::spawn(async move {
            // Вычисляем время ожидания
            let now = Utc::now();
            let remind_at = reminder.remind_at;
            let delay = (remind_at - now).to_std().unwrap_or(Duration::from_secs(0));

            if delay.as_secs() > 0 {
                tracing::debug!(
                    "Scheduled reminder {} to be sent in {} seconds",
                    reminder.id,
                    delay.as_secs()
                );

                // Ждём до нужного времени
                sleep(delay).await;
            }

            // Отправляем напоминание
            let notifier = ReminderNotifier::new(bot);
            if let Err(e) = notifier.send_reminder(&reminder).await {
                tracing::error!("Failed to send reminder {}: {}", reminder.id, e);
            } else {
                // Отмечаем как отправленное
                let repo = ReminderRepository::new(pool);
                if let Err(e) = repo.mark_as_sent(reminder.id).await {
                    tracing::error!("Failed to mark reminder {} as sent: {}", reminder.id, e);
                }

                // Убираем из списка запланированных
                let mut scheduled = scheduled_ids.lock().await;
                scheduled.remove(&reminder.id);

                tracing::info!("Reminder {} sent successfully", reminder.id);
            }
        });
    }

    /// Отправить напоминание немедленно
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
