use serde::{Deserialize, Serialize};

/// A community (a group of chats).
///
/// Several supergroups, channels, and bots linked together around a shared
/// topic or audience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    /// Unique identifier for this community. May have more than 32
    /// significant bits — safe to store as a signed 64-bit integer or
    /// double-precision float.
    pub id: i64,
    /// Name of the community.
    pub name: String,
}

/// A service message about a chat being added to a community.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityChatAdded {
    /// The new community to which the chat belongs.
    pub community: Community,
}

/// A service message about a chat being removed from a community.
///
/// Currently holds no information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommunityChatRemoved {}
