//! Telegram Mini App initData types.
//!
//! Typed representations of the objects Telegram injects into
//! `window.Telegram.WebApp.initData` when a Mini App is opened. All types in
//! this module mirror the shapes defined in `@rustigram/tma-core`'s Zod
//! schemas and are validated server-side by [`crate::validate_hmac`] or
//! [`crate::validate_ed25519`] before being handed to application code.
//!
//! @see <https://core.telegram.org/bots/webapps#webappinitdata>

use serde::{Deserialize, Serialize};

/// Type of group chat represented by a [`WebAppChat`].
///
/// Private chats and the "sender" context do not appear here — see
/// [`InitDataChatType`] for the full launch-context enum.
///
/// @see <https://core.telegram.org/bots/webapps#webappchat>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub enum WebAppChatType {
    /// Group chat.
    Group,
    /// Supergroup chat.
    Supergroup,
    /// Broadcast channel.
    Channel,
}

/// Type of chat from which the Mini App was opened, as reported in
/// [`WebAppInitData::chat_type`].
///
/// Wider than [`WebAppChatType`] — includes `sender` and `private` contexts
/// that are not valid for the `chat` field itself.
///
/// @see <https://core.telegram.org/bots/webapps#webappinitdata>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub enum InitDataChatType {
    /// Direct message from the attachment menu entry point.
    Sender,
    /// Private chat.
    Private,
    /// Group chat.
    Group,
    /// Supergroup chat.
    Supergroup,
    /// Broadcast channel.
    Channel,
}

/// Contains data about the Mini App user.
///
/// Returned as the `user` or `receiver` field of [`WebAppInitData`].
///
/// # Flag fields
///
/// `is_premium`, `added_to_attachment_menu`, and `allows_write_to_pm` are
/// `Option<bool>` but Telegram only ever sends them when `true`. An absent
/// field is semantically `false`. `Some(false)` will never arrive from the
/// platform.
///
/// @see <https://core.telegram.org/bots/webapps#webappuser>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct WebAppUser {
    /// A unique identifier for the user or bot. Has at most 52 significant
    /// bits, so a 64-bit integer type is safe for storing this value.
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub id: i64,

    /// `true` if this user is a bot. Returned only in the `receiver` field —
    /// never present on the initiating user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub is_bot: Option<bool>,

    /// First name of the user or bot.
    pub first_name: String,

    /// Last name of the user or bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub last_name: Option<String>,

    /// Username of the user or bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub username: Option<String>,

    /// IETF language tag of the user's language. Returned only in the `user`
    /// field, not in `receiver`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub language_code: Option<String>,

    /// `true` if this user is a Telegram Premium subscriber. Omitted entirely
    /// when `false` — Telegram never sends the field with a `false` value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub is_premium: Option<bool>,

    /// `true` if this user has added the bot to the attachment menu. Omitted
    /// entirely when `false`.
    ///
    /// @since Bot API 6.3
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub added_to_attachment_menu: Option<bool>,

    /// `true` if this user has allowed the bot to message them. Omitted
    /// entirely when `false`.
    ///
    /// @since Bot API 6.3
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub allows_write_to_pm: Option<bool>,

    /// URL of the user's profile photo. Available to all Mini Apps when the
    /// user's privacy settings permit it.
    ///
    /// @since Bot API 7.5
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub photo_url: Option<String>,
}

/// Represents a chat in which the Mini App was launched via the attachment menu.
///
/// @see <https://core.telegram.org/bots/webapps#webappchat>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct WebAppChat {
    /// Unique identifier for the chat. Has at most 52 significant bits, so a
    /// 64-bit integer type is safe for storing this value.
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub id: i64,

    /// Type of the chat.
    // Serialised as "type" to match the Telegram wire format.
    #[serde(rename = "type")]
    pub kind: WebAppChatType,

    /// Title of the chat.
    pub title: String,

    /// Username of the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub username: Option<String>,

    /// URL of the chat's photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub photo_url: Option<String>,
}

/// Contains data transferred to the Mini App when it is opened. Empty if
/// launched from a keyboard button or from inline mode.
///
/// **Warning:** Data from this object must not be trusted on the client.
/// Only use `initData` on the bot's server after it has been validated
/// against the bot token (HMAC-SHA256) or via Ed25519 for third-party use.
///
/// This is the output of a successful [`crate::validate_hmac`] or
/// [`crate::validate_ed25519`] call. Never construct it directly — the
/// validation functions are the only trusted source.
///
/// # Security
///
/// `hash` and `signature` are retained for completeness only. After
/// validation they carry no further security value and should not be
/// forwarded to untrusted parties.
///
/// @see <https://core.telegram.org/bots/webapps#webappinitdata>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct WebAppInitData {
    /// A unique identifier for the Mini App session, required for sending
    /// messages via the `answerWebAppQuery` Bot API method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub query_id: Option<String>,

    /// An object containing data about the current user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub user: Option<WebAppUser>,

    /// An object containing data about the chat partner of the current user
    /// in the chat where the bot was launched via the attachment menu.
    /// Returned only for private chats and only for Mini Apps launched via
    /// the attachment menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub receiver: Option<WebAppUser>,

    /// An object containing data about the chat where the bot was launched
    /// via the attachment menu. Returned for supergroups, channels, and group
    /// chats — only for Mini Apps launched via the attachment menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub chat: Option<WebAppChat>,

    /// Type of the chat from which the Mini App was opened. Returned only for
    /// direct link Mini Apps and groups, supergroups, and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub chat_type: Option<InitDataChatType>,

    /// Global identifier indicating the chat from which the Mini App was
    /// opened. Returned only for direct link Mini Apps and groups,
    /// supergroups, and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub chat_instance: Option<String>,

    /// The value of the `startattach` or `startapp` parameter passed in the
    /// link used to launch the Mini App.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub start_param: Option<String>,

    /// Time in seconds, after which a message can be sent via the
    /// `answerWebAppQuery` method.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional, type = "number"))]
    pub can_send_after: Option<i64>,

    /// Unix timestamp of when the form was opened. Used to prevent replay
    /// attacks. Required field — always present in valid initData.
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub auth_date: i64,

    /// An HMAC-SHA-256 hash of the `data-check-string` used to validate the
    /// data against the bot token.
    pub hash: String,

    /// An Ed25519 signature of the `data-check-string` for third-party
    /// validation without requiring the bot token.
    ///
    /// @since Bot API 8.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub signature: Option<String>,
}
