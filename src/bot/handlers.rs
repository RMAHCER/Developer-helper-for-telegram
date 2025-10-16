// Главный обработчик команд бота
use teloxide::{
    dispatching::{dialogue, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

use crate::bot::{commands, state::State};

/// Основная схема обработчиков бота
pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(commands::start))
        .branch(case![Command::Help].endpoint(commands::help))
        .branch(case![Command::AddTodo(text)].endpoint(commands::add_todo))
        .branch(case![Command::ListTodos].endpoint(commands::list_todos))
        .branch(case![Command::CompleteTodo(id)].endpoint(commands::complete_todo))
        .branch(case![Command::DeleteTodo(id)].endpoint(commands::delete_todo))
        .branch(case![Command::Remind(text)].endpoint(commands::set_reminder))
        .branch(case![Command::ListReminders].endpoint(commands::list_reminders))
        .branch(case![Command::CancelReminder(id)].endpoint(commands::cancel_reminder));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(commands::handle_message));

    let callback_query_handler = Update::filter_callback_query()
        .branch(dptree::endpoint(commands::handle_callback));

    dialogue::enter::<Update, dialogue::InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

/// Команды бота
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Начать работу с ботом")]
    Start,

    #[command(description = "Показать справку")]
    Help,

    #[command(description = "Добавить заyesчу: /addtodo <текст>")]
    AddTodo(String),

    #[command(description = "Показать список заyesч")]
    ListTodos,

    #[command(description = "Отметить заyesчу выполненной: /completetodo <id>")]
    CompleteTodo(i32),

    #[command(description = "Delete task: /deletetodo <id>")]
    DeleteTodo(i32),

    #[command(description = "Установить напоминание: /remind <время> <текст>")]
    Remind(String),

    #[command(description = "Список напоминаний")]
    ListReminders,

    #[command(description = "Отменить напоминание: /cancelreminder <id>")]
    CancelReminder(i32),
}
