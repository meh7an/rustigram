use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::games::GameHighScore;
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

// ─── setGameScore ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetGameScoreParams {
    user_id: i64,
    score: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}

/// Builder for the [`setGameScore`](https://core.telegram.org/bots/api#setgamescore) method.
///
/// Sets the score for a user in a game. Returns the edited `Message` on success
/// for chat messages, or `true` for inline messages.
///
/// Returns an error if the new score is not greater than the user's current score
/// and `force` is not set.
pub struct SetGameScore {
    client: BotClient,
    params: SetGameScoreParams,
}

impl SetGameScore {
    pub(crate) fn new(client: BotClient, user_id: i64, score: u32) -> Self {
        Self {
            client,
            params: SetGameScoreParams {
                user_id,
                score,
                force: None,
                disable_edit_message: None,
                chat_id: None,
                message_id: None,
                inline_message_id: None,
            },
        }
    }
    /// Pass `true` to allow the score to decrease — useful for fixing mistakes or banning cheaters.
    pub fn force(mut self, v: bool) -> Self {
        self.params.force = Some(v);
        self
    }
    /// Pass `true` to prevent the game message from being edited with the new scoreboard.
    pub fn disable_edit_message(mut self, v: bool) -> Self {
        self.params.disable_edit_message = Some(v);
        self
    }
    /// Sets the target chat message. Required if `inline_message_id` is not set.
    pub fn chat_message(mut self, chat_id: i64, message_id: i64) -> Self {
        self.params.chat_id = Some(chat_id);
        self.params.message_id = Some(message_id);
        self
    }
    /// Sets the target inline message. Required if `chat_id` and `message_id` are not set.
    pub fn inline_message_id(mut self, id: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(id.into());
        self
    }
}

// Returns the edited `Message` as `serde_json::Value` to handle the union
// return type (`Message | true`) until a proper enum is defined.
impl_into_future!(SetGameScore, serde_json::Value, "setGameScore");

// ─── getGameHighScores ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GetGameHighScoresParams {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
}

/// Builder for the [`getGameHighScores`](https://core.telegram.org/bots/api#getgamehighscores) method.
///
/// Returns the score of the specified user and several of their neighbours
/// in the game's high score table. Also returns the top 3 users if they are
/// not among those neighbours.
pub struct GetGameHighScores {
    client: BotClient,
    params: GetGameHighScoresParams,
}

impl GetGameHighScores {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: GetGameHighScoresParams {
                user_id,
                chat_id: None,
                message_id: None,
                inline_message_id: None,
            },
        }
    }
    /// Sets the target chat message. Required if `inline_message_id` is not set.
    pub fn chat_message(mut self, chat_id: i64, message_id: i64) -> Self {
        self.params.chat_id = Some(chat_id);
        self.params.message_id = Some(message_id);
        self
    }
    /// Sets the target inline message. Required if `chat_id` and `message_id` are not set.
    pub fn inline_message_id(mut self, id: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(id.into());
        self
    }
}

impl_into_future!(GetGameHighScores, Vec<GameHighScore>, "getGameHighScores");
