use serde::{Deserialize, Serialize};

/// Represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,

    /// `true` if this user is a bot.
    pub is_bot: bool,

    /// User's or bot's first name.
    pub first_name: String,

    /// User's or bot's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// User's or bot's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// IETF language tag of the user's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// `true` if this user is a Telegram Premium user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,

    /// `true` if this user added the bot to the attachment menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,

    /// Bots only — `true` if the bot can be invited to groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,

    /// Bots only — `true` if privacy mode is disabled for the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,

    /// Bots only — `true` if the bot supports inline queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,

    /// Bots only — `true` if the bot can be connected to a Telegram Business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,

    /// Bots only — `true` if the bot has a main Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,

    /// Bots only — `true` if the bot has forum topic mode enabled in private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_topics_enabled: Option<bool>,

    /// Bots only — `true` if the bot allows users to create and delete topics in private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_users_to_create_topics: Option<bool>,

    /// Bots only — `true` if other bots can be created to be controlled by this bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_bots: Option<bool>,

    /// Bots only — `true` if the bot supports guest queries from chats it is not a member of.
    /// Returned only in `getMe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_guest_queries: Option<bool>,
}

impl User {
    /// Returns a human-readable display name, preferring `first_name + last_name`.
    #[must_use]
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last) => format!("{} {}", self.first_name, last),
            None => self.first_name.clone(),
        }
    }

    /// Returns a `@username` mention string if available.
    #[must_use]
    pub fn mention(&self) -> Option<String> {
        self.username.as_ref().map(|u| format!("@{u}"))
    }
}

/// Container for a user's profile photos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: u32,

    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<crate::file::PhotoSize>>,
}

/// Container for a user's profile audios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileAudios {
    /// Total number of profile audios.
    pub total_count: u32,

    /// Requested profile audios.
    pub audios: Vec<crate::file::Audio>,
}

/// Bot command scope — defines where a specific list of commands applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    /// Default scope — used when no more specific scope is applicable.
    Default,
    /// Covers all private chats.
    AllPrivateChats,
    /// Covers all group and supergroup chats.
    AllGroupChats,
    /// Covers all group and supergroup chat administrators.
    AllChatAdministrators,
    /// Covers a specific chat.
    Chat {
        /// Unique identifier or username of the target chat.
        chat_id: ChatId,
    },
    /// Covers all administrators of a specific group or supergroup.
    ChatAdministrators {
        /// Unique identifier or username of the target chat.
        chat_id: ChatId,
    },
    /// Covers a specific member of a group or supergroup.
    ChatMember {
        /// Unique identifier or username of the target chat.
        chat_id: ChatId,
        /// Unique identifier of the target user.
        user_id: i64,
    },
}

/// Represents a bot command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command (1–32 characters, lowercase, alphanumeric + underscore).
    pub command: String,
    /// Description of the command (3–256 characters).
    pub description: String,
}

/// Flexible chat identifier — either a numeric ID or a `@username`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    /// Numeric chat ID (can be negative for groups/channels).
    Id(i64),
    /// Chat `@username`.
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        Self::Id(id)
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> Self {
        Self::Username(username.to_owned())
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> Self {
        Self::Username(username)
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{id}"),
            Self::Username(u) => write!(f, "{u}"),
        }
    }
}

/// The bot's display name.
///
/// Returned by [`getMyName`](https://core.telegram.org/bots/api#getmyname).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotName {
    /// The bot's name.
    pub name: String,
}

/// The bot's description shown in the chat when the chat is empty.
///
/// Returned by [`getMyDescription`](https://core.telegram.org/bots/api#getmydescription).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotDescription {
    /// The bot's description.
    pub description: String,
}

/// The bot's short description shown on the profile page and in sharing links.
///
/// Returned by [`getMyShortDescription`](https://core.telegram.org/bots/api#getmyshortdescription).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description.
    pub short_description: String,
}

/// Represents the rights of an administrator in a chat.
///
/// Used by [`setMyDefaultAdministratorRights`](https://core.telegram.org/bots/api#setmydefaultadministratorrights)
/// and [`getMyDefaultAdministratorRights`](https://core.telegram.org/bots/api#getmydefaultadministratorrights).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
    /// `true` if the user's presence in the chat is hidden.
    pub is_anonymous: bool,
    /// `true` if the administrator can access the chat event log, boost list, etc.
    pub can_manage_chat: bool,
    /// `true` if the administrator can delete messages of other users.
    pub can_delete_messages: bool,
    /// `true` if the administrator can manage video chats.
    pub can_manage_video_chats: bool,
    /// `true` if the administrator can restrict, ban, or unban chat members.
    pub can_restrict_members: bool,
    /// `true` if the administrator can promote members to administrators.
    pub can_promote_members: bool,
    /// `true` if the user is allowed to change the chat title, photo, and other settings.
    pub can_change_info: bool,
    /// `true` if the user is allowed to invite new users to the chat.
    pub can_invite_users: bool,
    /// `true` if the administrator can post stories to the chat.
    pub can_post_stories: bool,
    /// `true` if the administrator can edit stories posted by other users.
    pub can_edit_stories: bool,
    /// `true` if the administrator can delete stories posted by other users.
    pub can_delete_stories: bool,
    /// `true` if the administrator can post messages in the channel; channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// `true` if the administrator can edit messages of other users and pin messages; channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// `true` if the user is allowed to pin messages; groups and supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// `true` if the user is allowed to create, rename, close, and reopen forum topics; supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// `true` if the administrator can manage direct messages and decline suggested posts; channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// `true` if the administrator can edit the tags of regular members; groups and supergroups only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_tags: Option<bool>,
}

/// Describes the access settings of a bot.
///
/// Returned by [`getManagedBotAccessSettings`](https://core.telegram.org/bots/api#getmanagedbotaccesssettings).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotAccessSettings {
    /// `true` if only selected users can access the bot. The bot's owner can always access it.
    pub is_access_restricted: bool,

    /// The list of other users who have access to the bot if the access is restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_users: Option<Vec<User>>,
}
