// Todo models - модели и DTO для todo модуля
//
// Re-export моделей из db::models для удобства
// Добавляем специфичные DTO для API

pub use crate::db::models::{NewTodo, Todo, UpdateTodo};
use crate::shared::types::{Priority, TodoStatus};
use serde::{Deserialize, Serialize};

/// Фильтр для поиска задач
#[derive(Debug, Clone, Default)]
pub struct TodoFilter {
    pub status: Option<TodoStatus>,
    pub priority: Option<Priority>,
    pub search: Option<String>,
}

/// Опции сортировки задач
#[derive(Debug, Clone, Copy)]
pub enum TodoSort {
    CreatedAtAsc,
    CreatedAtDesc,
    PriorityAsc,
    PriorityDesc,
    TitleAsc,
    TitleDesc,
}

impl Default for TodoSort {
    fn default() -> Self {
        TodoSort::CreatedAtDesc
    }
}

/// DTO для вывода задачи пользователю
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoView {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub status_emoji: String,
    pub priority: i32,
    pub priority_emoji: String,
    pub created_at: String,
}

impl From<Todo> for TodoView {
    fn from(mut todo: Todo) -> Self {
        use crate::shared::utils::format_datetime;

        let status_str = todo.status.to_string();
        let status_emoji_str = todo.status_emoji().to_string();
        let priority_emoji_str = todo.priority_emoji().to_string();
        let created_str = format_datetime(&todo.created_at);

        TodoView {
            id: todo.id,
            title: todo.title,
            description: todo.description.take(),
            status: status_str,
            status_emoji: status_emoji_str,
            priority: todo.priority,
            priority_emoji: priority_emoji_str,
            created_at: created_str,
        }
    }
}
