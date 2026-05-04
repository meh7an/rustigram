# rustigram

[![Crates.io](https://img.shields.io/crates/v/rustigram.svg)](https://crates.io/crates/rustigram)
[![Docs.rs](https://docs.rs/rustigram/badge.svg)](https://docs.rs/rustigram)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Telegram Bot API](https://img.shields.io/badge/Bot%20API-9.7-blue.svg)](https://core.telegram.org/bots/api)

A comprehensive, async-first Rust framework for the [Telegram Bot API](https://core.telegram.org/bots/api).

```toml
[dependencies]
rustigram = "0.9.6"
tokio     = { version = "1", features = ["full"] }
```

---

## Overview

rustigram is a typed, ergonomic Rust library for building Telegram bots. Every Bot API method is exposed as an awaitable builder — set only the parameters you need, then `.await`. Incoming updates are routed through a composable filter-handler pipeline running concurrently on tokio.

The framework is split into focused crates so you can depend on only what you need:

| Crate              | Purpose                                                      |
| ------------------ | ------------------------------------------------------------ |
| `rustigram-types`  | All Bot API types, fully serde-serialisable                  |
| `rustigram-api`    | HTTP client and typed method builders                        |
| `rustigram-bot`    | Dispatcher, filters, handlers, FSM, update listeners         |
| `rustigram-macros` | Procedural macros (`#[handler]`, `#[derive(DialogueState)]`) |
| `rustigram`        | Public facade re-exporting all sub-crates                    |

**Supported Bot API version:** 9.7 (May 2026)
**Minimum Rust version:** 1.75

---

## Table of Contents

- [rustigram](#rustigram)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Quick start](#quick-start)
  - [Architecture](#architecture)
  - [Receiving updates](#receiving-updates)
    - [Long polling](#long-polling)
    - [Webhook](#webhook)
  - [Sending messages](#sending-messages)
    - [Available send methods](#available-send-methods)
  - [Filters](#filters)
    - [Built-in filters](#built-in-filters)
    - [Combinators](#combinators)
    - [Custom filters](#custom-filters)
  - [Handlers](#handlers)
  - [Callback queries and inline keyboards](#callback-queries-and-inline-keyboards)
  - [Conversation state (FSM)](#conversation-state-fsm)
  - [Shared state](#shared-state)
  - [File uploads](#file-uploads)
  - [Payments and Stars](#payments-and-stars)
    - [Telegram Stars (XTR)](#telegram-stars-xtr)
    - [Handling purchases](#handling-purchases)
    - [Star balance](#star-balance)
  - [Webhook setup](#webhook-setup)
  - [Local Bot API server](#local-bot-api-server)
  - [Error handling](#error-handling)
    - [Useful error predicates](#useful-error-predicates)
  - [Examples](#examples)
  - [Running the tests](#running-the-tests)
  - [Contributing](#contributing)
  - [License](#license)

---

## Quick start

```rust
use rustigram::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let bot = Bot::new(std::env::var("BOT_TOKEN")?)?;

    bot.dispatcher()
        .on(filters::command("start"), handler_fn(start))
        .on(filters::command("help"),  handler_fn(help))
        .on(filters::message(),        handler_fn(echo))
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn start(ctx: Context) -> BotResult<()> {
    if let Some(r) = ctx.reply("Hello! Send me any message.") {
        r.await?;
    }
    Ok(())
}

async fn help(ctx: Context) -> BotResult<()> {
    if let Some(r) = ctx.reply("/start — greeting\n/help — this message") {
        r.await?;
    }
    Ok(())
}

async fn echo(ctx: Context) -> BotResult<()> {
    if let (Some(text), Some(chat_id)) = (ctx.text(), ctx.chat_id()) {
        ctx.bot.send_message(chat_id, text).await?;
    }
    Ok(())
}
```

Get your bot token from [@BotFather](https://t.me/BotFather), then:

```bash
BOT_TOKEN=your_token cargo run
```

---

## Architecture

```
Telegram
   │  getUpdates (long poll) or POST (webhook)
   ▼
Dispatcher
   ├── Route 1: filter::command("start")  ──► handler A  ─┐
   ├── Route 2: filter::callback_query()  ──► handler B    │  tokio::spawn
   ├── Route 3: filter::message()         ──► handler C    │  (concurrent)
   └── Fallback                           ──► handler D  ──┘
                                                │
                                           Context { update, bot }
                                                │
                                     ctx.bot.send_message(...).await
```

Each incoming update is dispatched in its own `tokio::spawn` task. Handlers run concurrently — a slow handler does not block others. The dispatcher evaluates routes in registration order and stops at the first match.

---

## Receiving updates

### Long polling

Long polling is the simplest mode and requires no public server.

```rust
bot.dispatcher()
    .on(filters::message(), handler_fn(my_handler))
    .build()
    .polling()
    .await?;
```

The poller automatically advances the offset after each batch, so updates are never processed twice. Transient network errors (timeout, connection reset) are retried automatically. Rate-limit responses (HTTP 429) honour the `retry_after` value from Telegram.

### Webhook

```rust
// Register the webhook URL with Telegram once
bot.client
    .set_webhook("https://example.com/")
    .secret_token("change_me")
    .await?;

// Start the axum-based server
bot.dispatcher()
    .on(filters::message(), handler_fn(my_handler))
    .build()
    .webhook("0.0.0.0:8443".parse()?)
    .await?;
```

Supported Telegram webhook ports: **443, 80, 88, 8443**.

The secret token is validated on every incoming request. Requests with a missing or incorrect `X-Telegram-Bot-Api-Secret-Token` header are rejected with HTTP 401.

---

## Sending messages

Every API method returns a builder. Call only the setters you need, then `.await`.

```rust
// Plain text
ctx.bot.send_message(chat_id, "Hello!").await?;

// With formatting and a reply
ctx.bot
    .send_message(chat_id, "<b>Bold</b> and <i>italic</i>")
    .parse_mode(ParseMode::HTML)
    .reply_to(message_id)
    .await?;

// Disable notification
ctx.bot
    .send_message(chat_id, "Silent message")
    .disable_notification(true)
    .await?;

// Photo from URL
ctx.bot
    .send_photo(chat_id, InputFile::Url("https://example.com/photo.jpg".into()))
    .caption("A photo")
    .await?;

// Forward a message
ctx.bot.forward_message(to_chat_id, from_chat_id, message_id).await?;

// Delete a message
ctx.bot.delete_message(chat_id, message_id).await?;

// Edit an existing message
ctx.bot
    .edit_message_text(chat_id, message_id, "Updated text")
    .parse_mode(ParseMode::HTML)
    .await?;
```

### Available send methods

| Method             | Description                  |
| ------------------ | ---------------------------- |
| `send_message`     | Text (up to 4096 characters) |
| `send_photo`       | Photo (.jpg, .png, etc.)     |
| `send_audio`       | Audio file treated as music  |
| `send_document`    | General file                 |
| `send_video`       | Video (MPEG4)                |
| `send_animation`   | GIF or silent H.264 video    |
| `send_voice`       | Voice note (OGG/OPUS)        |
| `send_video_note`  | Rounded-square video         |
| `send_sticker`     | .WEBP / .TGS / .WEBM sticker |
| `send_location`    | Map point, optionally live   |
| `send_contact`     | Phone contact                |
| `send_poll`        | Native poll or quiz          |
| `send_dice`        | Animated dice/emoji          |
| `send_invoice`     | Payment invoice              |
| `send_media_group` | Album of 2–10 media items    |
| `send_chat_action` | Typing indicator etc.        |
| `forward_message`  | Forward from another chat    |
| `copy_message`     | Copy without forward header  |

---

## Filters

Filters are composable predicates evaluated against an incoming `Context`. The dispatcher calls the first handler whose filter returns `true`.

### Built-in filters

```rust
use rustigram::prelude::filters;

filters::message()                    // Any Message update
filters::edited_message()             // Any EditedMessage update
filters::callback_query()             // Any CallbackQuery update
filters::inline_query()               // Any InlineQuery update
filters::command("start")             // /start (case-insensitive, strips @BotName)
filters::text("exact string")         // Exact text match
filters::text_contains("substring")   // Substring match
filters::callback_data("btn_ok")      // Exact callback data
filters::callback_data_prefix("nav:") // Callback data prefix
filters::private()                    // Private chats only
filters::group()                      // Groups and supergroups
filters::any()                        // Always passes
```

### Combinators

```rust
use rustigram::prelude::filters;
use rustigram_bot::filter::FilterExt;

// Both must match
filters::message().and(filters::private())

// Either can match
filters::private().or(filters::group())

// Invert
filters::private().not()

// Chain freely
filters::command("ban")
    .and(filters::group())
    .and(filter_fn(|ctx| is_admin(ctx)))
```

### Custom filters

```rust
use rustigram_bot::filter::{filter_fn, Filter};

let has_photo = filter_fn(|ctx| {
    ctx.message().and_then(|m| m.photo.as_ref()).is_some()
});

bot.dispatcher()
    .on(has_photo, handler_fn(photo_handler))
    .build()
    .polling()
    .await?;
```

---

## Handlers

A handler is any async function with the signature `async fn(Context) -> BotResult<()>`.

```rust
use rustigram::prelude::*;

async fn my_handler(ctx: Context) -> BotResult<()> {
    // ctx.bot       — the API client, call any method
    // ctx.update    — the raw Update
    // ctx.text()    — message text or caption
    // ctx.command() — command name without / and @BotName
    // ctx.chat_id() — chat ID from message or callback query
    // ctx.from_id() — sender user ID
    // ctx.reply(t)  — send a reply to the current message

    if let Some(r) = ctx.reply("Got it!") {
        r.await?;
    }
    Ok(())
}
```

Handlers can also be closures:

```rust
bot.dispatcher()
    .on(filters::command("ping"), handler_fn(|ctx: Context| async move {
        if let Some(r) = ctx.reply("pong") { r.await?; }
        Ok(())
    }))
    .build()
    .polling()
    .await?;
```

---

## Callback queries and inline keyboards

```rust
use rustigram::prelude::*;
use rustigram_types::keyboard::{InlineKeyboardMarkup, InlineKeyboardButton};

async fn menu(ctx: Context) -> BotResult<()> {
    let markup = InlineKeyboardMarkup::new()
        .row(vec![
            InlineKeyboardButton::callback("Yes", "answer:yes"),
            InlineKeyboardButton::callback("No",  "answer:no"),
        ])
        .row(vec![
            InlineKeyboardButton::url("Documentation", "https://core.telegram.org/bots/api"),
        ]);

    if let Some(chat_id) = ctx.chat_id() {
        ctx.bot
            .send_message(chat_id, "Make a choice:")
            .reply_markup(ReplyMarkup::InlineKeyboard(markup))
            .await?;
    }
    Ok(())
}

async fn handle_answer(ctx: Context) -> BotResult<()> {
    if let Some(q) = ctx.callback_query() {
        let answer = q.data.as_deref().unwrap_or("").trim_start_matches("answer:");
        ctx.bot
            .answer_callback_query(&q.id)
            .text(format!("You chose: {answer}"))
            .await?;
    }
    Ok(())
}

// Registration
bot.dispatcher()
    .on(filters::command("menu"),               handler_fn(menu))
    .on(filters::callback_data_prefix("answer:"), handler_fn(handle_answer))
    .build()
    .polling()
    .await?;
```

---

## Conversation state (FSM)

`DialogueStorage` tracks per-user state keyed by `(chat_id, user_id)`. State values are type-erased with `Any` so any type works without a shared trait.

```rust
use rustigram::prelude::*;
use rustigram_bot::state::DialogueStorage;
use std::sync::Arc;

#[derive(Clone)]
enum RegistrationState {
    AwaitingName,
    AwaitingEmail { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let bot = Bot::new(std::env::var("BOT_TOKEN")?)?;
    let storage = Arc::new(DialogueStorage::new());

    bot.dispatcher()
        .on(filters::command("register"), {
            let s = storage.clone();
            handler_fn(move |ctx| { let s = s.clone(); async move { start(ctx, s).await } })
        })
        .on(filters::message(), {
            let s = storage.clone();
            handler_fn(move |ctx| { let s = s.clone(); async move { step(ctx, s).await } })
        })
        .build()
        .polling()
        .await?;

    Ok(())
}

async fn start(ctx: Context, storage: Arc<DialogueStorage>) -> BotResult<()> {
    let (chat_id, user_id) = match (ctx.chat_id(), ctx.from_id()) {
        (Some(ChatId::Id(c)), Some(u)) => (c, u),
        _ => return Ok(()),
    };
    storage.set(chat_id, user_id, RegistrationState::AwaitingName);
    if let Some(r) = ctx.reply("What is your name?") { r.await?; }
    Ok(())
}

async fn step(ctx: Context, storage: Arc<DialogueStorage>) -> BotResult<()> {
    let (chat_id, user_id) = match (ctx.chat_id(), ctx.from_id()) {
        (Some(ChatId::Id(c)), Some(u)) => (c, u),
        _ => return Ok(()),
    };
    let text = match ctx.text() { Some(t) => t.to_owned(), None => return Ok(()) };

    match storage.get::<RegistrationState>(chat_id, user_id) {
        Some(RegistrationState::AwaitingName) => {
            storage.set(chat_id, user_id, RegistrationState::AwaitingEmail { name: text.clone() });
            if let Some(r) = ctx.reply(format!("Nice to meet you, {text}! What is your email?")) { r.await?; }
        }
        Some(RegistrationState::AwaitingEmail { name }) => {
            storage.remove(chat_id, user_id);
            if let Some(r) = ctx.reply(format!("Registered: {name} <{text}>")) { r.await?; }
        }
        None => {}
    }
    Ok(())
}
```

---

## Shared state

`StateStorage` is a thread-safe, type-keyed store for data shared across all handlers — database connection pools, configuration, counters, etc.

```rust
use rustigram_bot::state::StateStorage;
use std::sync::Arc;

#[derive(Clone)]
struct AppConfig {
    admin_id: i64,
    prefix: String,
}

let store = StateStorage::new();
store.insert(AppConfig { admin_id: 123456, prefix: "!".into() });
store.insert(my_db_pool);

// Inside a handler (store captured via Arc):
let config = store.get::<AppConfig>().unwrap();
```

---

## File uploads

`InputFile` accepts three variants:

```rust
use rustigram_api::BotClient;
use rustigram_types::file::InputFile;

// Re-send an existing Telegram file by file_id (most efficient)
InputFile::FileId("AgACAgIAAxkBAAI...".into())

// Download from a URL (Telegram fetches it; max 20 MB for most types)
InputFile::Url("https://example.com/document.pdf".into())

// Upload raw bytes
InputFile::Bytes {
    filename: "report.pdf".into(),
    data: std::fs::read("report.pdf")?,
    mime_type: "application/pdf".into(),
}
```

All three variants share the same API:

```rust
ctx.bot
    .send_document(chat_id, InputFile::Bytes {
        filename: "data.csv".into(),
        data: csv_bytes,
        mime_type: "text/csv".into(),
    })
    .caption("Your export")
    .await?;
```

Downloading a file sent to your bot:

```rust
let file = ctx.bot.get_file(&document.file_id).await?;
let bytes = ctx.bot.download_file(&file.file_path.unwrap()).await?;
```

---

## Payments and Stars

### Telegram Stars (XTR)

```rust
use rustigram_types::payments::LabeledPrice;

ctx.bot
    .send_invoice(
        chat_id,
        "Premium Access",
        "30 days of premium features",
        "payload_premium_30d",
        "XTR",
        vec![LabeledPrice { label: "30-day plan".into(), amount: 100 }],
    )
    .await?;
```

No `provider_token` is needed for Stars. The `amount` is in whole Stars.

### Handling purchases

```rust
async fn handle_pre_checkout(ctx: Context) -> BotResult<()> {
    if let Some(q) = ctx.update.kind.as_pre_checkout_query() {
        ctx.bot
            .answer_pre_checkout_query(&q.id, true)
            .await?;
    }
    Ok(())
}
```

### Star balance

```rust
let balance = ctx.bot.get_my_star_balance().await?;
println!("Balance: {} Stars", balance.amount);
```

---

## Webhook setup

A complete production webhook setup:

```rust
use rustigram::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("BOT_TOKEN")?;
    let webhook_url = std::env::var("WEBHOOK_URL")?;   // e.g. https://bot.example.com
    let secret = std::env::var("WEBHOOK_SECRET")?;
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or("0.0.0.0:8443".into());

    let bot = Bot::new(&token)?;

    // Register with Telegram
    bot.client
        .set_webhook(&webhook_url)
        .secret_token(&secret)
        .max_connections(40)
        .drop_pending_updates(true)
        .await?;

    tracing::info!("Webhook registered at {}", webhook_url);

    bot.dispatcher()
        .on(filters::command("start"), handler_fn(start_handler))
        .on(filters::message(),        handler_fn(echo_handler))
        .build()
        .webhook(bind_addr.parse()?)
        .await?;

    Ok(())
}
```

To remove the webhook and switch back to polling:

```rust
bot.client.delete_webhook().drop_pending_updates(true).await?;
```

---

## Local Bot API server

Run a [local Bot API server](https://github.com/tdlib/telegram-bot-api) for higher limits:

- File upload up to 2 GB (vs 50 MB)
- File download up to 2 GB (vs 20 MB)
- Up to 100,000 webhook connections
- HTTP webhooks on any port and local IP
- `getFile` returns absolute local path

Point rustigram at your local server:

```rust
use rustigram_api::ClientConfig;
use std::time::Duration;

let config = ClientConfig::new(token)?
    .api_base_url("http://localhost:8081")
    .timeout(Duration::from_secs(120));

let client = rustigram_api::BotClient::new(config)?;
```

---

## Error handling

`BotError` covers all failure modes:

```rust
use rustigram_bot::error::BotError;
use rustigram_api::Error as ApiError;

match result {
    Err(BotError::Api(ApiError::Api { error_code: 403, .. })) => {
        // Bot was blocked by the user
    }
    Err(BotError::Api(ApiError::RateLimit { retry_after })) => {
        // Flood control — wait retry_after seconds
    }
    Err(BotError::Api(ApiError::Api { error_code: 400, description, .. })) => {
        // Bad request — check description
        eprintln!("Bad request: {}", description);
    }
    Err(e) => eprintln!("Unexpected error: {}", e),
    Ok(_) => {}
}
```

Polling and webhook modes handle `RateLimit` and transient network errors automatically. Only non-recoverable errors propagate to your application.

### Useful error predicates

```rust
let err: rustigram_api::Error = ...;

err.is_rate_limit()     // true for HTTP 429
err.is_blocked()        // true when bot was blocked by user
err.is_chat_not_found() // true for "chat not found" 400s
err.retry_after()       // Some(secs) when rate-limited
```

---

## Examples

The `examples/` directory contains complete runnable bots:

| Example           | Description                               |
| ----------------- | ----------------------------------------- |
| `echo_bot`        | Minimal bot — echoes any message back     |
| `inline_keyboard` | Inline buttons and callback query routing |
| `webhook_bot`     | Production webhook server setup           |
| `state_machine`   | Multi-step conversation with FSM          |

Run any example:

```bash
BOT_TOKEN=your_token cargo run --example echo_bot
```

---

## Running the tests

The test suite requires no bot token — all tests use hand-crafted in-memory data.

```bash
cargo test --workspace
```

| Suite              | Count | What is tested                                            |
| ------------------ | ----- | --------------------------------------------------------- |
| `client_tests`     | 11    | Token validation, error types, client config              |
| `types_tests`      | 23    | Serde round-trips, helper methods, Update deserialization |
| `dispatcher_tests` | 20    | All filters, combinators, Context helpers, state storage  |

---

## Contributing

Contributions are welcome. Please open an issue before submitting a pull request for non-trivial changes.

**Guidelines:**

- Code style follows `rustfmt` defaults — run `cargo fmt` before committing
- All public items must have doc comments
- New features should include tests
- Avoid `unwrap()` in library code; propagate errors with `?`

**Running checks locally:**

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

---

## License

MIT — see [LICENSE](LICENSE).
