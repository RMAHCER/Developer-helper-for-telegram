-- Users table
-- Хранение информации о пользователях Telegram

CREATE TABLE users (
    -- Primary key
    id SERIAL PRIMARY KEY,

    -- Telegram user ID (уникальный идентификатор из Telegram)
    telegram_id BIGINT UNIQUE NOT NULL,

    -- User metadata
    username VARCHAR(255),          -- @username (может отсутствовать)
    first_name VARCHAR(255),        -- Имя пользователя
    language_code VARCHAR(10) DEFAULT 'en',  -- Код языка (en, ru, etc.)

    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Индекс для быстрого поиска по telegram_id
CREATE INDEX idx_users_telegram_id ON users(telegram_id);

-- Индекс для username (для поиска пользователей)
CREATE INDEX idx_users_username ON users(username) WHERE username IS NOT NULL;
