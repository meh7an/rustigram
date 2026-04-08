use std::future::{Future, IntoFuture};
use std::pin::Pin;

use serde::Serialize;

use rustigram_types::checklist::InputChecklist;
use rustigram_types::keyboard::InlineKeyboardMarkup;
use rustigram_types::message::{LinkPreviewOptions, Message, MessageEntity, ParseMode};
use rustigram_types::user::ChatId;

use crate::client::BotClient;
use crate::error::Result;

// ─── Helper macro ─────────────────────────────────────────────────────────────

/// Generates an `IntoFuture` impl that calls `BotClient::post_json`.
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

/// Target identifier for inline message edits.
#[derive(Serialize)]
#[serde(untagged)]
/// Identifies the target message — either a chat message or an inline message.
pub enum EditTarget {
    /// Targets a regular chat message.
    Chat {
        /// The chat containing the message.
        chat_id: ChatId,
        /// Identifier of the message to edit.
        message_id: i64,
    },
    /// Targets an inline message sent via inline mode.
    Inline {
        /// Identifier of the inline message.
        inline_message_id: String,
    },
}

// ─── editMessageText ──────────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditMessageTextParams {
    #[serde(flatten)]
    target: EditTarget,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`editMessageText`](https://core.telegram.org/bots/api#editmessagetext) method.
pub struct EditMessageText {
    client: BotClient,
    params: EditMessageTextParams,
}

impl EditMessageText {
    pub(crate) fn in_chat(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        text: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: EditMessageTextParams {
                target: EditTarget::Chat {
                    chat_id: chat_id.into(),
                    message_id,
                },
                text: text.into(),
                business_connection_id: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                reply_markup: None,
            },
        }
    }
    pub(crate) fn inline(
        client: BotClient,
        inline_message_id: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: EditMessageTextParams {
                target: EditTarget::Inline {
                    inline_message_id: inline_message_id.into(),
                },
                text: text.into(),
                business_connection_id: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                reply_markup: None,
            },
        }
    }
    /// Sets the text parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.params.parse_mode = Some(m);
        self
    }
    /// Sets custom entities instead of using a parse mode.
    pub fn entities(mut self, e: Vec<MessageEntity>) -> Self {
        self.params.entities = Some(e);
        self
    }
    /// Configures link preview options for the edited message.
    pub fn link_preview_options(mut self, o: LinkPreviewOptions) -> Self {
        self.params.link_preview_options = Some(o);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(EditMessageText, Message, "editMessageText");

// ─── editMessageCaption ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditMessageCaptionParams {
    #[serde(flatten)]
    target: EditTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`editMessageCaption`](https://core.telegram.org/bots/api#editmessagecaption) method.
pub struct EditMessageCaption {
    client: BotClient,
    params: EditMessageCaptionParams,
}

impl EditMessageCaption {
    pub(crate) fn in_chat(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: EditMessageCaptionParams {
                target: EditTarget::Chat {
                    chat_id: chat_id.into(),
                    message_id,
                },
                business_connection_id: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                reply_markup: None,
            },
        }
    }
    /// Sets the new caption text (0–1024 characters).
    pub fn caption(mut self, c: impl Into<String>) -> Self {
        self.params.caption = Some(c.into());
        self
    }
    /// Sets the caption parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.params.parse_mode = Some(m);
        self
    }
    /// Shows the caption above the media instead of below it.
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.params.show_caption_above_media = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(EditMessageCaption, Message, "editMessageCaption");

// ─── editMessageMedia ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditMessageMediaParams {
    #[serde(flatten)]
    target: EditTarget,
    /// The new media content.
    ///
    /// Uses `serde_json::Value` until the `InputMedia` enum is defined in
    /// Priority 4. Pass the result of `serde_json::to_value(&your_input_media)`.
    media: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`editMessageMedia`](https://core.telegram.org/bots/api#editmessagemedia) method.
///
/// Edits the media content of a message (animation, audio, document, photo, or video).
///
/// The `media` parameter accepts `serde_json::Value` until the `InputMedia` enum is
/// defined in Priority 4. Construct it with `serde_json::json!({...})` or
/// `serde_json::to_value(&input_media)`.
pub struct EditMessageMedia {
    client: BotClient,
    params: EditMessageMediaParams,
}

impl EditMessageMedia {
    pub(crate) fn in_chat(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        media: serde_json::Value,
    ) -> Self {
        Self {
            client,
            params: EditMessageMediaParams {
                target: EditTarget::Chat {
                    chat_id: chat_id.into(),
                    message_id,
                },
                media,
                business_connection_id: None,
                reply_markup: None,
            },
        }
    }
    pub(crate) fn inline(
        client: BotClient,
        inline_message_id: impl Into<String>,
        media: serde_json::Value,
    ) -> Self {
        Self {
            client,
            params: EditMessageMediaParams {
                target: EditTarget::Inline {
                    inline_message_id: inline_message_id.into(),
                },
                media,
                business_connection_id: None,
                reply_markup: None,
            },
        }
    }
    /// Business connection ID for editing a message sent on behalf of a business account.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Attaches a new inline keyboard to the message.
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(EditMessageMedia, Message, "editMessageMedia");

// ─── editMessageReplyMarkup ───────────────────────────────────────────────────

#[derive(Serialize)]
struct EditMessageReplyMarkupParams {
    #[serde(flatten)]
    target: EditTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`editMessageReplyMarkup`](https://core.telegram.org/bots/api#editmessagereplymarkup) method.
pub struct EditMessageReplyMarkup {
    client: BotClient,
    params: EditMessageReplyMarkupParams,
}

impl EditMessageReplyMarkup {
    pub(crate) fn in_chat(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: EditMessageReplyMarkupParams {
                target: EditTarget::Chat {
                    chat_id: chat_id.into(),
                    message_id,
                },
                business_connection_id: None,
                reply_markup: None,
            },
        }
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
    /// Removes the inline keyboard from the message.
    pub fn remove_markup(mut self) -> Self {
        self.params.reply_markup = None;
        self
    }
}

impl_into_future!(EditMessageReplyMarkup, Message, "editMessageReplyMarkup");

// ─── editMessageChecklist ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditMessageChecklistParams {
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
    checklist: InputChecklist,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`editMessageChecklist`](https://core.telegram.org/bots/api#editmessagechecklist) method.
///
/// Business bots only — edits a checklist message sent on behalf of a connected
/// business account. Requires the `can_reply` business bot right.
pub struct EditMessageChecklist {
    client: BotClient,
    params: EditMessageChecklistParams,
}

impl EditMessageChecklist {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: InputChecklist,
    ) -> Self {
        Self {
            client,
            params: EditMessageChecklistParams {
                business_connection_id: business_connection_id.into(),
                chat_id,
                message_id,
                checklist,
                reply_markup: None,
            },
        }
    }
    /// Attaches a new inline keyboard to the message.
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(EditMessageChecklist, Message, "editMessageChecklist");

// ─── approveSuggestedPost ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct ApproveSuggestedPostParams {
    chat_id: i64,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_date: Option<i64>,
}

/// Builder for the [`approveSuggestedPost`](https://core.telegram.org/bots/api#approvesuggestedpost) method.
///
/// Approves a suggested post in a direct messages chat.
/// Requires the `can_post_messages` administrator right in the corresponding channel.
pub struct ApproveSuggestedPost {
    client: BotClient,
    params: ApproveSuggestedPostParams,
}

impl ApproveSuggestedPost {
    pub(crate) fn new(client: BotClient, chat_id: i64, message_id: i64) -> Self {
        Self {
            client,
            params: ApproveSuggestedPostParams {
                chat_id,
                message_id,
                send_date: None,
            },
        }
    }
    /// Unix timestamp when the post will be published (not more than 30 days in the future).
    ///
    /// Omit if the send date was already specified when the post was suggested.
    pub fn send_date(mut self, ts: i64) -> Self {
        self.params.send_date = Some(ts);
        self
    }
}

impl_into_future!(ApproveSuggestedPost, bool, "approveSuggestedPost");

// ─── declineSuggestedPost ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct DeclineSuggestedPostParams {
    chat_id: i64,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
}

/// Builder for the [`declineSuggestedPost`](https://core.telegram.org/bots/api#declinesuggestedpost) method.
///
/// Declines a suggested post in a direct messages chat.
/// Requires the `can_manage_direct_messages` administrator right in the corresponding channel.
pub struct DeclineSuggestedPost {
    client: BotClient,
    params: DeclineSuggestedPostParams,
}

impl DeclineSuggestedPost {
    pub(crate) fn new(client: BotClient, chat_id: i64, message_id: i64) -> Self {
        Self {
            client,
            params: DeclineSuggestedPostParams {
                chat_id,
                message_id,
                comment: None,
            },
        }
    }
    /// Optional comment for the creator of the suggested post (0–128 characters).
    pub fn comment(mut self, c: impl Into<String>) -> Self {
        self.params.comment = Some(c.into());
        self
    }
}

impl_into_future!(DeclineSuggestedPost, bool, "declineSuggestedPost");

// ─── editMessageLiveLocation ──────────────────────────────────────────────────

#[derive(Serialize)]
struct EditMessageLiveLocationParams {
    #[serde(flatten)]
    target: EditTarget,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`editMessageLiveLocation`](https://core.telegram.org/bots/api#editmessagelivelocation) method.
pub struct EditMessageLiveLocation {
    client: BotClient,
    params: EditMessageLiveLocationParams,
}

impl EditMessageLiveLocation {
    pub(crate) fn in_chat(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        message_id: i64,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            client,
            params: EditMessageLiveLocationParams {
                target: EditTarget::Chat {
                    chat_id: chat_id.into(),
                    message_id,
                },
                latitude,
                longitude,
                live_period: None,
                horizontal_accuracy: None,
                heading: None,
                proximity_alert_radius: None,
                reply_markup: None,
            },
        }
    }
    /// Sets how long the location stays live, in seconds (60–86400).
    pub fn live_period(mut self, v: u32) -> Self {
        self.params.live_period = Some(v);
        self
    }
    /// Sets the direction of movement in degrees (1–360).
    pub fn heading(mut self, v: u16) -> Self {
        self.params.heading = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(EditMessageLiveLocation, Message, "editMessageLiveLocation");

// ─── stopMessageLiveLocation ──────────────────────────────────────────────────

#[derive(Serialize)]
struct StopMessageLiveLocationParams {
    #[serde(flatten)]
    target: EditTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Builder for the [`stopMessageLiveLocation`](https://core.telegram.org/bots/api#stopmessagelivelocation) method.
pub struct StopMessageLiveLocation {
    client: BotClient,
    params: StopMessageLiveLocationParams,
}

impl StopMessageLiveLocation {
    pub(crate) fn in_chat(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: StopMessageLiveLocationParams {
                target: EditTarget::Chat {
                    chat_id: chat_id.into(),
                    message_id,
                },
                reply_markup: None,
            },
        }
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(StopMessageLiveLocation, Message, "stopMessageLiveLocation");
