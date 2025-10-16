// Todo repository - слой доступа к данным
//
// Repository pattern: изолирует бизнес-логику от деталей работы с БД
// Все SQL запросы находятся здесь

use crate::db::models::{NewTodo, Todo, UpdateTodo};
use crate::error::{not_found, Result};
use crate::shared::types::DbId;
use crate::todo::models::{TodoFilter, TodoSort};
use sqlx::PgPool;

/// Repository для работы с задачами
#[derive(Clone)]
pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Создать новую задачу
    pub async fn create(&self, new_todo: NewTodo) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (user_id, title, description, priority)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(new_todo.user_id)
        .bind(&new_todo.title)
        .bind(&new_todo.description)
        .bind(new_todo.priority)
        .fetch_one(&self.pool)
        .await?;

        tracing::debug!("Created todo {} for user {}", todo.id, todo.user_id);
        Ok(todo)
    }

    /// Найти задачу по ID
    pub async fn find_by_id(&self, id: DbId) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| not_found(format!("Todo with id {} not found", id)))?;

        Ok(todo)
    }

    /// Найти все задачи пользователя
    pub async fn find_by_user(
        &self,
        user_id: DbId,
        filter: TodoFilter,
        sort: TodoSort,
    ) -> Result<Vec<Todo>> {
        // БЕЗОПАСНОЕ построение запроса - используем только параметры
        // ORDER BY безопасен т.к. использует enum (не пользовательский ввод)
        let order_by = match sort {
            TodoSort::CreatedAtAsc => "created_at ASC",
            TodoSort::CreatedAtDesc => "created_at DESC",
            TodoSort::PriorityAsc => "priority ASC",
            TodoSort::PriorityDesc => "priority DESC",
            TodoSort::TitleAsc => "title ASC",
            TodoSort::TitleDesc => "title DESC",
        };

        // ЗАЩИТА от DoS: ограничиваем количество возвращаемых записей
        const MAX_TODOS: i64 = 1000;

        // Используем разные запросы в зависимости от фильтров (безопасно)
        let todos = match (&filter.status, filter.priority, &filter.search) {
            (None, None, None) => {
                // Без фильтров
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 ORDER BY {} LIMIT {}",
                    order_by, MAX_TODOS
                ))
                .bind(user_id)
                .fetch_all(&self.pool)
                .await?
            }
            (Some(status), None, None) => {
                // Только статус
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND status = $2 ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(status.to_string())
                .fetch_all(&self.pool)
                .await?
            }
            (None, Some(priority), None) => {
                // Только приоритет
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND priority = $2 ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(priority)
                .fetch_all(&self.pool)
                .await?
            }
            (None, None, Some(search)) => {
                // Только поиск
                let search_pattern = format!("%{}%", search);
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND (title ILIKE $2 OR description ILIKE $2) ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(search_pattern)
                .fetch_all(&self.pool)
                .await?
            }
            (Some(status), Some(priority), None) => {
                // Статус + приоритет
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND status = $2 AND priority = $3 ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(status.to_string())
                .bind(priority)
                .fetch_all(&self.pool)
                .await?
            }
            (Some(status), None, Some(search)) => {
                // Статус + поиск
                let search_pattern = format!("%{}%", search);
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND status = $2 AND (title ILIKE $3 OR description ILIKE $3) ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(status.to_string())
                .bind(search_pattern)
                .fetch_all(&self.pool)
                .await?
            }
            (None, Some(priority), Some(search)) => {
                // Приоритет + поиск
                let search_pattern = format!("%{}%", search);
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND priority = $2 AND (title ILIKE $3 OR description ILIKE $3) ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(priority)
                .bind(search_pattern)
                .fetch_all(&self.pool)
                .await?
            }
            (Some(status), Some(priority), Some(search)) => {
                // Все фильтры
                let search_pattern = format!("%{}%", search);
                sqlx::query_as::<_, Todo>(&format!(
                    "SELECT * FROM todos WHERE user_id = $1 AND status = $2 AND priority = $3 AND (title ILIKE $4 OR description ILIKE $4) ORDER BY {}",
                    order_by
                ))
                .bind(user_id)
                .bind(status.to_string())
                .bind(priority)
                .bind(search_pattern)
                .fetch_all(&self.pool)
                .await?
            }
        };

        tracing::debug!("Found {} todos for user {}", todos.len(), user_id);
        Ok(todos)
    }

    /// Обновить задачу
    pub async fn update(&self, id: DbId, update: UpdateTodo) -> Result<Todo> {
        // БЕЗОПАСНОЕ обновление - используем фиксированные запросы для каждой комбинации полей
        if update.title.is_none()
            && update.description.is_none()
            && update.status.is_none()
            && update.priority.is_none()
        {
            return self.find_by_id(id).await;
        }

        // Используем разные запросы в зависимости от того, какие поля обновляются
        let todo = match (&update.title, &update.description, &update.status, update.priority) {
            (Some(title), None, None, None) => {
                sqlx::query_as::<_, Todo>("UPDATE todos SET title = $1 WHERE id = $2 RETURNING *")
                    .bind(title)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (None, Some(desc), None, None) => {
                sqlx::query_as::<_, Todo>(
                    "UPDATE todos SET description = $1 WHERE id = $2 RETURNING *",
                )
                .bind(desc)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
            (None, None, Some(status), None) => {
                sqlx::query_as::<_, Todo>("UPDATE todos SET status = $1 WHERE id = $2 RETURNING *")
                    .bind(status.to_string())
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
            (None, None, None, Some(priority)) => {
                sqlx::query_as::<_, Todo>(
                    "UPDATE todos SET priority = $1 WHERE id = $2 RETURNING *",
                )
                .bind(priority)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
            // Множественные обновления
            (Some(title), Some(desc), None, None) => {
                sqlx::query_as::<_, Todo>(
                    "UPDATE todos SET title = $1, description = $2 WHERE id = $3 RETURNING *",
                )
                .bind(title)
                .bind(desc)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
            (Some(title), None, Some(status), None) => {
                sqlx::query_as::<_, Todo>(
                    "UPDATE todos SET title = $1, status = $2 WHERE id = $3 RETURNING *",
                )
                .bind(title)
                .bind(status.to_string())
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
            (Some(title), None, None, Some(priority)) => {
                sqlx::query_as::<_, Todo>(
                    "UPDATE todos SET title = $1, priority = $2 WHERE id = $3 RETURNING *",
                )
                .bind(title)
                .bind(priority)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
            // Обновление всех полей
            (Some(title), Some(desc), Some(status), Some(priority)) => {
                sqlx::query_as::<_, Todo>(
                    "UPDATE todos SET title = $1, description = $2, status = $3, priority = $4 WHERE id = $5 RETURNING *",
                )
                .bind(title)
                .bind(desc)
                .bind(status.to_string())
                .bind(priority)
                .bind(id)
                .fetch_one(&self.pool)
                .await?
            }
            // Для остальных комбинаций - обновляем все не-None поля
            _ => {
                // Fallback: обновляем только те поля, которые заданы
                let mut sql = String::from("UPDATE todos SET ");
                let mut updates = Vec::new();

                if update.title.is_some() {
                    updates.push("title = COALESCE($1, title)");
                }
                if update.description.is_some() {
                    updates.push("description = COALESCE($2, description)");
                }
                if update.status.is_some() {
                    updates.push("status = COALESCE($3, status)");
                }
                if update.priority.is_some() {
                    updates.push("priority = COALESCE($4, priority)");
                }

                sql.push_str(&updates.join(", "));
                sql.push_str(" WHERE id = $5 RETURNING *");

                sqlx::query_as::<_, Todo>(&sql)
                    .bind(update.title)
                    .bind(update.description)
                    .bind(update.status.map(|s| s.to_string()))
                    .bind(update.priority)
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?
            }
        };

        tracing::debug!("Updated todo {}", id);
        Ok(todo)
    }

    /// Удалить задачу
    pub async fn delete(&self, id: DbId) -> Result<()> {
        let result = sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(not_found(format!("Todo with id {} not found", id)));
        }

        tracing::debug!("Deleted todo {}", id);
        Ok(())
    }

    /// Отметить задачу как выполненную
    pub async fn mark_completed(&self, id: DbId) -> Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET status = 'completed', completed_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| not_found(format!("Todo with id {} not found", id)))?;

        tracing::debug!("Marked todo {} as completed", id);
        Ok(todo)
    }

    /// Получить статистику по задачам пользователя
    pub async fn get_user_stats(&self, user_id: DbId) -> Result<TodoStats> {
        let stats = sqlx::query_as::<_, TodoStats>(
            r#"
            SELECT
                COUNT(*) as total,
                COUNT(*) FILTER (WHERE status = 'pending') as pending,
                COUNT(*) FILTER (WHERE status = 'in_progress') as in_progress,
                COUNT(*) FILTER (WHERE status = 'completed') as completed,
                COUNT(*) FILTER (WHERE status = 'cancelled') as cancelled
            FROM todos
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(stats)
    }
}

/// Статистика по задачам
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TodoStats {
    pub total: i64,
    pub pending: i64,
    pub in_progress: i64,
    pub completed: i64,
    pub cancelled: i64,
}
