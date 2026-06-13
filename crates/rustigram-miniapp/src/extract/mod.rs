//! Axum extractors and Tower middleware for TMA request handling.
//!
//! Add [`BotTokenLayer`] to your router once, then declare [`TmaInitData`]
//! as a handler parameter to get verified, typed initData automatically.

pub mod axum;
pub mod gateway;
pub mod layer;

pub use axum::TmaInitData;
pub use gateway::{GatewaySecret, TmaGatewayLayer};
pub use layer::BotTokenLayer;

/// Newtype wrapping the bot token string.
///
/// Injected into every request by [`BotTokenLayer`] and read by [`TmaInitData`]
/// during extraction. Cloning is cheap — the `Arc` is inside if you need it,
/// but for most bots a plain `String` is fine.
#[derive(Debug, Clone)]
pub struct BotToken(pub String);
