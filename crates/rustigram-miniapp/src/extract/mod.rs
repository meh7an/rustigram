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

// `::axum` is spelled absolutely because this crate has its own `axum`
// submodule, which would otherwise shadow the dependency.
/// Reads the raw initData string from request headers.
///
/// Checks `X-Tma-Init-Data` first, then the `Authorization: tma <data>` form.
///
/// Both [`TmaInitData`] and [`TmaGatewayLayer`] go through this one function.
/// They previously each had their own copy and had drifted — the gateway read
/// `X-Telegram-Init-Data`, a header nothing else in the crate ever sets — so the
/// layer never signed a request the extractor would accept, and did so silently.
pub(crate) fn init_data_from_headers(headers: &::axum::http::HeaderMap) -> Option<String> {
    if let Some(value) = headers.get("x-tma-init-data") {
        return value.to_str().ok().map(str::to_owned);
    }
    headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("tma "))
        .map(str::to_owned)
}

/// Newtype wrapping the bot token string.
///
/// Injected into every request by [`BotTokenLayer`] and read by [`TmaInitData`]
/// during extraction. Cloning is cheap — the `Arc` is inside if you need it,
/// but for most bots a plain `String` is fine.
#[derive(Debug, Clone)]
pub struct BotToken(pub String);
