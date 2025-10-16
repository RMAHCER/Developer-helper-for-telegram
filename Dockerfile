# Multi-stage build для минимального размера образа

# Stage 1: Builder
FROM rust:1.75-alpine AS builder

# Установка зависимостей для сборки
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

WORKDIR /app

# Копируем Cargo файлы для кэширования зависимостей
COPY Cargo.toml Cargo.lock ./

# Создаем фейковый main.rs для кэширования зависимостей
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Копируем исходный код
COPY src ./src
COPY migrations ./migrations

# Собираем приложение
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.19

# Установка runtime зависимостей
RUN apk add --no-cache libgcc openssl ca-certificates

WORKDIR /app

# Копируем бинарник из builder
COPY --from=builder /app/target/release/telegram-multitool-bot /app/telegram-multitool-bot

# Копируем миграции
COPY migrations ./migrations

# Создаем директории для файлов
RUN mkdir -p /app/tmp /app/converted /app/storage

# Healthcheck (опционально)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD pgrep telegram-multitool-bot || exit 1

# Запуск
CMD ["/app/telegram-multitool-bot"]
