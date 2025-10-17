#Telegram Multitool Bot 🤖

A production-ready Telegram bot written in Rust with task management, reminders, and file conversion features.

## Features

- ✅ **Task Management (ToDo)**
- Create, view, and edit tasks
- Statuses and priorities
- Filter by status

- ⏰ **Reminder System**
- One-time and recurring reminders
- Task reminders
- Tokio Tasks scheduler

- 📄 **File Conversion**
- Images (PNG, JPEG, WebP)
- Documents
- Compression and optimization

## Tech Stack

- **Rust** - main language
- **Tokio** - asynchronous runtime
- **Teloxide** - framework for Telegram Bot API
- **SQLx** - work with PostgreSQL
- **Railway** - deployment platform

## Architecture

The project follows a Clean Architecture with a clear separation of Layers:

```
┌───────────────────────────────────────────┐
│ Telegram API (teloxide) │
└─────────────────┬────────────────────┘ 
│
┌────────────────▼────────────────────┐
│ Bot Layer (Handlers & Commands) │
└───┬──────────┬─────────────┬────────┘ 
│ │ │
┌───▼───┐ ┌──▼───┐ ┌─────▼─────┐
│ ToDo │ │Conv. │ │ Reminder │
│Service│ │Service│ │ Service │
└───┬───┘ └──┬───┘ └─────┬─────┘ 
│ │ │
┌───▼─────────▼─────────────▼──────┐
│ Repository Layer (SQLx) │
└────────────┬─────────────────────┘
│
┌────────────▼────────────────────────┐
│ PostgreSQL Database │
└────────────────────────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.75+
- PostgreSQL 14+ (or SQLite for development)
- Bot token from [@BotFather](https://t.me/BotFather)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/RMAHCER/Developer-helper-for-telegram
cd Developer-helper-for-telegram
```

2. Copy `.env.example` to `.env`:
```bash
cp .env.example .env
```

3. Set environment variables in `.env`:
```env
BOT_TOKEN=your_bot_token_from_botfather
DATABASE_URL=postgres://user:password@localhost:5432/telegram_bot
```

4. Run Database migration:
```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

5. Run the bot:
```bash
cargo run --release
```

## Bot commands

### Task management
- `/addtodo <text>` - add a new task
- `/listtodos` - show all tasks
- `/completetodo <id>` - mark a task as completed
- `/deletetodo <id>` - delete a task

### Reminders
- `/remind <time> <text>` - set a reminder
- Examples: `/remind 15m Check mail`, `/remind 2h Meeting`
- Formats: `5m` (minutes), `2h` (hours), `1d` (days)
- `/listreminders` - show active reminders
- `/cancelreminder <id>` - cancel reminder

### General
- `/start` - start working with the bot
- `/help` - show help

## Project Structure

```
telegram-bot/
├── src/
│ ├── main.rs # Entry point
│ ├── config.rs # Configuration
│ ├── error.rs # Error handling
│ ├── bot/ # Telegram bot
│ │ ├── handlers.rs # Command handlers
│ │ ├── commands.rs # Command implementation
│ │ ├── keyboards.rs # Keyboards
│ │ └── state.rs # State FSM
│ ├── todo/ # ToDo functionality
│ │ ├── service.rs
│ │ ├── repository.rs
│ │ └── models.rs
│ ├── reminder/ # Reminders
│ │ ├── scheduler.rs # Scheduler
│ │ ├── notifier.rs # Sending notifications
│ │ ├── service.rs
│ │ └── repository.rs
│ ├── converter/ # File conversion
│ │ └── processors/
│ ├── db/ # Database
│ │ ├── pool.rs
│ │ └── migrations.rs
│ └── shared/ # Shared utilities
├── migrations/ # SQL migrations
├── Dockerfile # Docker image
├── railway.toml # Railway config
└── README.md
```

## Deploy

### Railway

1. Install the [Railway CLI](https://docs.railway.app/develop/cli):
```bash
npm install -g @railway/cli
```

2. Log in Account:
```bash
railway login
```

3. Initialize the project:
```bash
railway init
```

4. Add PostgreSQL:
```bash
railway add postgresql
```

5. Set environment variables:
```bash
railway variables set BOT_TOKEN=your_bot_token
```

6. Deploy:
```bash
railway up
```

### Docker

```bash
# Build the image
docker build -t telegram-bot .

# Starting a container
docker run -d \
-e BOT_TOKEN=your_token \
-e DATABASE_URL=postgres://... \
--name telegram-bot \
telegram-bot
```

## Development

### Running with logging
```bash
RUST_LOG=debug cargo run
```

### Code formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Tests
```bash
cargo test
```

## Configuration

All settings are set via environment variables (see `.env.example`):

| Variable | Description | Default |
|-----------|-----------|---------------|
| `BOT_TOKEN` | Telegram bot token | - |
| `DATABASE_URL` | Database URL | - |
| `ENVIRONMENT` | Environment (development/production) | development |
| `LOG_LEVEL` | Level
