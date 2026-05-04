use serde::{Deserialize, Serialize};

use crate::chat::{Chat, Location, Venue};
use crate::checklist::{Checklist, ChecklistTasksAdded, ChecklistTasksDone};
use crate::direct_messages::{
    DirectMessagePriceChanged, DirectMessagesTopic, PaidMessagePriceChanged,
};
use crate::file::{Animation, Audio, Document, PhotoSize, Video, VideoNote, Voice};
use crate::keyboard::InlineKeyboardMarkup;
use crate::managed_bot::ManagedBotCreated;
use crate::poll::{Poll, PollOptionAdded, PollOptionDeleted};
use crate::sticker::Sticker;
use crate::suggested_post::{
    SuggestedPostApprovalFailed, SuggestedPostApproved, SuggestedPostDeclined, SuggestedPostInfo,
    SuggestedPostPaid, SuggestedPostRefunded,
};
use crate::user::User;

/// A Telegram message.
///
/// Only fields that were actually sent will be `Some`. Consult the
/// official Bot API documentation for field availability rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub message_id: i64,

    /// Optional — unique identifier of a message thread to which the message belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Information about the direct messages chat topic that contains the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic: Option<DirectMessagesTopic>,

    /// Sender of the message; empty for messages sent to channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    /// Sender of the message; empty for messages sent to channels or on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,

    /// For supergroup messages — boost count of the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<u32>,

    /// The bot that actually sent the message on behalf of the business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,

    /// Tag or custom title of the sender; for supergroups only (Bot API 9.5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_tag: Option<String>,

    /// Date the message was sent, as a Unix timestamp.
    pub date: i64,

    /// Unique identifier of the business connection from which the message was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Conversation the message belongs to.
    pub chat: Chat,

    /// Information about the original message for forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,

    /// `true` if the message is sent to a forum topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,

    /// `true` if the message is a channel post automatically forwarded to the
    /// connected discussion group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,

    /// For replies in the same chat and message thread, the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    /// Information about the message that is being replied to from another chat or topic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,

    /// For replies that quote part of the original message, the quoted part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,

    /// For replies to a story, the original story.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<serde_json::Value>,

    /// Identifier of the checklist task being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_checklist_task_id: Option<i64>,

    /// Persistent identifier of the poll option being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_poll_option_id: Option<String>,

    /// Bot through which the message was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    /// Date the message was last edited, as a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,

    /// `true` if the message can't be forwarded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    /// `true` if the message was sent by an implicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,

    /// `true` if the message is a paid post.
    ///
    /// Paid posts must not be deleted for 24 hours after sending and cannot be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paid_post: Option<bool>,

    /// The unique identifier of a media message group this message belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    /// Signature of the post author for messages in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    /// Number of Telegram Stars paid by the sender to send this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<i64>,

    /// Actual UTF-8 text of the message (0–4096 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Options used for link preview generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// Information about a suggested post; present when the message is a suggested
    /// post in a channel direct messages chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_info: Option<SuggestedPostInfo>,

    /// Unique identifier of the message effect added to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,

    // Media fields
    /// Animation attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Audio file attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Document attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Paid media attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<serde_json::Value>,
    /// Photo attached to the message (array of sizes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Sticker attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Story attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<serde_json::Value>,
    /// Video attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Video note attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Voice note attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    /// Caption for the animation, audio, document, paid media, photo, video or voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// `true` if the caption must be shown above the message media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    /// `true` if the message media is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,

    /// Checklist attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Checklist>,

    // Service message types
    /// Contact shared in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Dice result in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Game in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<serde_json::Value>,
    /// Poll in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Venue in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    /// Location in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    // Group events
    /// New members that joined the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    /// A member that left the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    /// Service message: chat owner has left.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_owner_left: Option<ChatOwnerLeft>,
    /// Service message: chat owner has changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_owner_changed: Option<ChatOwnerChanged>,
    /// New chat title (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// New chat photo (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// `true` if the chat photo was deleted (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    /// `true` if the group was created (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    /// `true` if the supergroup was created (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    /// `true` if the channel was created (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    /// Auto-delete timer changed (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<serde_json::Value>,
    /// The group has been migrated to a supergroup with this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,
    /// The pinned message (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    // Payment fields
    /// Invoice for a payment (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    /// Successful payment information (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<serde_json::Value>,
    /// Refunded payment information (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<serde_json::Value>,

    // Web app
    /// Data from the Web App.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,

    // Forum topic events
    /// Forum topic created (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<serde_json::Value>,
    /// Forum topic edited (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<serde_json::Value>,
    /// Forum topic closed (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<serde_json::Value>,
    /// Forum topic reopened (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<serde_json::Value>,
    /// General forum topic hidden (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<serde_json::Value>,
    /// General forum topic unhidden (service message).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<serde_json::Value>,

    // Managed bot events
    /// Service message: a new managed bot was created (Bot API 9.6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_bot_created: Option<ManagedBotCreated>,

    // Poll events
    /// Service message: an option was added to a poll (Bot API 9.6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_added: Option<PollOptionAdded>,
    /// Service message: an option was deleted from a poll (Bot API 9.6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_deleted: Option<PollOptionDeleted>,

    // Checklist events
    /// Service message: tasks in a checklist were marked done or not done.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_tasks_done: Option<ChecklistTasksDone>,
    /// Service message: tasks were added to a checklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_tasks_added: Option<ChecklistTasksAdded>,

    // Direct messages events
    /// Service message: the price for paid messages in the direct messages chat changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_price_changed: Option<DirectMessagePriceChanged>,
    /// Service message: the price for paid messages in the chat changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_message_price_changed: Option<PaidMessagePriceChanged>,

    // Suggested post events
    /// Service message: a suggested post was approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_approved: Option<SuggestedPostApproved>,
    /// Service message: approval of a suggested post has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_approval_failed: Option<SuggestedPostApprovalFailed>,
    /// Service message: a suggested post was declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_declined: Option<SuggestedPostDeclined>,
    /// Service message: payment for a suggested post was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_paid: Option<SuggestedPostPaid>,
    /// Service message: payment for a suggested post was refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_refunded: Option<SuggestedPostRefunded>,

    // Guest mode fields
    /// For a message sent by a guest bot, the user whose original message triggered the bot's response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_bot_caller_user: Option<User>,

    /// For a message sent by a guest bot, the chat whose original message triggered the bot's response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_bot_caller_chat: Option<crate::chat::Chat>,

    /// The unique identifier for the guest query.
    ///
    /// Use with [`answerGuestQuery`](https://core.telegram.org/bots/api#answerguestquery) to send a
    /// response. If non-empty, the message belongs to a chat of the corresponding business account
    /// independent from any potential bot chat sharing the same identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_query_id: Option<String>,
}

impl Message {
    /// Returns the effective text of the message — `text` or `caption`.
    #[must_use]
    pub fn effective_text(&self) -> Option<&str> {
        self.text.as_deref().or(self.caption.as_deref())
    }

    /// Returns `true` if the message is a command (text starts with `/`).
    #[must_use]
    pub fn is_command(&self) -> bool {
        self.entities
            .as_deref()
            .unwrap_or_default()
            .iter()
            .any(|e| e.kind == MessageEntityKind::BotCommand && e.offset == 0)
    }

    /// Extracts the command string (e.g. `"start"` from `/start@bot`), if present.
    #[must_use]
    pub fn command(&self) -> Option<&str> {
        let text = self.text.as_deref()?;
        let entity = self
            .entities
            .as_deref()?
            .iter()
            .find(|e| e.kind == MessageEntityKind::BotCommand && e.offset == 0)?;
        let raw = &text[..entity.length as usize];
        // Strip the `@BotUsername` suffix if present.
        Some(
            raw.find('@')
                .map_or(raw, |i| &raw[..i])
                .trim_start_matches('/'),
        )
    }
}

/// Type of a message entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityKind {
    /// @username mention.
    Mention,
    /// #hashtag.
    Hashtag,
    /// $cashtag.
    Cashtag,
    /// /bot_command.
    BotCommand,
    /// Plain URL.
    Url,
    /// Email address.
    Email,
    /// Phone number.
    PhoneNumber,
    /// **Bold** text.
    Bold,
    /// _Italic_ text.
    Italic,
    /// Underlined text.
    Underline,
    /// ~~Strikethrough~~ text.
    Strikethrough,
    /// ||Spoiler|| text.
    Spoiler,
    /// Block quotation.
    Blockquote,
    /// An "expandable" block quotation that can be expanded to show the full text.
    ExpandableBlockquote,
    /// Monospaced text.
    Code,
    /// Monospaced block.
    Pre,
    /// A text link. The `url` field will contain the destination URL.
    TextLink,
    /// A text mention of a user. The `user` field will contain the mentioned user.
    TextMention,
    /// Custom emoji. The `custom_emoji_id` field will contain the identifier of the custom emoji.
    CustomEmoji,
    /// Date/time entity.
    DateTime,
}

/// One special entity in a message text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Type of the entity.
    #[serde(rename = "type")]
    pub kind: MessageEntityKind,
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: u32,
    /// Length of the entity in UTF-16 code units.
    pub length: u32,
    /// For `TextLink` — URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// For `TextMention` — the mentioned user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// For `Pre` — the programming language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// For `CustomEmoji` — identifier of the custom emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// For `DateTime` — Unix timestamp associated with the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_time: Option<i64>,
    /// For `DateTime` — format string (`r|w?[dD]?[tT]?`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,
}

/// Origin of a forwarded message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    /// Forwarded from a user with a visible profile.
    User {
        /// Date the original message was sent, as a Unix timestamp.
        date: i64,
        /// The user who sent the original message.
        sender_user: User,
    },
    /// Forwarded from a user with a hidden profile.
    HiddenUser {
        /// Date the original message was sent, as a Unix timestamp.
        date: i64,
        /// Name of the user who sent the original message.
        sender_user_name: String,
    },
    /// Forwarded from a chat on behalf of the chat itself.
    Chat {
        /// Date the original message was sent, as a Unix timestamp.
        date: i64,
        /// The chat that sent the original message.
        sender_chat: Chat,
        /// Signature of the original post author.
        #[serde(skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
    /// Forwarded from a channel.
    Channel {
        /// Date the original message was sent, as a Unix timestamp.
        date: i64,
        /// The channel that sent the original message.
        chat: Chat,
        /// Identifier of the original message in the channel.
        message_id: i64,
        /// Signature of the original post author.
        #[serde(skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
}

/// Parameters for replying to a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to.
    pub message_id: i64,
    /// Chat containing the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<crate::user::ChatId>,
    /// `true` if the message should be sent even if the specified message is not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    /// Parse mode for the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<ParseMode>,
    /// Special entities in the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Position of the quote in the original message (UTF-16 offset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<u32>,
    /// Identifier of the poll option being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_id: Option<String>,
    /// Identifier of the checklist task being replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_task_id: Option<i64>,
}

/// Text parse mode for message formatting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParseMode {
    /// Markdown v2 — recommended.
    MarkdownV2,
    /// HTML formatting.
    HTML,
    /// Legacy Markdown — limited.
    Markdown,
}

/// Reaction type on a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    /// A standard emoji reaction.
    Emoji {
        /// The emoji character.
        emoji: String,
    },
    /// A custom emoji reaction.
    CustomEmoji {
        /// Identifier of the custom emoji.
        custom_emoji_id: String,
    },
    /// A paid star reaction.
    Paid,
}

/// Options for controlling how links are previewed in messages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    /// `true` if link preview is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    /// URL to use for the link preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// `true` if the media in the link preview should be shrunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    /// `true` if the media in the link preview should be enlarged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    /// `true` if the link preview should be shown above the message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

/// Information about the quoted part of a message that is replied to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of the message.
    pub text: String,
    /// Special entities in the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Approximate quote position in the original message (UTF-16 offset).
    pub position: u32,
    /// `true` if the quote was chosen manually by the message sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

/// Information about a message that is being replied to from outside the thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    /// Origin of the message being replied to.
    pub origin: MessageOrigin,
    /// Chat the original message belongs to (if not the current chat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    /// Identifier of the original message in the original chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Options for link preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Animation in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Audio in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Document in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Photo in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Sticker in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Video in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Video note in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Voice note in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// `true` if the media is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Checklist in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist: Option<Checklist>,
    /// Contact in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Dice in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Location in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Venue in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    /// Poll in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
}

/// Phone contact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// Contact's phone number.
    pub phone_number: String,
    /// Contact's first name.
    pub first_name: String,
    /// Contact's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Contact's Telegram user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Contact's vCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// Animated dice with a random value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based.
    pub emoji: String,
    /// Value of the dice (1–6 for 🎲🎯🎳; 1–5 for 🏀⚽; 1–64 for 🎰).
    pub value: u8,
}

/// Data sent from a Web App.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppData {
    /// Data sent by the Web App.
    pub data: String,
    /// Text of the button that opened the Web App.
    pub button_text: String,
}

/// Information about a Web App.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// HTTPS URL of the Web App.
    pub url: String,
}

/// Lightweight message identifier returned by `copyMessage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId {
    /// Identifier of the message.
    pub message_id: i64,
}

/// Service message: the chat owner left the chat (Bot API 9.4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOwnerLeft {
    /// The user who will become the new owner if the previous owner does not return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<User>,
}

/// Service message: ownership of the chat changed (Bot API 9.4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatOwnerChanged {
    /// The new owner of the chat.
    pub new_owner: User,
}
