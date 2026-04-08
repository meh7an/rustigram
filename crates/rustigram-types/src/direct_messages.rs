use serde::{Deserialize, Serialize};

use crate::user::User;

/// A topic in a channel's direct messages chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic.
    pub topic_id: i64,
    /// The user that created the topic; currently always present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

/// Service message: the price for paid messages in a direct messages chat changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessagePriceChanged {
    /// `true` if direct messages are enabled for the channel chat.
    pub are_direct_messages_enabled: bool,
    /// The new number of Telegram Stars users must pay per direct message.
    ///
    /// Does not apply to users exempted by administrators. Defaults to `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_star_count: Option<i64>,
}

/// Service message: the price for paid messages in the chat changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidMessagePriceChanged {
    /// The new number of Telegram Stars non-administrator users must pay per message
    /// in the supergroup chat.
    pub paid_message_star_count: i64,
}
