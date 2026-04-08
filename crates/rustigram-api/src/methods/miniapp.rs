use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::keyboard::KeyboardButton;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── Helper macro ─────────────────────────────────────────────────────────────

macro_rules! impl_into_future {
    ($builder:ident, $return_ty:ty, $method:literal) => {
        impl IntoFuture for $builder {
            type Output = Result<$return_ty>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.client.post_json($method, &self.params).await })
            }
        }
    };
}

// ─── savePreparedKeyboardButton ───────────────────────────────────────────────

#[derive(Serialize)]
struct SavePreparedKeyboardButtonParams {
    user_id: i64,
    button: KeyboardButton,
}

/// Builder for the [`savePreparedKeyboardButton`](https://core.telegram.org/bots/api#savepreparedkeyboardbutton) method (Bot API 9.6).
///
/// Stores a keyboard button that can be used by a user within a Mini App.
/// The button must be of type `request_users`, `request_chat`, or `request_managed_bot`.
///
/// Returns a `PreparedKeyboardButton` object as `serde_json::Value` until the
/// type is formally defined in Priority 4.
pub struct SavePreparedKeyboardButton {
    client: BotClient,
    params: SavePreparedKeyboardButtonParams,
}

impl SavePreparedKeyboardButton {
    pub(crate) fn new(client: BotClient, user_id: i64, button: KeyboardButton) -> Self {
        Self {
            client,
            params: SavePreparedKeyboardButtonParams { user_id, button },
        }
    }
}

// Returns `PreparedKeyboardButton` as `serde_json::Value` until the type is defined in Priority 4.
impl_into_future!(
    SavePreparedKeyboardButton,
    serde_json::Value,
    "savePreparedKeyboardButton"
);

// ─── setUserEmojiStatus ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetUserEmojiStatusParams {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_status_expiration_date: Option<i64>,
}

/// Builder for the [`setUserEmojiStatus`](https://core.telegram.org/bots/api#setuseremojistatus) method.
///
/// Changes the emoji status for a user who previously allowed the bot to manage
/// their emoji status via the Mini App method `requestEmojiStatusAccess`.
///
/// Omit `emoji_status_custom_emoji_id` or pass an empty string to remove the status.
pub struct SetUserEmojiStatus {
    client: BotClient,
    params: SetUserEmojiStatusParams,
}

impl SetUserEmojiStatus {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: SetUserEmojiStatusParams {
                user_id,
                emoji_status_custom_emoji_id: None,
                emoji_status_expiration_date: None,
            },
        }
    }
    /// Sets the custom emoji identifier for the emoji status.
    /// Pass an empty string to remove the current status.
    pub fn emoji_status_custom_emoji_id(mut self, id: impl Into<String>) -> Self {
        self.params.emoji_status_custom_emoji_id = Some(id.into());
        self
    }
    /// Sets the expiration date of the emoji status as a Unix timestamp.
    pub fn emoji_status_expiration_date(mut self, ts: i64) -> Self {
        self.params.emoji_status_expiration_date = Some(ts);
        self
    }
}

impl_into_future!(SetUserEmojiStatus, bool, "setUserEmojiStatus");
