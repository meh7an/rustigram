use crate::user::User;
use serde::{Deserialize, Serialize};

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
