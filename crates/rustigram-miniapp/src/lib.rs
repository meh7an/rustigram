//! Server-side Telegram Mini App bridge for the rustigram framework.
//!
//! Provides initData validation, typed Axum extractors, a Tower middleware
//! pipeline, and an optional ts-rs type generation pipeline so Rust structs
//! stay in sync with the TypeScript `@rustigram/tma-*` packages.
//!
//! # Feature flags
//!
//! | Flag | Effect |
//! |---|---|
//! | `ts` | Enables ts-rs derives on all public types. Required for `gen-types`. |
//!
//! # Deployment modes
//!
//! ## Mode 1 — TypeScript only
//!
//! `@rustigram/tma-server` handles validation. No Rust backend required.
//! `BOT_TOKEN` lives in the TypeScript environment.
//!
//! ## Mode 2 — Rust only
//!
//! Rust is the API server. `@rustigram/tma-server` is not used on the
//! backend at all. `@rustigram/tma-core` still works on the frontend.
//! `BOT_TOKEN` lives in the Rust environment only.
//!
//! ## Mode 3 — Rust gateway + SolidStart BFF
//!
//! Rust validates and signs forwarded requests. SolidStart parses without
//! re-validating. `BOT_TOKEN` lives in Rust only. SolidStart only needs
//! `GATEWAY_SECRET`.
//!
//! See [`extract::BotTokenLayer`], [`extract::TmaGatewayLayer`], and
//! [`extract::TmaInitData`] for the full setup.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use axum::{routing::post, Router};
//! use rustigram_miniapp::{
//!     extract::TmaInitData, BotToken, BotTokenLayer,
//!     GatewaySecret, TmaGatewayLayer,
//! };
//!
//! async fn tma_handler(TmaInitData(data): TmaInitData) {
//!     println!("user: {:?}", data.user);
//! }
//!
//! // Mode 2 — Rust only
//! let app = Router::new()
//!     .route("/tma", post(tma_handler))
//!     .layer(BotTokenLayer(BotToken(
//!         std::env::var("BOT_TOKEN").unwrap(),
//!     )));
//!
//! // Mode 3 — Rust gateway, add TmaGatewayLayer for the trust signal
//! let app = Router::new()
//!     .route("/tma", post(tma_handler))
//!     .layer(TmaGatewayLayer(GatewaySecret(
//!         std::env::var("GATEWAY_SECRET").unwrap(),
//!     )))
//!     .layer(BotTokenLayer(BotToken(
//!         std::env::var("BOT_TOKEN").unwrap(),
//!     )));
//! ```
//!
//! # Security — Mini App origin restriction (Bot API 10.2)
//!
//! Telegram now disallows calling Mini App methods from an origin different
//! from the Mini App's own domain. This protection is enforced client-side by
//! Telegram and rolls out automatically to all Mini Apps on July 20, 2026 —
//! opt-out is via @BotFather, not via this crate. It requires **no changes**
//! in `rustigram-miniapp`: initData validation, the Axum extractors, and the
//! gateway layer all operate on data Telegram has already validated against
//! the registered domain, so there is nothing for this crate to additionally
//! enforce. If you opted out via @BotFather, you are responsible for ensuring
//! your Mini App has no links to untrusted sites.

/// Error types and result alias used across the crate.
pub mod error;

/// Axum extractors and Tower middleware for TMA request handling.
///
/// See [`extract::TmaInitData`] for the main extractor,
/// [`extract::BotTokenLayer`] for injecting the bot token, and
/// [`extract::TmaGatewayLayer`] for the Mode 3 gateway trust signal.
pub mod extract;

/// initData query string parsing utilities (crate-internal).
pub mod parse;

/// Typed representations of all Telegram Mini App objects.
pub mod types;

/// initData validation — HMAC-SHA256 ([`validate::validate_hmac`]) and
/// Ed25519 ([`validate::validate_ed25519`]).
pub mod validate;

pub use error::{MiniAppError, Result};
pub use extract::{BotToken, BotTokenLayer, GatewaySecret, TmaGatewayLayer, TmaInitData};
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
