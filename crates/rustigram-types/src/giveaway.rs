use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::message::Message;
use crate::user::User;

/// Service message: a scheduled giveaway was created.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GiveawayCreated {
    /// Number of Telegram Stars to be split among winners; Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}

/// A scheduled giveaway.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Giveaway {
    /// The chats the user must join to take part.
    pub chats: Vec<Chat>,
    /// Point in time when winners will be selected, as a Unix timestamp.
    pub winners_selection_date: i64,
    /// Number of users who will be selected as winners.
    pub winner_count: u32,
    /// `true` if only users who join the chats after the giveaway started
    /// should be eligible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// `true` if the list of winners will be visible to everyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    /// Description of additional giveaway prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    /// Two-letter ISO 3166-1 alpha-2 country codes of eligible users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// Number of Telegram Stars to be split among winners; Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// Number of months the Telegram Premium subscription won will be active
    /// for; Premium giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<u32>,
}

/// Service message: giveaway winners were selected.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GiveawayWinners {
    /// The chat that created the giveaway.
    pub chat: Chat,
    /// Identifier of the message with the giveaway in the chat.
    pub giveaway_message_id: i64,
    /// Point in time when winners were selected, as a Unix timestamp.
    pub winners_selection_date: i64,
    /// Total number of winners.
    pub winner_count: u32,
    /// The users who won the giveaway, up to 100.
    pub winners: Vec<User>,
    /// Number of other chats the user had to join to take part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<u32>,
    /// Number of Telegram Stars split among winners; Star giveaways only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// Number of months the won Premium subscription is active for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<u32>,
    /// Number of undistributed prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<u32>,
    /// `true` if only users who joined after the giveaway started were eligible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    /// `true` if the giveaway was cancelled because the payment was refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Description of additional giveaway prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

/// Service message: a giveaway without public winners has completed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway.
    pub winner_count: u32,
    /// Number of undistributed prizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<u32>,
    /// The message with the giveaway that was completed.
    ///
    /// Boxed because [`Message`] transitively contains this type, and an
    /// unboxed cycle would make the struct infinitely sized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
    /// `true` if the giveaway is a Telegram Star giveaway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
}
