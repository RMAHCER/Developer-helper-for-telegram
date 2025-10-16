// Main entry point for Telegram bot
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
    // Initialize logging
    telemetry::init_telemetry().map_err(|e| anyhow::anyhow!(e))?;
    tracing::info!("🚀 Starting Telegram Multitool Bot...");

    // Load configuration from .env
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    tracing::info!("✅ Configuration loaded");

    // Create database connection pool
    let db_pool = create_pool(&config.database).await?;
    tracing::info!("✅ Database pool created");

    // Run migrations
    telegram_multitool_bot::db::migrations::run_migrations(&db_pool).await?;
    tracing::info!("✅ Database migrations completed");

    // Initialize bot
    let bot = Bot::new(&config.telegram.bot_token);
    tracing::info!("✅ Bot initialized");

    // Start reminder scheduler (background task)
    let scheduler = ReminderScheduler::new(db_pool.clone(), bot.clone());
    let scheduler_handle = tokio::spawn(async move {
        if let Err(e) = scheduler.run().await {
            tracing::error!("❌ Reminder scheduler error: {}", e);
        }
    });
    tracing::info!("✅ Reminder scheduler started");

    // Create command dispatcher
    let handler = telegram_multitool_bot::bot::handlers::schema();

    tracing::info!("✅ Starting bot dispatcher...");
    tracing::info!("Bot is ready to receive messages!");

    // Start bot with long polling
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db_pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    // Wait for background tasks to complete
    scheduler_handle.await?;

    tracing::info!("👋 Bot stopped");
    Ok(())
}
