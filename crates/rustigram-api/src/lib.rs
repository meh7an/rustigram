//! HTTP client and typed method builders for the Telegram Bot API.
//!
//! The central type is [`BotClient`], which is cheap to clone (all state is
//! behind an `Arc`) and safe to share across tasks. Every Bot API method is
//! exposed as a builder that returns a future:
//!
//! ```rust,ignore
//! let message = client
//!     .send_message(chat_id, "Hello!")
//!     .parse_mode(ParseMode::HTML)
//!     .disable_notification(true)
//!     .await?;
//! ```
//!
//! # Error handling
//!
//! All methods return [`Result<T>`]. The [`Error`] type covers API-level
//! failures (bad token, flood control, missing permissions), transport errors,
//! and serialisation failures. Rate-limit errors (HTTP 429) are automatically
//! retried by the polling engine in `rustigram-bot`; individual API calls
//! surface them as [`Error::RateLimit`].
//!
//! # Feature flags
//!
//! | Feature | Default | Description |
//! |---|---|---|
//! | `rustls` | yes | TLS via rustls |
//!
//! # Re-exports
//!
//! The [`methods`] module re-exports every builder type. You generally do not
//! need to import them directly — call the corresponding method on
//! [`BotClient`] and the builder is returned.
#![warn(missing_docs)]
/// Core HTTP client and configuration.
pub mod client;

/// Error types for API operations.
pub mod error;

/// Typed method builders for every Telegram Bot API endpoint.
pub mod methods;

pub use client::{BotClient, ClientConfig};
pub use error::{Error, Result};
