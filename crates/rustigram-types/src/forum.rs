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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

// ─── Service messages ────────────────────────────────────────────────────────
//
// Telegram delivers forum lifecycle events as service messages on [`Message`].
// Four of the six carry no data at all — their presence is the entire signal —
// but they are still objects on the wire, so each is an empty struct rather
// than a unit struct: a unit struct deserializes from `null`, not from `{}`.
//
// [`Message`]: crate::message::Message

/// Service message: a forum topic was created.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForumTopicCreated {
    /// Name of the topic.
    pub name: String,
    /// Color of the topic icon as an RGB integer.
    pub icon_color: u32,
    /// Custom emoji identifier used as the topic icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// `true` if the topic name was derived from the first message rather than
    /// set explicitly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_implicit: Option<bool>,
}

/// Service message: a forum topic was closed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForumTopicClosed {}

/// Service message: a forum topic was edited.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New icon emoji identifier, if it was changed. An empty string means the
    /// custom icon was removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// Service message: a forum topic was reopened.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForumTopicReopened {}

/// Service message: the "General" forum topic was hidden.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GeneralForumTopicHidden {}

/// Service message: the "General" forum topic was unhidden.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GeneralForumTopicUnhidden {}
