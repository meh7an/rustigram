//! High-level bot framework built on top of [`rustigram-api`].
//!
//! This crate provides everything needed to receive and route Telegram updates:
//!
//! - [`Bot`] — entry point, owns the [`crate::bot::Bot::client`] field and creates a dispatcher
//! - [`Dispatcher`] — routes updates to handlers via composable filters
//! - [`Context`] — passed to every handler, provides access to the update and the API client
//! - [`filter`] — built-in and composable filter predicates
//! - [`handler`] — the [`handler::Handler`] trait and [`handler::handler_fn`] wrapper
//! - [`state`] — [`state::StateStorage`] for shared data and [`state::DialogueStorage`] for FSM
//! - [`update_listener`] — long-polling and axum-based webhook implementations
//!
//!  # Quick start
//!
//! ```rust,ignore
//! use rustigram_bot::{Bot, Context, BotResult};
//! use rustigram_bot::filter::filters;
//! use rustigram_bot::handler::handler_fn;
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
//! # Concurrency model
//!
//! Each incoming update is dispatched in its own [`tokio::spawn`] task.
//! Handlers run concurrently — a slow handler never blocks others.
//! The dispatcher evaluates routes in registration order and stops at the
//! first matching filter.
#![warn(missing_docs)]
/// Entry-point type for creating and running a bot.
pub mod bot;
/// Handler context passed to every update handler.
pub mod context;
/// Update dispatcher and routing.
pub mod dispatcher;
/// Error types for the bot framework layer.
pub mod error;
/// Composable update filters.
pub mod filter;
/// Handler trait and function wrapper.
pub mod handler;
/// Shared and per-user state storage.
pub mod state;
/// Update listeners (long polling and webhook).
pub mod update_listener;

pub use bot::Bot;
pub use context::Context;
pub use dispatcher::{Dispatcher, DispatcherBuilder};
pub use error::{BotError, BotResult};
pub use filter::{Filter, FilterExt};
pub use handler::{Handler, HandlerFn};
pub use state::StateStorage;
