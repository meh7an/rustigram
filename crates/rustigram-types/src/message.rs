use serde::{Deserialize, Serialize};

use crate::chat::{Chat, Location, Venue};
use crate::file::{Animation, Audio, Document, PhotoSize, Video, VideoNote, Voice};
use crate::keyboard::InlineKeyboardMarkup;
use crate::poll::Poll;
use crate::sticker::Sticker;
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

    /// Sender of the message, empty for messages sent to channels.
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

    /// Date the message was sent , as a Unix timestamp.
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

    /// Information about the message that is being replied to by the current message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,

    /// For replies that quote part of the original message, the quoted part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,

    /// For replies to a story, the original story.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<serde_json::Value>,

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

    /// The unique identifier of a media message group this message belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    /// Signature of the post author for messages in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    /// Actual UTF-8 text of the message (0–4096 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Special entities like usernames, URLs, bot commands, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Options used for link preview generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

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

    // Direct messages
    /// Direct messages topic identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Suggested post information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post: Option<serde_json::Value>,
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
    /// Expandable block quotation.
    ExpandableBlockquote,
    /// Inline `code`.
    Code,
    /// Pre-formatted code block.
    Pre,
    /// Clickable text link.
    TextLink,
    /// Mention of a user without a username.
    TextMention,
    /// Custom emoji.
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
    /// For `DateTime` — Unix time.
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
