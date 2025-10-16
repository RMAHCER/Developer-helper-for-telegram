// Todo service - business logic for working with tasks
//
// Service layer: contains business rules and orchestration
// Uses repository for data access

use crate::db::models::{NewTodo, UpdateTodo};
use crate::error::{validation_error, Result};
use crate::shared::types::{DbId, Priority, TodoStatus};
use crate::todo::models::{Todo, TodoFilter, TodoSort, TodoView};
use crate::todo::repository::TodoRepository;

/// Service for working with tasks
#[derive(Clone)]
pub struct TodoService {
    repo: TodoRepository,
}

impl TodoService {
    pub fn new(repo: TodoRepository) -> Self {
        Self { repo }
    }

    /// Create a new task с валиyesцией
    pub async fn create_todo(
        &self,
        user_id: DbId,
        title: String,
        description: Option<String>,
        priority: Option<Priority>,
    ) -> Result<Todo> {
        // Validation
        if title.trim().is_empty() {
            return Err(validation_error("Todo title cannot be empty"));
        }

        if title.len() > 500 {
            return Err(validation_error("Todo title is too long (max 500 chars)"));
        }

        if let Some(desc) = &description {
            if desc.len() > 2000 {
                return Err(validation_error(
                    "Todo description is too long (max 2000 chars)",
                ));
            }
        }

        let priority = priority.unwrap_or(3); // Default priority
        if !(1..=5).contains(&priority) {
            return Err(validation_error("Priority must be between 1 and 5"));
        }

        let new_todo = NewTodo {
            user_id,
            title: title.trim().to_string(),
            description,
            priority,
        };

        self.repo.create(new_todo).await
    }

    /// Get task by ID
    pub async fn get_todo(&self, id: DbId) -> Result<Todo> {
        self.repo.find_by_id(id).await
    }

    /// Get all user tasks with filters
    pub async fn list_user_todos(
        &self,
        user_id: DbId,
        status: Option<TodoStatus>,
        sort: Option<TodoSort>,
    ) -> Result<Vec<TodoView>> {
        let filter = TodoFilter {
            status,
            ..Default::default()
        };

        let sort = sort.unwrap_or_default();
        let todos = self.repo.find_by_user(user_id, filter, sort).await?;

        Ok(todos.into_iter().map(TodoView::from).collect())
    }

    /// Update task
    pub async fn update_todo(
        &self,
        id: DbId,
        title: Option<String>,
        description: Option<String>,
        status: Option<TodoStatus>,
        priority: Option<Priority>,
    ) -> Result<Todo> {
        // Validation
        if let Some(ref t) = title {
            if t.trim().is_empty() {
                return Err(validation_error("Title cannot be empty"));
            }
            if t.len() > 500 {
                return Err(validation_error("Title is too long"));
            }
        }

        if let Some(ref d) = description {
            if d.len() > 2000 {
                return Err(validation_error("Description is too long"));
            }
        }

        if let Some(p) = priority {
            if !(1..=5).contains(&p) {
                return Err(validation_error("Priority must be between 1 and 5"));
            }
        }

        let update = UpdateTodo {
            title: title.map(|t| t.trim().to_string()),
            description,
            status,
            priority,
        };

        self.repo.update(id, update).await
    }

    /// Delete task
    pub async fn delete_todo(&self, id: DbId) -> Result<()> {
        self.repo.delete(id).await
    }

    /// Mark task as completed
    pub async fn complete_todo(&self, id: DbId) -> Result<Todo> {
        self.repo.mark_completed(id).await
    }

    /// Change task status
    pub async fn change_status(&self, id: DbId, status: TodoStatus) -> Result<Todo> {
        let update = UpdateTodo {
            status: Some(status),
            ..Default::default()
        };

        self.repo.update(id, update).await
    }

    /// Get user statistics
    pub async fn get_stats(&self, user_id: DbId) -> Result<String> {
        let stats = self.repo.get_user_stats(user_id).await?;

        Ok(format!(
            "📊 Your Statistics:\n\n\
             Total: {}\n\
             ⏳ Pending: {}\n\
             🔄 In Progress: {}\n\
             ✅ Completed: {}\n\
             ❌ Cancelled: {}",
            stats.total, stats.pending, stats.in_progress, stats.completed, stats.cancelled
        ))
    }
}
