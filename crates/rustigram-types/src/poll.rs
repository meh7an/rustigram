use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::message::MessageEntity;
use crate::user::User;

/// A native Telegram poll or quiz.
///
/// Polls are either `regular` (users vote freely) or `quiz` (one or more correct
/// answers). Returned inside [`Message`](crate::message::Message) when a poll is
/// sent or forwarded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,
    /// Poll question (1–300 characters).
    pub question: String,
    /// Special entities in the question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
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
    /// `true` if voters can change their chosen answer options.
    pub allows_revoting: bool,
    /// Zero-based indices of the correct answers (quiz polls only).
    ///
    /// Bot API 9.6 changed this from a single `u8` to an array to support
    /// quizzes with multiple correct answers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_ids: Option<Vec<u8>>,
    /// Explanation shown after a quiz is answered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Special entities in the explanation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Duration in seconds the poll stays active after creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<u32>,
    /// Unix timestamp when the poll closes automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// Description of the poll; present only inside `Message` objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Special entities in the description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_entities: Option<Vec<MessageEntity>>,
    /// `true` if voting is limited to users who have been members of the chat for more than 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_only: Option<bool>,
    /// Two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which users can vote.
    /// If absent, users from any country can participate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
}

/// One option in a [`Poll`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    /// Persistent identifier of this option, stable across poll edits.
    pub persistent_id: String,
    /// Option text (1–100 characters).
    pub text: String,
    /// Special entities in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Number of users who voted for this option.
    pub voter_count: u32,
    /// The user who added this option, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by_user: Option<User>,
    /// The chat that added this option, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by_chat: Option<Chat>,
    /// Unix timestamp when this option was added; absent if it existed at poll creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addition_date: Option<i64>,
}

/// An option to include when creating a poll with `sendPoll`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPollOption {
    /// Option text (1–100 characters).
    pub text: String,
    /// Parse mode for the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the option text; alternative to `text_parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
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

/// The type of a poll.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PollType {
    /// A regular poll where users can vote freely.
    Regular,
    /// A quiz with one or more correct answers.
    Quiz,
}

/// An answer submitted by a user in a non-anonymous poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,
    /// The chat that changed the answer (for anonymous polls in groups).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    /// The user who changed the answer (for non-anonymous polls).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Zero-based indices of the chosen options; empty if the vote was retracted.
    pub option_ids: Vec<u8>,
    /// Persistent identifiers of the chosen options; empty if the vote was retracted.
    pub option_persistent_ids: Vec<String>,
}

/// Service message: a new option was added to a poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOptionAdded {
    /// Message containing the poll to which the option was added.
    ///
    /// Uses `serde_json::Value` because the API returns a `MaybeInaccessibleMessage`
    /// union type here. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_message: Option<serde_json::Value>,
    /// Persistent identifier of the added option.
    pub option_persistent_id: String,
    /// Text of the added option.
    pub option_text: String,
    /// Special entities in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_text_entities: Option<Vec<MessageEntity>>,
}

/// Service message: an option was deleted from a poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOptionDeleted {
    /// Message containing the poll from which the option was deleted.
    ///
    /// Uses `serde_json::Value` because the API returns a `MaybeInaccessibleMessage`
    /// union type here. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_message: Option<serde_json::Value>,
    /// Persistent identifier of the deleted option.
    pub option_persistent_id: String,
    /// Text of the deleted option.
    pub option_text: String,
    /// Special entities in the option text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_text_entities: Option<Vec<MessageEntity>>,
}
