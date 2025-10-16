-- Initial schema setup
-- Создание базовых расширений PostgreSQL

-- UUID extension (если понадобится в будущем)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Индексы для полнотекстового поиска (если понадобится)
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
