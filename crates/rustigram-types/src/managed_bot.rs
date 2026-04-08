use serde::{Deserialize, Serialize};

use crate::user::User;

/// Service message: a new bot was created to be managed by the current bot.
///
/// Delivered inside [`Message`](crate::message::Message) via the
/// `managed_bot_created` field. The bot's token can be fetched with
/// `getManagedBotToken`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedBotCreated {
    /// Information about the newly created managed bot.
    pub bot: User,
}
