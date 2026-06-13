//! TMA bridge for the rustigram framework.
//!
//! Provides server-side initData validation, typed Axum extractors, and an optional
//! ts-rs-powered type generation pipeline so Rust structs stay in sync with the
//! TypeScript `@rustigram/tma-*` packages.
//!
//! # Feature flags
//!
//! - `ts` — enables ts-rs derives on all public types. Required to run `gen-types`.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use axum::Router;
//! use rustigram_miniapp::{BotToken, BotTokenLayer, extract::TmaInitData};
//!
//! async fn tma_handler(TmaInitData(init_data): TmaInitData) {
//!     println!("user: {:?}", init_data.user);
//! }
//!
//! let app = Router::new()
//!     .route("/tma", axum::routing::post(tma_handler))
//!     .layer(BotTokenLayer(BotToken(std::env::var("BOT_TOKEN").unwrap())));
//! ```

//! TMA bridge for the rustigram framework.
//!
//! Provides server-side initData validation, typed Axum extractors, and an
//! optional ts-rs type generation pipeline.
//!
//! # Feature flags
//!
//! - `ts` — enables ts-rs derives on all public types; required for `gen-types`.

/// Error types and result aliases used across the crate.
pub mod error;
/// Axum extractors for TMA initData and bot token handling.
pub mod extract;
/// Parsing utilities for TMA initData payloads and related request data.
pub mod parse;
/// Public types used by the TMA bridge.
pub mod types;
/// Validation helpers for HMAC and Ed25519 request signing.
pub mod validate;

pub use error::{MiniAppError, Result};
pub use extract::{BotToken, BotTokenLayer, TmaInitData};
pub use types::{
    ColorScheme, ContentSafeAreaInset, InitDataChatType, SafeAreaInset, ThemeParams, WebAppChat,
    WebAppChatType, WebAppInitData, WebAppUser,
};
pub use validate::{
    validate_ed25519, validate_hmac, Ed25519ValidateOpts, HmacValidateOpts, TelegramEnv,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {}
}
