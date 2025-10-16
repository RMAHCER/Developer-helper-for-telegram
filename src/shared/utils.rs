// Shared utilities - common utility functions

use chrono::{DateTime, Duration, Utc};

/// Parse relative time (например: "1h", "30m", "2d")
pub fn parse_relative_time(input: &str) -> Option<DateTime<Utc>> {
    let input = input.trim().to_lowercase();

    // Parse into number and time unit
    let (num_str, unit) = if input.ends_with('m') {
        (&input[..input.len() - 1], 'm')
    } else if input.ends_with('h') {
        (&input[..input.len() - 1], 'h')
    } else if input.ends_with('d') {
        (&input[..input.len() - 1], 'd')
    } else {
        return None;
    };

    let num: i64 = num_str.parse().ok()?;
    let now = Utc::now();

    match unit {
        'm' => Some(now + Duration::minutes(num)),
        'h' => Some(now + Duration::hours(num)),
        'd' => Some(now + Duration::days(num)),
        _ => None,
    }
}

/// Format date for user display
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M UTC").to_string()
}

/// Truncate text to specified length with "..."
pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len - 3])
    }
}

/// Эскейпинг специальных символов для Markdown
pub fn escape_markdown(text: &str) -> String {
    text.replace('_', "\\_")
        .replace('*', "\\*")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('~', "\\~")
        .replace('`', "\\`")
        .replace('>', "\\>")
        .replace('#', "\\#")
        .replace('+', "\\+")
        .replace('-', "\\-")
        .replace('=', "\\=")
        .replace('|', "\\|")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('.', "\\.")
        .replace('!', "\\!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_relative_time() {
        let result = parse_relative_time("30m");
        assert!(result.is_some());

        let result = parse_relative_time("2h");
        assert!(result.is_some());

        let result = parse_relative_time("invalid");
        assert!(result.is_none());
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("Hello", 10), "Hello");
        assert_eq!(truncate_text("Hello World!", 8), "Hello...");
    }
}
