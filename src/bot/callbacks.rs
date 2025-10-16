// Обработка callback запросов от inline кнопок
use teloxide::prelude::*;

pub type CallbackResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// Обработка callback для задач
pub async fn handle_todo_callback(
    bot: Bot,
    query: CallbackQuery,
    data: String,
) -> CallbackResult {
    bot.answer_callback_query(&query.id).await?;

    if data.starts_with("complete_") {
        let todo_id = data.strip_prefix("complete_").unwrap().parse::<i32>()?;
        // Логика завершения задачи
        if let Some(Message { chat, .. }) = query.message {
            bot.send_message(chat.id, format!("✅ Задача #{} отмечена как выполненная!", todo_id))
                .await?;
        }
    } else if data.starts_with("delete_") {
        let todo_id = data.strip_prefix("delete_").unwrap().parse::<i32>()?;
        if let Some(Message { chat, .. }) = query.message {
            bot.send_message(chat.id, format!("🗑 Задача #{} удалена!", todo_id))
                .await?;
        }
    }

    Ok(())
}
