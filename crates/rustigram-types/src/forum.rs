use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A topic in a Telegram forum supergroup.
///
/// Forum topics are the individual discussion threads inside a supergroup
/// with `is_forum = true`. Returned by [`createForumTopic`](https://core.telegram.org/bots/api#createforumtopic).
pub struct ForumTopic {
    /// Unique identifier of the forum topic thread.
    pub message_thread_id: i64,
    /// Name of the topic (1–128 characters).
    pub name: String,
    /// Color of the topic icon as an RGB integer.
    pub icon_color: u32,
    /// Custom emoji identifier used as the topic icon.
    pub icon_custom_emoji_id: Option<String>,
}
