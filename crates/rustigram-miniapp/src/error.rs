//! Error types for `rustigram-miniapp`.
//!
//! All fallible operations in this crate return [`Result<T>`], which is an
//! alias for `std::result::Result<T, MiniAppError>`. Every variant maps to an
//! appropriate HTTP status code via [`axum::response::IntoResponse`] so
//! handlers can return errors directly without manual conversion.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// All error conditions produced by `rustigram-miniapp`.
///
/// Implements [`IntoResponse`] so variants can be returned directly from Axum
/// handlers. Auth failures → `401 Unauthorized`; bad input → `400 Bad
/// Request`; missing layer → `500 Internal Server Error`.
#[derive(Debug, Error)]
pub enum MiniAppError {
    /// The initData query string was unparseable, or a required field
    /// (`auth_date`, `hash`) was absent, or a sub-object field (`user`,
    /// `chat`, `receiver`) contained invalid JSON.
    #[error("malformed initData: {0}")]
    MalformedInitData(String),

    /// The HMAC-SHA256 hash did not match the expected value. The initData was
    /// either tampered with or validated against the wrong bot token.
    #[error("invalid HMAC hash")]
    InvalidHmac,

    /// The Ed25519 signature did not verify against Telegram's published
    /// public key for the selected environment.
    #[error("invalid Ed25519 signature")]
    InvalidSignature,

    /// The `auth_date` field is older than the configured `max_age_secs`.
    /// The initData is considered stale and should be rejected.
    #[error("initData expired")]
    Expired,

    /// No [`crate::extract::BotToken`] extension was found on the request.
    /// The router is missing a [`crate::extract::BotTokenLayer`].
    #[error("missing BotToken extension — add BotTokenLayer to the router")]
    MissingBotToken,
}

/// Convenience alias used throughout this crate.
pub type Result<T> = std::result::Result<T, MiniAppError>;

impl IntoResponse for MiniAppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InvalidHmac | Self::InvalidSignature | Self::Expired => StatusCode::UNAUTHORIZED,
            Self::MalformedInitData(_) => StatusCode::BAD_REQUEST,
            Self::MissingBotToken => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
