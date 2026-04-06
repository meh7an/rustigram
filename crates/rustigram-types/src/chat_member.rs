use crate::chat::ChatPermissions;
use crate::user::User;
use serde::{Deserialize, Serialize};

/// Information about one member of a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChatMember {
    /// The chat owner.
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    /// An administrator with custom privileges.
    Administrator(ChatMemberAdministrator),
    /// A regular chat member.
    Member(ChatMemberMember),
    /// A member with restricted permissions.
    Restricted(ChatMemberRestricted),
    /// A user who left the chat.
    Left(ChatMemberLeft),
    /// A banned (kicked) user.
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}

impl ChatMember {
    /// Returns the user from any variant.
    #[must_use]
    pub fn user(&self) -> &User {
        match self {
            Self::Owner(m) => &m.user,
            Self::Administrator(m) => &m.user,
            Self::Member(m) => &m.user,
            Self::Restricted(m) => &m.user,
            Self::Left(m) => &m.user,
            Self::Banned(m) => &m.user,
        }
    }
}

/// Chat owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    /// The user.
    pub user: User,
    /// `true` if the user's presence in the chat is hidden.
    pub is_anonymous: bool,
    /// Custom title shown instead of "Owner".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Chat administrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberAdministrator {
    /// The user.
    pub user: User,
    /// `true` if the bot can edit this administrator's privileges.
    pub can_be_edited: bool,
    /// `true` if the admin's identity is hidden.
    pub is_anonymous: bool,
    /// Allows managing the chat.
    pub can_manage_chat: bool,
    /// Allows deleting messages of other users.
    pub can_delete_messages: bool,
    /// Allows managing video chats.
    pub can_manage_video_chats: bool,
    /// Allows restricting, banning, or unbanning members.
    pub can_restrict_members: bool,
    /// Allows the admin to add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user).
    pub can_promote_members: bool,
    /// Allows changing the chat title, photo, and other settings.
    pub can_change_info: bool,
    /// Allows inviting new users.
    pub can_invite_users: bool,
    #[serde(default)]
    /// Allows posting messages in channels.
    pub can_post_messages: bool,
    #[serde(default)]
    /// Allows editing messages in channels.
    pub can_edit_messages: bool,
    /// Allows pinning messages.
    #[serde(default)]
    pub can_pin_messages: bool,
    /// Allows managing forum topics.
    #[serde(default)]
    pub can_manage_topics: bool,
    /// Allows posting stories.
    #[serde(default)]
    pub can_post_stories: bool,
    /// Allows editing stories.
    #[serde(default)]
    pub can_edit_stories: bool,
    /// Allows deleting stories.
    #[serde(default)]
    pub can_delete_stories: bool,
    /// Allows managing direct messages.
    #[serde(default)]
    pub can_manage_direct_messages: bool,
    /// Allows managing tags.
    #[serde(default)]
    pub can_manage_tags: bool,
    /// Custom title shown instead of "Administrator".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

/// Regular chat member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberMember {
    /// The user.
    pub user: User,
    /// Date when the membership expires, as a Unix timestamp. `0` means permanent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

/// Restricted chat member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberRestricted {
    /// The user.
    pub user: User,
    /// `true` if the user is still a member of the chat.
    pub is_member: bool,
    /// The current permissions of the restricted user.
    #[serde(flatten)]
    pub permissions: ChatPermissions,
    /// Unix timestamp when the restriction ends. `0` means permanent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A chat member who left or was removed from the chat.
pub struct ChatMemberLeft {
    /// The user.
    pub user: User,
}

/// Banned (kicked) chat member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    /// The user.
    pub user: User,
    /// Unix timestamp when the ban ends. `0` means permanent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
