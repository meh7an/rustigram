use rustigram_api::BotClient;
use rustigram_types::update::Update;
use tracing::debug;

use crate::error::BotResult;

/// Long-polling update source.
///
/// Maintains an internal `offset` so each call to [`LongPoller::next_batch`]
/// only returns unseen updates. Uses a 30-second server-side timeout.
pub struct LongPoller {
    client: BotClient,
    offset: i64,
    timeout: u32,
    limit: u8,
    allowed_updates: Option<Vec<String>>,
}

impl LongPoller {
    /// Creates a new `LongPoller` with sensible defaults.
    pub fn new(client: BotClient) -> Self {
        Self {
            client,
            offset: 0,
            timeout: 30,
            limit: 100,
            allowed_updates: None,
        }
    }

    /// Overrides the long-poll server-side timeout (seconds).
    pub fn timeout(mut self, secs: u32) -> Self {
        self.timeout = secs;
        self
    }

    /// Restricts the update types to receive.
    pub fn allowed_updates(mut self, types: Vec<impl Into<String>>) -> Self {
        self.allowed_updates = Some(types.into_iter().map(Into::into).collect());
        self
    }

    /// Fetches the next batch of updates from the Telegram API.
    ///
    /// This call blocks for up to `timeout` seconds if no updates are
    /// available (long polling). On success, the internal offset is
    /// automatically advanced.
    pub async fn next_batch(&mut self) -> BotResult<Vec<Update>> {
        let mut req = self
            .client
            .get_updates()
            .timeout(self.timeout)
            .limit(self.limit);

        if self.offset != 0 {
            req = req.offset(self.offset);
        }

        if let Some(ref types) = self.allowed_updates {
            req = req.allowed_updates(types.clone());
        }

        let updates = req.await.map_err(crate::error::BotError::Api)?;

        if let Some(last) = updates.last() {
            // Advance the offset to acknowledge processed updates.
            self.offset = last.update_id + 1;
            debug!(
                "Polled {} updates, next offset: {}",
                updates.len(),
                self.offset
            );
        }

        Ok(updates)
    }
}
