use crate::dispatcher::DispatcherBuilder;
use crate::error::BotResult;
use rustigram_api::{BotClient, ClientConfig};

/// Entry point for building and running a Telegram bot.
///
/// `Bot` owns the [`BotClient`] and provides a convenient starting point
/// for setting up the dispatcher.
pub struct Bot {
    /// The underlying API client.
    pub client: BotClient,
}

impl Bot {
    /// Creates a `Bot` from a bot token.
    ///
    /// # Errors
    /// Returns `Err` if the token format is invalid or the HTTP client fails to build.
    pub fn new(token: impl Into<String>) -> BotResult<Self> {
        let client = BotClient::from_token(token).map_err(crate::error::BotError::Api)?;
        Ok(Self { client })
    }

    /// Creates a `Bot` from a [`ClientConfig`] for advanced configuration.
    ///
    /// # Errors
    /// Returns `Err` if the HTTP client fails to build.
    pub fn from_config(config: ClientConfig) -> BotResult<Self> {
        let client = BotClient::new(config).map_err(crate::error::BotError::Api)?;
        Ok(Self { client })
    }

    /// Returns a [`DispatcherBuilder`] pre-configured with this bot's client.
    pub fn dispatcher(&self) -> DispatcherBuilder {
        crate::dispatcher::Dispatcher::builder(self.client.clone())
    }
}
