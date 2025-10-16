// Keyboards and inline buttons for bot
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};

/// Main bot menu
pub fn main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("📝 Tasks"),
            KeyboardButton::new("⏰ Reminders"),
        ],
        vec![
            KeyboardButton::new("📄 Convert File"),
            KeyboardButton::new("❓ Help"),
        ],
    ])
    .resize_keyboard(true)
}

/// Actions for specific task
pub fn todo_actions(todo_id: i32) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Complete", format!("complete_{}", todo_id)),
            InlineKeyboardButton::callback("❌ Delete", format!("delete_{}", todo_id)),
        ],
        vec![
            InlineKeyboardButton::callback("⏰ Remind", format!("remind_{}", todo_id)),
        ],
    ])
}

/// File conversion menu
pub fn conversion_menu(file_type: &str) -> InlineKeyboardMarkup {
    match file_type {
        "image" => InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("→ PNG", "convert_png"),
                InlineKeyboardButton::callback("→ JPEG", "convert_jpeg"),
            ],
            vec![
                InlineKeyboardButton::callback("→ WebP", "convert_webp"),
                InlineKeyboardButton::callback("🔄 Compress", "compress"),
            ],
        ]),
        "document" => InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("→ PDF", "convert_pdf"),
            ],
        ]),
        _ => InlineKeyboardMarkup::new(Vec::<Vec<InlineKeyboardButton>>::new()),
    }
}

/// Confirmation buttons
pub fn confirmation_keyboard(action: &str, id: i32) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Yes", format!("{}_{}_yes", action, id)),
            InlineKeyboardButton::callback("❌ No", format!("{}_{}_no", action, id)),
        ],
    ])
}
