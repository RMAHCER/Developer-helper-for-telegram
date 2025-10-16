// Main entry point для Telegram бота
use anyhow::Result;
use telegram_multitool_bot::{
    config::Config,
    db::pool::create_pool,
    reminder::scheduler::ReminderScheduler,
    shared::telemetry,
};
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализация логирования
    telemetry::init_telemetry().map_err(|e| anyhow::anyhow!(e))?;
    tracing::info!("🚀 Starting Telegram Multitool Bot...");

    // Загрузка конфигурации из .env
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    tracing::info!("✅ Configuration loaded");

    // Создание пула подключений к базе данных
    let db_pool = create_pool(&config.database).await?;
    tracing::info!("✅ Database pool created");

    // Запуск миграций
    telegram_multitool_bot::db::migrations::run_migrations(&db_pool).await?;
    tracing::info!("✅ Database migrations completed");

    // Инициализация бота
    let bot = Bot::new(&config.telegram.bot_token);
    tracing::info!("✅ Bot initialized");

    // Запуск планировщика напоминаний (фоновая задача)
    let scheduler = ReminderScheduler::new(db_pool.clone(), bot.clone());
    let scheduler_handle = tokio::spawn(async move {
        if let Err(e) = scheduler.run().await {
            tracing::error!("❌ Reminder scheduler error: {}", e);
        }
    });
    tracing::info!("✅ Reminder scheduler started");

    // Создание диспетчера команд
    let handler = telegram_multitool_bot::bot::handlers::schema();

    tracing::info!("✅ Starting bot dispatcher...");
    tracing::info!("Bot is ready to receive messages!");

    // Запуск бота с long polling
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db_pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    // Ожидание завершения фоновых задач
    scheduler_handle.await?;

    tracing::info!("👋 Bot stopped");
    Ok(())
}
