// Клавиатуры и inline-кнопки для бота
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};

/// Главное меню бота
pub fn main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("📝 Задачи"),
            KeyboardButton::new("⏰ Напоминания"),
        ],
        vec![
            KeyboardButton::new("📄 Конвертировать файл"),
            KeyboardButton::new("❓ Помощь"),
        ],
    ])
    .resize_keyboard(true)
}

/// Действия для конкретной задачи
pub fn todo_actions(todo_id: i32) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Выполнено", format!("complete_{}", todo_id)),
            InlineKeyboardButton::callback("❌ Удалить", format!("delete_{}", todo_id)),
        ],
        vec![
            InlineKeyboardButton::callback("⏰ Напомнить", format!("remind_{}", todo_id)),
        ],
    ])
}

/// Меню конвертации файлов
pub fn conversion_menu(file_type: &str) -> InlineKeyboardMarkup {
    match file_type {
        "image" => InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("→ PNG", "convert_png"),
                InlineKeyboardButton::callback("→ JPEG", "convert_jpeg"),
            ],
            vec![
                InlineKeyboardButton::callback("→ WebP", "convert_webp"),
                InlineKeyboardButton::callback("🔄 Сжать", "compress"),
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

/// Кнопки подтверждения
pub fn confirmation_keyboard(action: &str, id: i32) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("✅ Да", format!("{}_{}_yes", action, id)),
            InlineKeyboardButton::callback("❌ Нет", format!("{}_{}_no", action, id)),
        ],
    ])
}
