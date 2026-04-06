use thiserror::Error;

/// Top-level error type for the bot framework layer.
#[derive(Debug, Error)]
pub enum BotError {
    /// Propagated from the API layer.
    #[error("API error: {0}")]
    Api(#[from] rustigram_api::Error),

    /// Handler returned an error.
    #[error("Handler error: {0}")]
    Handler(#[from] anyhow::Error),

    /// I/O error (e.g. while binding the webhook server).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The dispatcher was shut down.
    #[error("Dispatcher shut down")]
    Shutdown,
}

/// A specialised `Result` type for rustigram bot operations.
pub type BotResult<T> = std::result::Result<T, BotError>;
