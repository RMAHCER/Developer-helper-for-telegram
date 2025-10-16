// Реализация команд бота
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

/// /start - приветствие и регистрация пользователя
pub async fn start(bot: Bot, msg: Message, pool: PgPool) -> HandlerResult {
    // БЕЗОПАСНО: проверяем наличие пользователя
    let user = msg.from().ok_or("No user in message")?;

    // Регистрируем пользователя в БД (если еще не зарегистрирован)
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
        "👋 Привет, {}!\n\n\
        Я — мультифункциональный бот-помощник.\n\n\
        🎯 **Что я умею:**\n\
        ✅ Управление задачами (ToDo)\n\
        ⏰ Напоминания\n\
        📄 Конвертация файлов\n\n\
        Используй /help, чтобы увидеть все команды.",
        user.first_name
    );

    bot.send_message(msg.chat.id, welcome_text)
        .parse_mode(ParseMode::MarkdownV2)
        .reply_markup(keyboards::main_menu())
        .await?;

    Ok(())
}

/// /help - справка по командам
pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    let help_text = r#"📚 Доступные команды:

Управление задачами:
/addtodo <текст> \- добавить новую задачу
/listtodos \- показать все задачи
/completetodo <id> \- отметить задачу выполненной
/deletetodo <id> \- удалить задачу

Напоминания:
/remind <время> <текст> \- установить напоминание
  Пример: /remind 15m Проверить почту
  Форматы: 5m \(минуты\), 2h \(часы\), 1d \(дни\)
/listreminders \- показать активные напоминания
/cancelreminder <id> \- отменить напоминание

Общее:
/start \- начать работу
/help \- эта справка"#;

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// /addtodo - добавить новую задачу
pub async fn add_todo(bot: Bot, msg: Message, pool: PgPool, text: String) -> HandlerResult {
    // ВАЛИДАЦИЯ: ограничиваем длину текста (защита от DoS)
    if text.is_empty() {
        bot.send_message(msg.chat.id, "❌ Текст задачи не может быть пустым!")
            .await?;
        return Ok(());
    }

    if text.len() > 1000 {
        bot.send_message(msg.chat.id, "❌ Текст задачи слишком длинный (макс. 1000 символов)!")
            .await?;
        return Ok(());
    }

    let user_id = msg.from().ok_or("No user in message")?.id.0 as i64;

    // Получаем ID пользователя из БД
    let user: crate::db::models::User = sqlx::query_as(
        "SELECT * FROM users WHERE telegram_id = $1",
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

    // Создаем задачу
    let todo_repo = TodoRepository::new(pool);
    let todo_service = TodoService::new(todo_repo);
    let todo = todo_service.create_todo(user.id, text, None, Some(3)).await?;

    bot.send_message(
        msg.chat.id,
        format!("✅ Задача добавлена\\!\n\n📝 {}\n🆔 ID: {}",
            todo.title.replace("-", "\\-").replace(".", "\\.").replace("!", "\\!"),
            todo.id
        ),
    )
    .parse_mode(ParseMode::MarkdownV2)
    .reply_markup(keyboards::todo_actions(todo.id))
    .await?;

    Ok(())
}

/// /listtodos - показать все задачи
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
        bot.send_message(msg.chat.id, "📋 У вас пока нет задач.\nДобавьте первую: /addtodo <текст>")
            .await?;
        return Ok(());
    }

    let mut text = "📋 *Ваши задачи:*\n\n".to_string();

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

/// /completetodo - отметить задачу выполненной
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

    // Проверяем владельца
    if todo.user_id != user.id {
        bot.send_message(msg.chat.id, "❌ Это не ваша задача!")
            .await?;
        return Ok(());
    }

    // Обновляем статус через репозиторий напрямую
    let repo2 = TodoRepository::new(pool);
    repo2.mark_completed(id).await?;

    bot.send_message(msg.chat.id, format!("✅ Задача #{} отмечена как выполненная!", id))
        .await?;

    Ok(())
}

/// /deletetodo - удалить задачу
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
        bot.send_message(msg.chat.id, "❌ Это не ваша задача!")
            .await?;
        return Ok(());
    }

    todo_service.delete_todo(id).await?;

    bot.send_message(msg.chat.id, format!("🗑 Задача #{} удалена!", id))
        .await?;

    Ok(())
}

/// /remind - установить напоминание
pub async fn set_reminder(bot: Bot, msg: Message, pool: PgPool, text: String) -> HandlerResult {
    // Парсим формат: "15m Проверить почту" или "2h Встреча"
    let parts: Vec<&str> = text.splitn(2, ' ').collect();

    if parts.len() < 2 {
        bot.send_message(
            msg.chat.id,
            "❌ Неверный формат!\n\nИспользуйте: /remind <время> <текст>\nПример: /remind 15m Проверить почту",
        )
        .await?;
        return Ok(());
    }

    let time_str = parts[0];
    let reminder_text = parts[1];

    // ВАЛИДАЦИЯ: ограничиваем длину текста напоминания
    if reminder_text.len() > 500 {
        bot.send_message(msg.chat.id, "❌ Текст напоминания слишком длинный (макс. 500 символов)!")
            .await?;
        return Ok(());
    }

    // Парсим время
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
            "⏰ Напоминание установлено!\n\n📝 {}\n🕐 Напомню через {}\n🆔 ID: {}",
            reminder_text, time_str, reminder.id
        ),
    )
    .await?;

    Ok(())
}

/// /listreminders - показать активные напоминания
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
        bot.send_message(msg.chat.id, "⏰ У вас нет активных напоминаний.")
            .await?;
        return Ok(());
    }

    let mut text = "⏰ *Активные напоминания:*\n\n".to_string();

    for reminder in reminders {
        text.push_str(&format!(
            "🆔 \\#{} \\- {}\n🕐 {}\n\n",
            reminder.id,
            reminder.message.as_deref().unwrap_or("Без текста").replace("-", "\\-").replace(".", "\\."),
            reminder.remind_at.format("%d\\.%m\\.%Y %H:%M")
        ));
    }

    bot.send_message(msg.chat.id, text)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

/// /cancelreminder - отменить напоминание
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
        bot.send_message(msg.chat.id, "❌ Это не ваше напоминание!")
            .await?;
        return Ok(());
    }

    let repo2 = ReminderRepository::new(pool);
    repo2.delete(id).await?;

    bot.send_message(msg.chat.id, format!("✅ Напоминание #{} отменено!", id))
        .await?;

    Ok(())
}

/// Обработка произвольных сообщений
pub async fn handle_message(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Используйте /help для просмотра доступных команд.")
        .await?;
    Ok(())
}

/// Обработка callback кнопок
pub async fn handle_callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    if let Some(data) = &q.data {
        bot.answer_callback_query(&q.id).await?;

        // Обработка callback данных
        tracing::info!("Callback received: {}", data);
    }
    Ok(())
}

/// Парсинг длительности из строки (5m, 2h, 1d)
fn parse_duration(s: &str) -> Result<chrono::Duration, Box<dyn std::error::Error + Send + Sync>> {
    let len = s.len();
    if len < 2 {
        return Err("Invalid duration format".into());
    }

    let num: i64 = s[..len-1].parse()?;

    // ВАЛИДАЦИЯ: защита от отрицательных и слишком больших значений
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
