use serde::{Deserialize, Serialize};

use crate::file::PhotoSize;

/// A user shared with the bot via a
/// [`KeyboardButtonRequestUsers`](crate::keyboard::KeyboardButtonRequestUsers) button.
///
/// The optional name and photo fields are only present when the user granted
/// the bot access to them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SharedUser {
    /// Identifier of the shared user.
    ///
    /// May have more than 32 significant bits; 64-bit integers are safe for
    /// storing it. The bot may not have access to the user yet — it only gains
    /// access once the user sends a message.
    pub user_id: i64,
    /// First name of the user, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Username of the user, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Available sizes of the user's photo, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

/// Service message: users were shared with the bot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UsersShared {
    /// Identifier of the request that produced this share.
    pub request_id: i32,
    /// Information about the shared users.
    pub users: Vec<SharedUser>,
}

/// Service message: a chat was shared with the bot.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatShared {
    /// Identifier of the request that produced this share.
    pub request_id: i32,
    /// Identifier of the shared chat.
    ///
    /// The bot may not have access to the chat yet — it only gains access once
    /// the chat sends a message or the bot is added to it.
    pub chat_id: i64,
    /// Title of the chat, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Username of the chat, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Available sizes of the chat photo, if it was shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}
