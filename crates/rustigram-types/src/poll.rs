use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A native Telegram poll or quiz.
///
/// Polls are either `regular` (users vote freely) or `quiz` (one correct
/// answer). Returned inside [`Message`](crate::message::Message) when a
/// poll is sent or forwarded.
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,
    /// Poll question (1–300 characters).
    pub question: String,
    /// Special entities in the question.
    pub question_entities: Option<Vec<crate::message::MessageEntity>>,
    /// List of answer options.
    pub options: Vec<PollOption>,
    /// Total number of users who voted.
    pub total_voter_count: u32,
    /// `true` if the poll is closed.
    pub is_closed: bool,
    /// `true` if the poll is anonymous.
    pub is_anonymous: bool,
    /// Poll type.
    #[serde(rename = "type")]
    pub kind: PollType,
    /// `true` if the poll allows multiple answers.
    pub allows_multiple_answers: bool,
    /// `true` if voters can change their vote.
    pub allows_revoting: Option<bool>,
    /// Zero-based index of the correct answer (quiz polls only).
    pub correct_option_id: Option<u8>,
    /// Explanation shown after the quiz is answered.
    pub explanation: Option<String>,
    /// Special entities in the explanation.
    pub explanation_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Duration in seconds for which the poll will be active after creation.
    pub open_period: Option<u32>,
    /// Unix timestamp when the poll closes automatically.
    pub close_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One option in a [`Poll`].
pub struct PollOption {
    /// Option text (1–100 characters).
    pub text: String,
    /// Special entities in the option text.
    pub text_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Number of users who voted for this option.
    pub voter_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An option to include when creating a poll with `sendPoll`.
pub struct InputPollOption {
    /// Option text (1–100 characters).
    pub text: String,
    /// Parse mode for the option text.
    pub text_parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the option text.
    pub text_entities: Option<Vec<crate::message::MessageEntity>>,
}

impl InputPollOption {
    /// Creates a new `InputPollOption` with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            text_parse_mode: None,
            text_entities: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The type of a poll.
pub enum PollType {
    /// A regular poll where users can vote freely.
    Regular,
    /// A quiz with one correct answer.
    Quiz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A portion of a price — label and amount in the smallest currency unit.
///
/// For Telegram Stars (`XTR`) the amount is in whole Stars.
/// For fiat currencies (e.g. `USD`) the amount is in cents.
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,
    /// The chat that changed the answer (for anonymous polls in groups).
    pub voter_chat: Option<crate::chat::Chat>,
    /// The user who changed the answer (for non-anonymous polls).
    pub user: Option<User>,
    /// Indices of the chosen options (empty if the vote was retracted).
    pub option_ids: Vec<u8>,
}
