// Todo handlers - command handlers for todo module
//
// Handlers connect Telegram commands with business logic
// Format responses for user

use crate::error::Result;
use crate::shared::types::TodoStatus;
use crate::todo::models::TodoView;

/// Format task list for user display
pub fn format_todo_list(todos: Vec<TodoView>) -> String {
    if todos.is_empty() {
        return "📝 You have no tasks yet.\n\nUse /newtodo to create one!".to_string();
    }

    let mut output = format!("📝 Your Tasks ({}):\n\n", todos.len());

    for (idx, todo) in todos.iter().enumerate() {
        output.push_str(&format!(
            "{}. {} {} *{}*\n",
            idx + 1,
            todo.status_emoji,
            todo.priority_emoji,
            escape_markdown(&todo.title)
        ));

        if let Some(desc) = &todo.description {
            if !desc.is_empty() {
                let short_desc = if desc.len() > 50 {
                    format!("{}...", &desc[..50])
                } else {
                    desc.clone()
                };
                output.push_str(&format!("   _{}_\n", escape_markdown(&short_desc)));
            }
        }

        output.push_str(&format!("   ID: {} | Priority: {}\n\n", todo.id, todo.priority));
    }

    output.push_str("💡 Use /todo <id> to view details");
    output
}

/// Format task details
pub fn format_todo_details(todo: &TodoView) -> String {
    let mut output = format!(
        "{} {} *Task #{}*\n\n",
        todo.status_emoji, todo.priority_emoji, todo.id
    );

    output.push_str(&format!("*Title:* {}\n\n", escape_markdown(&todo.title)));

    if let Some(desc) = &todo.description {
        if !desc.is_empty() {
            output.push_str(&format!("*Description:*\n{}\n\n", escape_markdown(desc)));
        }
    }

    output.push_str(&format!(
        "*Status:* {}\n*Priority:* {}\n*Created:* {}\n",
        todo.status, todo.priority, todo.created_at
    ));

    output.push_str("\n💡 Commands:\n");
    output.push_str(&format!("• /complete {} - Mark as done\n", todo.id));
    output.push_str(&format!("• /delete {} - Delete task\n", todo.id));

    output
}

/// Escape special characters for Markdown
fn escape_markdown(text: &str) -> String {
    text.replace('_', "\\_")
        .replace('*', "\\*")
        .replace('[', "\\[")
        .replace('`', "\\`")
}

/// Parse status string into enum
pub fn parse_status(status_str: &str) -> Result<TodoStatus> {
    status_str.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_markdown() {
        let input = "Test *bold* and _italic_";
        let escaped = escape_markdown(input);
        assert!(escaped.contains("\\*"));
        assert!(escaped.contains("\\_"));
    }
}
