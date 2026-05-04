//! Async Rust framework for the [Telegram Bot API](https://core.telegram.org/bots/api).
//!
//! rustigram re-exports all sub-crates through a single dependency and a
//! convenient [`prelude`] module covering the most common types.
//!
//! # Getting started
//!
//! Add to `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rustigram = "0.9.6"
//! tokio     = { version = "1", features = ["full"] }
//! ```
//!
//! ```rust,ignore
//! use rustigram::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let bot = Bot::new(std::env::var("BOT_TOKEN")?)?;
//!
//!     bot.dispatcher()
//!         .on(filters::command("start"), handler_fn(start))
//!         .on(filters::message(),        handler_fn(echo))
//!         .build()
//!         .polling()
//!         .await?;
//!
//!     Ok(())
//! }
//!
//! async fn start(ctx: Context) -> BotResult<()> {
//!     if let Some(r) = ctx.reply("Hello!") { r.await?; }
//!     Ok(())
//! }
//!
//! async fn echo(ctx: Context) -> BotResult<()> {
//!     if let (Some(text), Some(chat_id)) = (ctx.text(), ctx.chat_id()) {
//!         ctx.bot.send_message(chat_id, text).await?;
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Crate structure
//!
//! | Crate | Purpose |
//! |---|---|
//! | [`api`] | HTTP client and typed method builders |
//! | [`bot`] | Dispatcher, filters, handlers, FSM |
//! | [`types`] | All Bot API type definitions |
//!
pub use rustigram_api as api;
pub use rustigram_bot as bot;
pub use rustigram_types as types;

/// Commonly used types and traits from all sub-crates.
pub mod prelude {
    pub use rustigram_api::BotClient;
    pub use rustigram_bot::{
        filter::{filters, Filter, FilterExt},
        handler::handler_fn,
        state::{DialogueStorage, StateStorage},
        Bot, BotResult, Context,
    };
    pub use rustigram_types::{
        keyboard::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup},
        message::ParseMode,
        user::ChatId,
    };
}
