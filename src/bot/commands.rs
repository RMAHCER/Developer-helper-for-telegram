// Bot command implementations
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use sqlx::PgPool;

use crate::{
    bot::keyboards,
    todo::service::TodoService,
    todo::repository::TodoRepository,
    reminder::service::ReminderService,
    reminder::repository::ReminderRepository,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// /start - welcome and register user
pub async fn start(bot: Bot, msg: Message, pool: PgPool) -> HandlerResult {
    // SAFE: check user exists
    let user = msg.from().ok_or("No user in message")?;

    // Register user in DB (if not already registered)
    sqlx::query(
        r#"
        INSERT INTO users (telegram_id, username, first_name)
        VALUES ($1, $2, $3)
        ON CONFLICT (telegram_id)
        DO UPDATE SET last_active_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(user.id.0 as i64)
    .bind(user.username.as_deref())
    .bind(user.first_name.as_str())
    .execute(&pool)
    .await?;

    let welcome_text = format!(
        "👋 Hello, {}\\!\n\n\
        I am a multifunctional assistant bot\\.\n\n\
        🎯 **What I can do:**\n\
        ✅ Task management \\(ToDo\\)\n\
        ⏰ Reminders\n\
        📄 File conversion\n\n\
        Use /help to see all commands\\.",
        user.first_name.replace("!", "\\!")
    );

    bot.send_message(msg.chat.id, welcome_text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboards::main_menu())
        .await?;

    Ok(())
}

/// /help - command reference
pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    let help_text = r#"📚 Available commands:

Task Management:
/addtodo <text> \- add new task
/listtodos \- show all tasks
/completetodo <id> \- mark task as completed
/deletetodo <id> \- delete task

Reminders:
/remind <time> <text> \- set reminder
  Example: /remind 15m Check email
  Formats: 5m \(minutes\), 2h \(hours\), 1d \(days\)
/listreminders \- show active reminders
/cancelreminder <id> \- cancel reminder

General:
/start \- start bot
/help \- this help message"#;

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// /addtodo - add new task
pub async fn add_todo(bot: Bot, msg: Message, pool: PgPool, text: String) -> HandlerResult {
    // VALIDATION: limit text length (DoS protection)
    if text.is_empty() {
        bot.send_message(msg.chat.id, "❌ Task text cannot be empty!")
            .await?;
        return Ok(());
    }

    if text.len() > 1000 {
        bot.send_message(msg.chat.id, "❌ Task text is too long (max 1000 characters)!")
            .await?;
        return Ok(());
    }

    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    // Get user ID from DB
    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    // Create task
    let todo_repo = TodoRepository::new(pool);
    let todo_service = TodoService::new(todo_repo);
    let todo = todo_service.create_todo(user.id, text, None, Some(3)).await?;

    bot.send_message(
        msg.chat.id,
        format!("✅ Task added\\!\n\n📝 {}\n🆔 ID: {}",
            todo.title.replace("-", "\\-").replace(".", "\\.").replace("!", "\\!"),
            todo.id
        ),
    )
    .parse_mode(ParseMode::MarkdownV2)
    .reply_markup(keyboards::todo_actions(todo.id))
    .await?;

    Ok(())
}

/// /listtodos - show all tasks
pub async fn list_todos(bot: Bot, msg: Message, pool: PgPool) -> HandlerResult {
    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    let todo_repo = TodoRepository::new(pool);
    let todo_service = TodoService::new(todo_repo);
    let todos = todo_service.list_user_todos(user.id, None, None).await?;

    if todos.is_empty() {
        bot.send_message(msg.chat.id, "📋 You have no tasks yet\\.\nAdd your first task: /addtodo <text>")
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        return Ok(());
    }

    let mut text = "📋 *Your tasks:*\n\n".to_string();

    for todo in todos {
        let status_icon = match todo.status.as_str() {
            "completed" => "✅",
            "in_progress" => "🔄",
            "cancelled" => "❌",
            _ => "⏳",
        };

        text.push_str(&format!(
            "{} *\\#{}* {}\n   Status: {}\n\n",
            status_icon, todo.id,
            todo.title.replace("-", "\\-").replace(".", "\\.").replace("!", "\\!"),
            todo.status
        ));
    }

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// /completetodo - mark task as completed
pub async fn complete_todo(bot: Bot, msg: Message, pool: PgPool, id: i32) -> HandlerResult {
    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    let todo_repo = TodoRepository::new(pool.clone());
    let todo_service = TodoService::new(todo_repo);
    let todo = todo_service.get_todo(id).await?;

    // Check ownership
    if todo.user_id != user.id {
        bot.send_message(msg.chat.id, "❌ This is not your task!")
            .await?;
        return Ok(());
    }

    // Update status via repository directly
    let repo2 = TodoRepository::new(pool);
    repo2.mark_completed(id).await?;

    bot.send_message(msg.chat.id, format!("✅ Task #{} marked as completed!", id))
        .await?;

    Ok(())
}

/// /deletetodo - delete task
pub async fn delete_todo(bot: Bot, msg: Message, pool: PgPool, id: i32) -> HandlerResult {
    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    let todo_repo = TodoRepository::new(pool.clone());
    let todo_service = TodoService::new(todo_repo);
    let todo = todo_service.get_todo(id).await?;

    if todo.user_id != user.id {
        bot.send_message(msg.chat.id, "❌ This is not your task!")
            .await?;
        return Ok(());
    }

    todo_service.delete_todo(id).await?;

    bot.send_message(msg.chat.id, format!("🗑 Task #{} deleted!", id))
        .await?;

    Ok(())
}

/// /remind - set reminder
pub async fn set_reminder(bot: Bot, msg: Message, pool: PgPool, text: String) -> HandlerResult {
    // Parse format: "15m Check email" or "2h Meeting"
    let parts: Vec<&str> = text.splitn(2, ' ').collect();

    if parts.len() < 2 {
        bot.send_message(
            msg.chat.id,
            "❌ Invalid format!\n\nUse: /remind <time> <text>\nExample: /remind 15m Check email",
        )
        .await?;
        return Ok(());
    }

    let time_str = parts[0];
    let reminder_text = parts[1];

    // VALIDATION: limit reminder text length
    if reminder_text.len() > 500 {
        bot.send_message(msg.chat.id, "❌ Reminder text is too long (max 500 characters)!")
            .await?;
        return Ok(());
    }

    // Parse time
    let duration = parse_duration(time_str)?;
    let _remind_time = chrono::Utc::now() + duration;

    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;
    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    let reminder_repo = ReminderRepository::new(pool);
    let reminder_service = ReminderService::new(reminder_repo);
    let reminder = reminder_service
        .create_reminder(user.id, None, &format!("{}m", duration.num_minutes()), Some(reminder_text.to_string()))
        .await?;

    bot.send_message(
        msg.chat.id,
        format!(
            "⏰ Reminder set!\n\n📝 {}\n🕐 Will remind in {}\n🆔 ID: {}",
            reminder_text, time_str, reminder.id
        ),
    )
    .await?;

    Ok(())
}

/// /listreminders - show active reminders
pub async fn list_reminders(bot: Bot, msg: Message, pool: PgPool) -> HandlerResult {
    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    let reminder_repo = ReminderRepository::new(pool);
    let reminders = reminder_repo.find_by_user(user.id).await?;

    if reminders.is_empty() {
        bot.send_message(msg.chat.id, "⏰ You have no active reminders.")
            .await?;
        return Ok(());
    }

    let mut text = "⏰ *Active reminders:*\n\n".to_string();

    for reminder in reminders {
        text.push_str(&format!(
            "🆔 \\#{} \\- {}\n🕐 {}\n\n",
            reminder.id,
            reminder.message.as_deref().unwrap_or("No text").replace("-", "\\-").replace(".", "\\."),
            reminder.remind_at.format("%d\\.%m\\.%Y %H:%M")
        ));
    }

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// /cancelreminder - cancel reminder
pub async fn cancel_reminder(bot: Bot, msg: Message, pool: PgPool, id: i32) -> HandlerResult {
    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    let reminder_repo = ReminderRepository::new(pool.clone());
    let reminder = reminder_repo.find_by_id(id).await?;

    if reminder.user_id != user.id {
        bot.send_message(msg.chat.id, "❌ This is not your reminder!")
            .await?;
        return Ok(());
    }

    let repo2 = ReminderRepository::new(pool);
    repo2.delete(id).await?;

    bot.send_message(msg.chat.id, format!("✅ Reminder #{} cancelled!", id))
        .await?;

    Ok(())
}

/// Handle arbitrary messages
pub async fn handle_message(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Use /help to see available commands.")
        .await?;
    Ok(())
}

/// Handle callback buttons
pub async fn handle_callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    if let Some(data) = &q.data {
        bot.answer_callback_query(&q.id).await?;

        // Process callback data
        tracing::info!("Callback received: {}", data);
    }
    Ok(())
}

/// Parse duration from string (5m, 2h, 1d)
fn parse_duration(s: &str) -> Result<chrono::Duration, Box<dyn std::error::Error + Send + Sync>> {
    let len = s.len();
    if len < 2 {
        return Err("Invalid duration format".into());
    }

    let num: i64 = s[..len-1].parse()?;

    // VALIDATION: protect from negative and too large values
    if num <= 0 {
        return Err("Duration must be positive".into());
    }

    if num > 365 {
        return Err("Duration too large (max 365 units)".into());
    }

    let unit = &s[len-1..];

    match unit {
        "m" => Ok(chrono::Duration::minutes(num)),
        "h" => Ok(chrono::Duration::hours(num)),
        "d" => Ok(chrono::Duration::days(num)),
        _ => Err("Invalid time unit (use m, h, or d)".into()),
    }
}
