-- Reminders table
-- Хранение напоминаний для задач и произвольных событий

CREATE TABLE reminders (
    -- Primary key
    id SERIAL PRIMARY KEY,

    -- Связь с задачей (опционально, может быть отдельное напоминание)
    todo_id INTEGER REFERENCES todos(id) ON DELETE CASCADE,

    -- Связь с пользователем (обязательно, каскадное удаление)
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Reminder details
    remind_at TIMESTAMP NOT NULL,     -- Когда напомнить
    message TEXT,                      -- Текст напоминания (если не связано с задачей)

    -- Status
    is_sent BOOLEAN DEFAULT FALSE,     -- Отправлено ли напоминание
    sent_at TIMESTAMP,                 -- Когда было отправлено

    -- Recurrence
    is_recurring BOOLEAN DEFAULT FALSE,        -- Повторяющееся напоминание
    recurrence_pattern VARCHAR(50),            -- daily, weekly, monthly, custom

    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Индекс для быстрого поиска неотправленных напоминаний
-- Это критический индекс для планировщика
CREATE INDEX idx_reminders_remind_at ON reminders(remind_at)
WHERE is_sent = FALSE;

-- Индекс для поиска напоминаний пользователя
CREATE INDEX idx_reminders_user_id ON reminders(user_id);

-- Индекс для поиска напоминаний конкретной задачи
CREATE INDEX idx_reminders_todo_id ON reminders(todo_id)
WHERE todo_id IS NOT NULL;

-- Индекс для повторяющихся напоминаний
CREATE INDEX idx_reminders_recurring ON reminders(is_recurring)
WHERE is_recurring = TRUE AND is_sent = FALSE;
