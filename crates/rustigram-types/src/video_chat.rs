use serde::{Deserialize, Serialize};

use crate::user::User;

/// Service message: a video chat was scheduled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatScheduled {
    /// Point in time when the video chat is expected to start, as a Unix timestamp.
    pub start_date: i64,
}

/// Service message: a video chat started.
///
/// Carries no data — its presence is the entire signal. Declared with braces
/// rather than as a unit struct because Telegram sends `{}`, and a unit struct
/// would only deserialize from `null`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatStarted {}

/// Service message: a video chat ended.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatEnded {
    /// Video chat duration in seconds.
    pub duration: u32,
}

/// Service message: new participants were invited to a video chat.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VideoChatParticipantsInvited {
    /// The users invited to the video chat.
    pub users: Vec<User>,
}
