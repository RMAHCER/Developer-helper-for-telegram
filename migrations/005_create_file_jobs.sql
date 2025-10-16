-- File conversions table
-- Хранение истории конвертации файлов

CREATE TABLE file_conversions (
    -- Primary key
    id SERIAL PRIMARY KEY,

    -- Связь с пользователем (каскадное удаление)
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- File details
    source_file_id VARCHAR(255),      -- Telegram file_id исходного файла
    source_format VARCHAR(50),        -- jpg, png, pdf, etc.
    target_format VARCHAR(50),        -- целевой формат

    -- Job status
    status VARCHAR(20),               -- pending, processing, completed, failed
    result_file_path TEXT,            -- Путь к результату (если completed)
    error_message TEXT,               -- Сообщение об ошибке (если failed)

    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP            -- Когда завершена обработка
);

-- Индекс для поиска конвертаций пользователя
CREATE INDEX idx_conversions_user_status ON file_conversions(user_id, status);

-- Индекс для очистки старых конвертаций
CREATE INDEX idx_conversions_created_at ON file_conversions(created_at);

-- Индекс для поиска по статусу (для обработки очереди)
CREATE INDEX idx_conversions_status ON file_conversions(status)
WHERE status IN ('pending', 'processing');
