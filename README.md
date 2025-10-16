# Telegram Multitool Bot 🤖

Production-ready Telegram бот на Rust с функциями управления задачами, напоминаниями и конвертацией файлов.

## Возможности

- ✅ **Управление задачами (ToDo)**
  - Создание, просмотр, редактирование задач
  - Статусы и приоритеты
  - Фильтрация по статусу

- ⏰ **Система напоминаний**
  - Одноразовые и повторяющиеся напоминания
  - Напоминания для задач
  - Планировщик на Tokio tasks

- 📄 **Конвертация файлов**
  - Изображения (PNG, JPEG, WebP)
  - Документы
  - Сжатие и оптимизация

## Технологический стек

- **Rust** - основной язык
- **Tokio** - асинхронный runtime
- **Teloxide** - фреймворк для Telegram Bot API
- **SQLx** - работа с PostgreSQL
- **Railway** - платформа для деплоя

## Архитектура

Проект следует Clean Architecture с четким разделением на слои:

```
┌─────────────────────────────────────┐
│     Telegram API (teloxide)         │
└────────────────┬────────────────────┘
                 │
┌────────────────▼────────────────────┐
│  Bot Layer (Handlers & Commands)    │
└───┬──────────┬─────────────┬────────┘
    │          │             │
┌───▼───┐  ┌──▼───┐   ┌─────▼─────┐
│ ToDo  │  │Conv. │   │ Reminder  │
│Service│  │Service│   │ Service   │
└───┬───┘  └──┬───┘   └─────┬─────┘
    │         │             │
┌───▼─────────▼─────────────▼──────┐
│   Repository Layer (SQLx)        │
└────────────┬─────────────────────┘
             │
┌────────────▼─────────────────────┐
│      PostgreSQL Database         │
└──────────────────────────────────┘
```

## Быстрый старт

### Предварительные требования

- Rust 1.75+
- PostgreSQL 14+ (или SQLite для разработки)
- Токен бота от [@BotFather](https://t.me/BotFather)

### Установка

1. Клонируйте репозиторий:
```bash
git clone <repo-url>
cd telegram-bot
```

2. Скопируйте `.env.example` в `.env`:
```bash
cp .env.example .env
```

3. Настройте переменные окружения в `.env`:
```env
BOT_TOKEN=your_bot_token_from_botfather
DATABASE_URL=postgres://user:password@localhost:5432/telegram_bot
```

4. Запустите миграции базы данных:
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

5. Запустите бота:
```bash
cargo run --release
```

## Команды бота

### Управление задачами
- `/addtodo <текст>` - добавить новую задачу
- `/listtodos` - показать все задачи
- `/completetodo <id>` - отметить задачу выполненной
- `/deletetodo <id>` - удалить задачу

### Напоминания
- `/remind <время> <текст>` - установить напоминание
  - Примеры: `/remind 15m Проверить почту`, `/remind 2h Встреча`
  - Форматы: `5m` (минуты), `2h` (часы), `1d` (дни)
- `/listreminders` - показать активные напоминания
- `/cancelreminder <id>` - отменить напоминание

### Общее
- `/start` - начать работу с ботом
- `/help` - показать справку

## Структура проекта

```
telegram-bot/
├── src/
│   ├── main.rs              # Entry point
│   ├── config.rs            # Конфигурация
│   ├── error.rs             # Обработка ошибок
│   ├── bot/                 # Telegram бот
│   │   ├── handlers.rs      # Обработчики команд
│   │   ├── commands.rs      # Реализация команд
│   │   ├── keyboards.rs     # Клавиатуры
│   │   └── state.rs         # FSM состояния
│   ├── todo/                # ToDo функционал
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   └── models.rs
│   ├── reminder/            # Напоминания
│   │   ├── scheduler.rs     # Планировщик
│   │   ├── notifier.rs      # Отправка уведомлений
│   │   ├── service.rs
│   │   └── repository.rs
│   ├── converter/           # Конвертация файлов
│   │   └── processors/
│   ├── db/                  # База данных
│   │   ├── pool.rs
│   │   └── migrations.rs
│   └── shared/              # Общие утилиты
├── migrations/              # SQL миграции
├── Dockerfile               # Docker образ
├── railway.toml             # Конфиг Railway
└── README.md
```

## Деплой

### Railway

1. Установите [Railway CLI](https://docs.railway.app/develop/cli):
```bash
npm install -g @railway/cli
```

2. Войдите в аккаунт:
```bash
railway login
```

3. Инициализируйте проект:
```bash
railway init
```

4. Добавьте PostgreSQL:
```bash
railway add postgresql
```

5. Установите переменные окружения:
```bash
railway variables set BOT_TOKEN=your_bot_token
```

6. Деплой:
```bash
railway up
```

### Docker

```bash
# Сборка образа
docker build -t telegram-bot .

# Запуск контейнера
docker run -d \
  -e BOT_TOKEN=your_token \
  -e DATABASE_URL=postgres://... \
  --name telegram-bot \
  telegram-bot
```

## Разработка

### Запуск с логированием
```bash
RUST_LOG=debug cargo run
```

### Форматирование кода
```bash
cargo fmt
```

### Линтинг
```bash
cargo clippy
```

### Тесты
```bash
cargo test
```

## Конфигурация

Все настройки задаются через переменные окружения (см. `.env.example`):

| Переменная | Описание | По умолчанию |
|-----------|----------|--------------|
| `BOT_TOKEN` | Токен Telegram бота | - |
| `DATABASE_URL` | URL базы данных | - |
| `ENVIRONMENT` | Окружение (development/production) | development |
| `LOG_LEVEL` | Уровень логирования | info |
| `DB_MAX_CONNECTIONS` | Макс. соединений с БД | 10 |

## Система напоминаний

Планировщик напоминаний работает на Tokio tasks:

1. При запуске загружаются все pending напоминания
2. Для каждого создается Tokio task с задержкой
3. Background задача проверяет новые напоминания каждые 30 сек
4. Поддерживает до ~10K одновременных напоминаний

## Миграции базы данных

Миграции находятся в `migrations/` и выполняются автоматически при запуске.

Создание новой миграции:
```bash
sqlx migrate add <название>
```

Применение миграций:
```bash
sqlx migrate run
```

Откат последней миграции:
```bash
sqlx migrate revert
```

## Лицензия

MIT

## Автор

Создано как production-ready стартап проект на Rust.

---

**Полезные ссылки:**
- [Teloxide Documentation](https://docs.rs/teloxide/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Railway Docs](https://docs.railway.app/)
