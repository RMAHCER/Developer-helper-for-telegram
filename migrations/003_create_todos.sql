-- Todos table
-- Хранение задач пользователей

CREATE TABLE todos (
    -- Primary key
    id SERIAL PRIMARY KEY,

    -- Связь с пользователем (каскадное удаление)
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Todo content
    title TEXT NOT NULL,              -- Заголовок задачи
    description TEXT,                 -- Описание (опционально)

    -- Status and priority
    status VARCHAR(20) DEFAULT 'pending',  -- pending, in_progress, completed, cancelled
    priority INTEGER DEFAULT 3,            -- 1 (highest) to 5 (lowest)

    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP             -- Время завершения задачи
);

-- Индекс для быстрого поиска задач пользователя по статусу
CREATE INDEX idx_todos_user_status ON todos(user_id, status);

-- Индекс для сортировки по приоритету
CREATE INDEX idx_todos_priority ON todos(priority DESC);

-- Индекс для сортировки по дате создания
CREATE INDEX idx_todos_created_at ON todos(created_at DESC);

-- Триггер для автоматического обновления updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_todos_updated_at
    BEFORE UPDATE ON todos
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
