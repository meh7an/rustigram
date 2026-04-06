/// Long-polling update source.
pub mod polling;
/// Axum-based webhook server.
pub mod webhook;

use crate::error::BotResult;
use rustigram_types::update::Update;

/// Trait for update sources — anything that can produce batches of [`Update`] values.
pub trait UpdateListener: Send {
    /// Fetches the next batch of updates from the source.
    fn next_batch(&mut self) -> impl std::future::Future<Output = BotResult<Vec<Update>>> + Send;
}

/// A pinned, boxed stream of individual updates.
pub type UpdateStream = std::pin::Pin<Box<dyn futures::Stream<Item = BotResult<Update>> + Send>>;
