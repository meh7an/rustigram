use serde::{Deserialize, Serialize};

use crate::chat::ChatJoinRequest;
use crate::inline::{ChosenInlineResult, InlineQuery};
use crate::message::Message;
use crate::payments::{PreCheckoutQuery, ShippingQuery};
use crate::poll::{Poll, PollAnswer};
use crate::user::User;

/// An incoming update from Telegram.
///
/// Only one of the optional fields will be `Some` per update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    /// Unique sequential identifier.
    pub update_id: i64,
    /// The kind of update and its payload.
    #[serde(flatten)]
    pub kind: UpdateKind,
}

/// The specific kind of an incoming update.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind {
    /// New incoming message.
    Message(Message),
    /// New version of a message.
    EditedMessage(Message),
    /// New incoming channel post.
    ChannelPost(Message),
    /// New version of a channel post.
    EditedChannelPost(Message),
    /// A message from a business account.
    BusinessMessage(Message),
    /// Edited message from a business account.
    EditedBusinessMessage(Message),
    /// New incoming inline query.
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen.
    ChosenInlineResult(ChosenInlineResult),
    /// New incoming callback query.
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query (only for invoices with flexible price).
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query.
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state.
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll.
    PollAnswer(PollAnswer),
    /// Bot's chat member status was updated in a chat.
    MyChatMember(ChatMemberUpdated),
    /// A chat member's status was updated in a chat.
    ChatMember(ChatMemberUpdated),
    /// A request to join the chat has been sent.
    ChatJoinRequest(ChatJoinRequest),
    /// A reaction to a message was changed by a user.
    MessageReaction(MessageReactionUpdated),
    /// Reactions to a message with anonymous reactions were changed.
    MessageReactionCount(MessageReactionCountUpdated),
    /// A chat boost was added or changed.
    ChatBoost(ChatBoostUpdated),
    /// A boost was removed from a chat.
    RemovedChatBoost(ChatBoostRemoved),
    /// A managed bot was connected or disconnected.
    ManagedBot(ManagedBotUpdated),
    /// A business connection was established or removed.
    BusinessConnection(BusinessConnection),
    /// Messages were deleted from a connected business account.
    DeletedBusinessMessages(BusinessMessagesDeleted),
    /// Purchased paid media.
    PurchasedPaidMedia(PaidMediaPurchased),
}

impl Update {
    /// Returns the `chat_id` if the update contains a message or callback query.
    #[must_use]
    pub fn chat_id(&self) -> Option<i64> {
        match &self.kind {
            UpdateKind::Message(m)
            | UpdateKind::EditedMessage(m)
            | UpdateKind::ChannelPost(m)
            | UpdateKind::EditedChannelPost(m)
            | UpdateKind::BusinessMessage(m)
            | UpdateKind::EditedBusinessMessage(m) => Some(m.chat.id),
            UpdateKind::CallbackQuery(q) => q.message.as_ref().map(|m| m.chat.id),
            _ => None,
        }
    }

    /// Returns the `from` user if available.
    #[must_use]
    pub fn from(&self) -> Option<&User> {
        match &self.kind {
            UpdateKind::Message(m)
            | UpdateKind::EditedMessage(m)
            | UpdateKind::ChannelPost(m)
            | UpdateKind::EditedChannelPost(m)
            | UpdateKind::BusinessMessage(m)
            | UpdateKind::EditedBusinessMessage(m) => m.from.as_ref(),
            UpdateKind::CallbackQuery(q) => Some(&q.from),
            UpdateKind::InlineQuery(q) => Some(&q.from),
            UpdateKind::ShippingQuery(q) => Some(&q.from),
            UpdateKind::PreCheckoutQuery(q) => Some(&q.from),
            UpdateKind::PollAnswer(a) => a.user.as_ref(),
            _ => None,
        }
    }
}

/// Incoming callback query from a callback button in an inline keyboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// User who sent the query.
    pub from: User,
    /// Message sent by the bot with the button that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Identifier of the message sent via the bot in inline mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat where the query originated.
    pub chat_instance: String,
    /// Data associated with the callback button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Short name of a Game to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}

/// A chat member's status change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// The chat where the change occurred.
    pub chat: crate::chat::Chat,
    /// The user who triggered the change.
    pub from: User,
    /// Date of the change, as a Unix timestamp.
    pub date: i64,
    /// Previous chat member status.
    pub old_chat_member: crate::chat_member::ChatMember,
    /// New chat member status.
    pub new_chat_member: crate::chat_member::ChatMember,
    /// Invite link used to join the chat, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<crate::chat::ChatInviteLink>,
    /// `true` if the user joined via a join request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    /// `true` if the user joined via a folder invite link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

/// A reaction to a message changed by a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message.
    pub chat: crate::chat::Chat,
    /// Identifier of the message that was reacted to.
    pub message_id: i64,
    /// The user who changed the reaction (non-anonymous reactions only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// The chat that changed the reaction (anonymous reactions in groups).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<crate::chat::Chat>,
    /// Date of the change, as a Unix timestamp.
    pub date: i64,
    /// Previous list of reactions.
    pub old_reaction: Vec<crate::message::ReactionType>,
    /// New list of reactions.
    pub new_reaction: Vec<crate::message::ReactionType>,
}

/// Anonymous reactions to a message were changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message.
    pub chat: crate::chat::Chat,
    /// Identifier of the message.
    pub message_id: i64,
    /// Date of the change, as a Unix timestamp.
    pub date: i64,
    /// Updated list of reactions with counts.
    pub reactions: Vec<ReactionCount>,
}

/// Count of a specific reaction type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionCount {
    /// The reaction type.
    #[serde(rename = "type")]
    pub kind: crate::message::ReactionType,
    /// Total number of reactions of this type.
    pub total_count: u32,
}

/// A boost was added or changed in a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    /// The chat that was boosted.
    pub chat: crate::chat::Chat,
    /// Information about the boost.
    pub boost: ChatBoost,
}

/// Information about a chat boost.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Unix timestamp when the boost was added.
    pub add_date: i64,
    /// Unix timestamp when the boost expires.
    pub expiration_date: i64,
    /// Source of the boost.
    pub source: serde_json::Value,
}

/// A boost was removed from a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    /// The chat that lost the boost.
    pub chat: crate::chat::Chat,
    /// Unique identifier of the boost.
    pub boost_id: String,
    /// Unix timestamp when the boost was removed.
    pub remove_date: i64,
    /// Source of the removed boost.
    pub source: serde_json::Value,
}

/// A managed bot was connected or disconnected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedBotUpdated {
    /// The user who connected or disconnected the managed bot.
    pub user: User,
    /// The managed bot.
    pub bot: User,
}

/// A business connection was established or removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection.
    pub id: String,
    /// The business account user.
    pub user: User,
    /// Identifier of the private chat with the user.
    pub user_chat_id: i64,
    /// Date the connection was established as a Unix timestamp.
    pub date: i64,
    /// `true` if the bot can act on behalf of the business account.
    pub can_reply: bool,
    /// `true` if the connection is active.
    pub is_enabled: bool,
    /// Rights of the bot in the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<serde_json::Value>,
}

/// Business messages that were deleted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: String,
    /// The chat in which the messages were deleted.
    pub chat: crate::chat::Chat,
    /// Identifiers of the deleted messages.
    pub message_ids: Vec<i64>,
}

/// Purchased paid media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidMediaPurchased {
    /// The user who purchased the media.
    pub from: User,
    /// Bot-specified paid media payload.
    pub paid_media_payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A list of updates returned by `getUpdates`. Internal deserialization wrapper.
pub struct Updates {
    /// `true` if the request was successful.
    pub ok: bool,
    /// The list of updates.
    pub result: Vec<Update>,
}
