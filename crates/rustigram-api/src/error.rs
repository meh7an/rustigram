use thiserror::Error;

/// Errors that can occur when interacting with the Telegram Bot API.
#[derive(Debug, Error)]
pub enum Error {
    /// The Telegram API returned `ok: false`.
    #[error("Telegram API error {error_code}: {description}")]
    Api {
        /// The HTTP error code returned by Telegram.
        error_code: u16,
        /// Human-readable description of the error.
        description: String,
        /// If set, the group was migrated to a supergroup with this ID.
        migrate_to_chat_id: Option<i64>,
        /// If set, retry the request after this many seconds.
        retry_after: Option<u32>,
    },

    /// HTTP transport or network error.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Failed to serialise the request body.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The response body could not be decoded.
    #[error("Response decode error: {0}")]
    Decode(String),

    /// Rate limit hit — caller should wait `retry_after` seconds before retrying.
    #[error("Rate limited: retry after {retry_after}s")]
    RateLimit {
        /// Number of seconds to wait before retrying.
        retry_after: u32,
    },

    /// The provided token format is invalid.
    #[error("Invalid bot token format")]
    InvalidToken,

    /// A required parameter was missing.
    #[error("Missing required parameter: {0}")]
    MissingParam(&'static str),
}

impl Error {
    /// Returns `true` if this is a flood-control error (429).
    #[must_use]
    pub fn is_rate_limit(&self) -> bool {
        matches!(
            self,
            Self::RateLimit { .. }
                | Self::Api {
                    error_code: 429,
                    ..
                }
        )
    }

    /// Extracts the `retry_after` value if available.
    #[must_use]
    pub fn retry_after(&self) -> Option<u32> {
        match self {
            Self::RateLimit { retry_after } => Some(*retry_after),
            Self::Api { retry_after, .. } => *retry_after,
            _ => None,
        }
    }

    /// Returns `true` if the bot is blocked by the user.
    #[must_use]
    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Api { error_code: 403, description, .. }
            if description.contains("bot was blocked"))
    }

    /// Returns `true` if the chat was not found.
    #[must_use]
    pub fn is_chat_not_found(&self) -> bool {
        matches!(self, Self::Api { error_code: 400, description, .. }
            if description.contains("chat not found"))
    }
}

/// A specialised `Result` type for rustigram API operations.
pub type Result<T> = std::result::Result<T, Error>;
