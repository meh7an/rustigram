use crate::file::{Animation, PhotoSize};
use crate::message::MessageEntity;
use crate::user::User;
use serde::{Deserialize, Serialize};

/// A game to be played in a Telegram chat.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Game {
    /// Title of the game.
    pub title: String,
    /// Description of the game.
    pub description: String,
    /// Photo displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game, or high scores included in the message.
    ///
    /// Set by calling [`setGameScore`](https://core.telegram.org/bots/api#setgamescore),
    /// or edited via
    /// [`editMessageText`](https://core.telegram.org/bots/api#editmessagetext).
    /// 0–4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Special entities that appear in `text`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Animation displayed in the game message in chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

/// Placeholder identifying a game callback button.
///
/// Telegram documents this as an object with no fields, so it is an empty
/// braced struct rather than a unit struct — the wire value is `{}`, and a unit
/// struct would only deserialize from `null`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackGame {}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One entry in a game high score table.
///
/// Returned as a list by [`getGameHighScores`](https://core.telegram.org/bots/api#getgamehighscores).
pub struct GameHighScore {
    /// Position in the high score table.
    pub position: u32,
    /// The user at this position.
    pub user: User,
    /// Score of the user.
    pub score: i64,
}
