use std::future::{Future, IntoFuture};
use std::pin::Pin;

use reqwest::multipart::{Form, Part};
use serde::Serialize;

use rustigram_types::file::InputFile;
use rustigram_types::keyboard::ReplyMarkup;
use rustigram_types::message::{LinkPreviewOptions, Message, ParseMode, ReplyParameters};
use rustigram_types::poll::InputPollOption;
use rustigram_types::user::ChatId;

use crate::client::BotClient;
use crate::error::Result;

// ─── Helper macro ────────────────────────────────────────────────────────────

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

// ─── sendMessage ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendMessageParams {
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<rustigram_types::message::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendMessage`](https://core.telegram.org/bots/api#sendmessage) method.
pub struct SendMessage {
    client: BotClient,
    params: SendMessageParams,
}

impl SendMessage {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SendMessageParams {
                chat_id: chat_id.into(),
                text: text.into(),
                business_connection_id: None,
                message_thread_id: None,
                parse_mode: None,
                entities: None,
                link_preview_options: None,
                disable_notification: None,
                protect_content: None,
                allow_paid_broadcast: None,
                message_effect_id: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Business connection ID for sending on behalf of a business account.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Sets the text parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, mode: ParseMode) -> Self {
        self.params.parse_mode = Some(mode);
        self
    }
    /// Sets custom message entities instead of using a parse mode.
    pub fn entities(mut self, entities: Vec<rustigram_types::message::MessageEntity>) -> Self {
        self.params.entities = Some(entities);
        self
    }
    /// Configures link preview generation options.
    pub fn link_preview_options(mut self, opts: LinkPreviewOptions) -> Self {
        self.params.link_preview_options = Some(opts);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Allows sending to large audiences at the cost of Telegram Stars.
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.params.allow_paid_broadcast = Some(v);
        self
    }
    /// Attaches a message effect (animated emoji reaction) to the message.
    pub fn message_effect_id(mut self, id: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(id.into());
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(rp);
        self
    }
    /// Convenience shortcut for `reply_parameters` — sets the reply-to message ID.
    pub fn reply_to(mut self, message_id: i64) -> Self {
        self.params.reply_parameters = Some(ReplyParameters {
            message_id,
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
            poll_option_id: None,
            checklist_task_id: None,
        });
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(markup.into());
        self
    }
}

impl_into_future!(SendMessage, Message, "sendMessage");

// ─── forwardMessage ───────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ForwardMessageParams {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_start_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
}

/// Builder for the [`forwardMessage`](https://core.telegram.org/bots/api#forwardmessage) method.
pub struct ForwardMessage {
    client: BotClient,
    params: ForwardMessageParams,
}

impl ForwardMessage {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> Self {
        Self {
            client,
            params: ForwardMessageParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_id,
                message_thread_id: None,
                video_start_timestamp: None,
                disable_notification: None,
                protect_content: None,
            },
        }
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl_into_future!(ForwardMessage, Message, "forwardMessage");

// ─── copyMessage ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CopyMessageParams {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_start_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<rustigram_types::message::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`copyMessage`](https://core.telegram.org/bots/api#copymessage) method.
pub struct CopyMessage {
    client: BotClient,
    params: CopyMessageParams,
}

impl CopyMessage {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> Self {
        Self {
            client,
            params: CopyMessageParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_id,
                message_thread_id: None,
                video_start_timestamp: None,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                show_caption_above_media: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Sets the caption (0–1024 characters) for media messages.
    pub fn caption(mut self, c: impl Into<String>) -> Self {
        self.params.caption = Some(c.into());
        self
    }
    /// Sets the text parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.params.parse_mode = Some(m);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(
    CopyMessage,
    rustigram_types::message::MessageId,
    "copyMessage"
);

// ─── sendChatAction ───────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendChatActionParams {
    chat_id: ChatId,
    action: ChatAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
}

#[derive(Serialize, Clone, Copy)]
/// The chat action to display while the bot is preparing a response.
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    /// Indicates the bot is composing a message.
    Typing,
    /// Indicates the bot is uploading a photo.
    UploadPhoto,
    /// Indicates the bot is recording a video.
    RecordVideo,
    /// Indicates the bot is uploading a video.
    UploadVideo,
    /// Indicates the bot is recording a voice note.
    RecordVoice,
    /// Indicates the bot is uploading a voice note.
    UploadVoice,
    /// Indicates the bot is uploading a document.
    UploadDocument,
    /// Indicates the bot is choosing a sticker.
    ChooseSticker,
    /// Indicates the bot is finding a location.
    FindLocation,
    /// Indicates the bot is recording a video note.
    RecordVideoNote,
    /// Indicates the bot is uploading a video note.
    UploadVideoNote,
}

/// Builder for the [`sendChatAction`](https://core.telegram.org/bots/api#sendchataction) method.
pub struct SendChatAction {
    client: BotClient,
    params: SendChatActionParams,
}

impl SendChatAction {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, action: ChatAction) -> Self {
        Self {
            client,
            params: SendChatActionParams {
                chat_id: chat_id.into(),
                action,
                business_connection_id: None,
                message_thread_id: None,
            },
        }
    }
    /// Business connection ID for sending on behalf of a business account.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
}

impl_into_future!(SendChatAction, bool, "sendChatAction");

// ─── sendDice ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendDiceParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendDice`](https://core.telegram.org/bots/api#senddice) method.
pub struct SendDice {
    client: BotClient,
    params: SendDiceParams,
}

impl SendDice {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: SendDiceParams {
                chat_id: chat_id.into(),
                emoji: None,
                message_thread_id: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// The dice/emoji to animate. One of 🎲 🎯 🏀 ⚽ 🎳 🎰.
    pub fn emoji(mut self, e: impl Into<String>) -> Self {
        self.params.emoji = Some(e.into());
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(SendDice, Message, "sendDice");

// ─── sendLocation ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendLocationParams {
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendLocation`](https://core.telegram.org/bots/api#sendlocation) method.
pub struct SendLocation {
    client: BotClient,
    params: SendLocationParams,
}

impl SendLocation {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        Self {
            client,
            params: SendLocationParams {
                chat_id: chat_id.into(),
                latitude,
                longitude,
                message_thread_id: None,
                horizontal_accuracy: None,
                live_period: None,
                heading: None,
                proximity_alert_radius: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Sets the radius of uncertainty for the location, in metres (0–1500).
    pub fn horizontal_accuracy(mut self, v: f64) -> Self {
        self.params.horizontal_accuracy = Some(v);
        self
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
    /// Sets the maximum distance in metres for proximity alerts.
    pub fn proximity_alert_radius(mut self, v: u32) -> Self {
        self.params.proximity_alert_radius = Some(v);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(SendLocation, Message, "sendLocation");

// ─── sendContact ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendContactParams {
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendContact`](https://core.telegram.org/bots/api#sendcontact) method.
pub struct SendContact {
    client: BotClient,
    params: SendContactParams,
}

impl SendContact {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SendContactParams {
                chat_id: chat_id.into(),
                phone_number: phone_number.into(),
                first_name: first_name.into(),
                last_name: None,
                vcard: None,
                message_thread_id: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Sets the last name of the contact.
    pub fn last_name(mut self, v: impl Into<String>) -> Self {
        self.params.last_name = Some(v.into());
        self
    }
    /// Sets the vCard data of the contact.
    pub fn vcard(mut self, v: impl Into<String>) -> Self {
        self.params.vcard = Some(v.into());
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(SendContact, Message, "sendContact");

// ─── sendPoll ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendPollParams {
    chat_id: ChatId,
    question: String,
    options: Vec<InputPollOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    question_entities: Option<Vec<rustigram_types::message::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    poll_type: Option<rustigram_types::poll::PollType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_revoting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correct_option_ids: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendPoll`](https://core.telegram.org/bots/api#sendpoll) method.
pub struct SendPoll {
    client: BotClient,
    params: SendPollParams,
}

impl SendPoll {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        question: impl Into<String>,
        options: Vec<InputPollOption>,
    ) -> Self {
        Self {
            client,
            params: SendPollParams {
                chat_id: chat_id.into(),
                question: question.into(),
                options,
                question_parse_mode: None,
                question_entities: None,
                message_thread_id: None,
                poll_type: None,
                is_anonymous: None,
                allows_multiple_answers: None,
                allows_revoting: None,
                correct_option_ids: None,
                explanation: None,
                explanation_parse_mode: None,
                open_period: None,
                close_date: None,
                is_closed: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Sets whether the poll is anonymous.
    pub fn is_anonymous(mut self, v: bool) -> Self {
        self.params.is_anonymous = Some(v);
        self
    }
    /// Allows voters to select multiple answers.
    pub fn allows_multiple_answers(mut self, v: bool) -> Self {
        self.params.allows_multiple_answers = Some(v);
        self
    }
    /// Allows voters to change their vote.
    pub fn allows_revoting(mut self, v: bool) -> Self {
        self.params.allows_revoting = Some(v);
        self
    }
    /// Converts the poll to a quiz with the given correct option index.
    pub fn quiz(mut self, correct_option_id: u8) -> Self {
        self.params.poll_type = Some(rustigram_types::poll::PollType::Quiz);
        self.params.correct_option_ids = Some(vec![correct_option_id]);
        self
    }
    /// Sets the explanation text shown after a quiz answer.
    pub fn explanation(mut self, text: impl Into<String>) -> Self {
        self.params.explanation = Some(text.into());
        self
    }
    /// Sets how long the poll stays open in seconds (5–2628000).
    pub fn open_period(mut self, secs: u32) -> Self {
        self.params.open_period = Some(secs);
        self
    }
    /// Sets the Unix timestamp when the poll closes automatically.
    pub fn close_date(mut self, ts: i64) -> Self {
        self.params.close_date = Some(ts);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(SendPoll, Message, "sendPoll");

// ─── sendMessageDraft ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendMessageDraftParams {
    chat_id: ChatId,
    draft_id: i64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<rustigram_types::message::MessageEntity>>,
}

/// Builder for the [`sendMessageDraft`](https://core.telegram.org/bots/api#sendmessagedraft) method.
/// Streams a partial message to the user while it is being generated (Bot API 9.5+).
pub struct SendMessageDraft {
    client: BotClient,
    params: SendMessageDraftParams,
}

impl SendMessageDraft {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        draft_id: i64,
        text: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SendMessageDraftParams {
                chat_id: chat_id.into(),
                draft_id,
                text: text.into(),
                message_thread_id: None,
                parse_mode: None,
                entities: None,
            },
        }
    }
    /// Sets the text parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.params.parse_mode = Some(m);
        self
    }
    /// Sets custom message entities instead of using a parse mode.
    pub fn entities(mut self, e: Vec<rustigram_types::message::MessageEntity>) -> Self {
        self.params.entities = Some(e);
        self
    }
}

impl_into_future!(SendMessageDraft, bool, "sendMessageDraft");

// ─── File-sending builders ────────────────────────────────────────────────────
//
// Photo, Audio, Document, Video, Animation, Voice, VideoNote each share a
// similar shape but differ in field names and constraints. We use a common
// pattern: store the InputFile and an optional Form for multipart, and build
// the form lazily in `IntoFuture`.

/// Common optional parameters shared by most media-send methods.
#[derive(Default)]
pub struct MediaSendOptions {
    /// Business connection ID for sending on behalf of a business account.
    pub business_connection_id: Option<String>,
    /// Forum topic thread ID.
    pub message_thread_id: Option<i64>,
    /// Sets the caption (0–1024 characters) for media messages.
    pub caption: Option<String>,
    /// Parse mode for the caption.
    pub parse_mode: Option<ParseMode>,
    /// Special entities in the caption.
    pub caption_entities: Option<Vec<rustigram_types::message::MessageEntity>>,
    /// Shows the caption above the media instead of below it.
    pub show_caption_above_media: Option<bool>,
    /// Marks the media as a spoiler — blurs it until the user taps to reveal.
    pub has_spoiler: Option<bool>,
    /// Sends the message silently — the recipient receives no notification sound.
    pub disable_notification: Option<bool>,
    /// Protects the message from being forwarded or saved.
    pub protect_content: Option<bool>,
    /// Allows sending to large audiences at the cost of Telegram Stars.
    pub allow_paid_broadcast: Option<bool>,
    /// Reply parameters for this message.
    pub reply_parameters: Option<ReplyParameters>,
    /// Reply markup attached to the message.
    pub reply_markup: Option<ReplyMarkup>,
}

/// Builds the JSON body for a simple (non-file-upload) part of a media send.
fn media_json_body(
    chat_id: &ChatId,
    media_field: &str,
    media_value: &str,
    opts: &MediaSendOptions,
    extra: serde_json::Value,
) -> serde_json::Value {
    let mut map = serde_json::json!({
        "chat_id": chat_id,
        media_field: media_value,
    });
    let obj = map.as_object_mut().unwrap();
    if let Some(v) = &opts.business_connection_id {
        obj.insert("business_connection_id".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = &opts.message_thread_id {
        obj.insert("message_thread_id".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = &opts.caption {
        obj.insert("caption".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = &opts.parse_mode {
        obj.insert("parse_mode".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = &opts.caption_entities {
        obj.insert("caption_entities".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = opts.show_caption_above_media {
        obj.insert("show_caption_above_media".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = opts.has_spoiler {
        obj.insert("has_spoiler".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = opts.disable_notification {
        obj.insert("disable_notification".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = opts.protect_content {
        obj.insert("protect_content".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = opts.allow_paid_broadcast {
        obj.insert("allow_paid_broadcast".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = &opts.reply_parameters {
        obj.insert("reply_parameters".to_owned(), serde_json::json!(v));
    }
    if let Some(v) = &opts.reply_markup {
        obj.insert("reply_markup".to_owned(), serde_json::json!(v));
    }
    if let serde_json::Value::Object(extra_obj) = extra {
        for (k, v) in extra_obj {
            obj.insert(k, v);
        }
    }
    map
}

// ─── sendPhoto ────────────────────────────────────────────────────────────────

/// Builder for the [`sendPhoto`](https://core.telegram.org/bots/api#sendphoto) method.
pub struct SendPhoto {
    client: BotClient,
    chat_id: ChatId,
    photo: InputFile,
    opts: MediaSendOptions,
}

impl SendPhoto {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, photo: InputFile) -> Self {
        Self {
            client,
            chat_id: chat_id.into(),
            photo,
            opts: MediaSendOptions::default(),
        }
    }
    /// Sets the caption (0–1024 characters) for media messages.
    pub fn caption(mut self, c: impl Into<String>) -> Self {
        self.opts.caption = Some(c.into());
        self
    }
    /// Sets the caption parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.opts.parse_mode = Some(m);
        self
    }
    /// Marks the media as a spoiler — blurs it until the user taps to reveal.
    pub fn has_spoiler(mut self, v: bool) -> Self {
        self.opts.has_spoiler = Some(v);
        self
    }
    /// Shows the caption above the media instead of below it.
    pub fn show_caption_above_media(mut self, v: bool) -> Self {
        self.opts.show_caption_above_media = Some(v);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.opts.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.opts.protect_content = Some(v);
        self
    }
    /// Allows sending to large audiences at the cost of Telegram Stars.
    pub fn allow_paid_broadcast(mut self, v: bool) -> Self {
        self.opts.allow_paid_broadcast = Some(v);
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.opts.reply_parameters = Some(rp);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.opts.reply_markup = Some(m.into());
        self
    }
}

impl IntoFuture for SendPhoto {
    type Output = Result<Message>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            match &self.photo {
                InputFile::Bytes {
                    filename,
                    data,
                    mime_type,
                } => {
                    let part = Part::bytes(data.clone())
                        .file_name(filename.clone())
                        .mime_str(mime_type)
                        .map_err(|e| crate::error::Error::Decode(e.to_string()))?;
                    let mut form = Form::new().part("photo", part);
                    form = form.text("chat_id", self.chat_id.to_string());
                    if let Some(c) = &self.opts.caption {
                        form = form.text("caption", c.clone());
                    }
                    if let Some(m) = &self.opts.parse_mode {
                        form = form.text("parse_mode", format!("{m:?}"));
                    }
                    if let Some(v) = self.opts.disable_notification {
                        form = form.text("disable_notification", v.to_string());
                    }
                    if let Some(v) = self.opts.has_spoiler {
                        form = form.text("has_spoiler", v.to_string());
                    }
                    if let Some(v) = &self.opts.reply_markup {
                        form = form.text("reply_markup", serde_json::to_string(v).unwrap());
                    }
                    self.client.post_multipart("sendPhoto", form).await
                }
                _ => {
                    let body = media_json_body(
                        &self.chat_id,
                        "photo",
                        self.photo.as_str(),
                        &self.opts,
                        serde_json::Value::Null,
                    );
                    self.client.post_json("sendPhoto", &body).await
                }
            }
        })
    }
}

// ─── Macro for simpler media senders (Audio, Document, Video, Animation, Voice, VideoNote, Sticker)

macro_rules! media_sender {
    ($(#[$doc:meta])* $name:ident, $field:literal, $method:literal, $return_ty:ty, [$($extra_field:ident: $extra_ty:ty),*]) => {
        $(#[$doc])*
        pub struct $name {
            /// The API client to use for sending the request.
            client: BotClient,
            /// Unique identifier for the target chat or username of the target channel.
            chat_id: ChatId,
            /// The file to send. Can be a file ID, URL, or new upload.
            file: InputFile,
            /// Common optional parameters for media sending.
            opts: MediaSendOptions,
            /// Extra optional parameters specific to this media type.
            $($extra_field: Option<$extra_ty>,)*
        }

        impl $name {
            pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, file: InputFile) -> Self {
                Self {
                    client,
                    chat_id: chat_id.into(),
                    file,
                    opts: MediaSendOptions::default(),
                    $($extra_field: None,)*
                }
            }
            /// Sets the caption (0–1024 characters) for media messages.
            pub fn caption(mut self, c: impl Into<String>) -> Self { self.opts.caption = Some(c.into()); self }
            /// Sets the caption parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
            pub fn parse_mode(mut self, m: ParseMode) -> Self { self.opts.parse_mode = Some(m); self }
            /// Sends the message silently — the recipient receives no notification sound.
            pub fn disable_notification(mut self, v: bool) -> Self { self.opts.disable_notification = Some(v); self }
            /// Protects the message from being forwarded or saved.
            pub fn protect_content(mut self, v: bool) -> Self { self.opts.protect_content = Some(v); self }
            /// Allows sending to large audiences at the cost of Telegram Stars.
            pub fn allow_paid_broadcast(mut self, v: bool) -> Self { self.opts.allow_paid_broadcast = Some(v); self }
            /// Reply parameters for this message.
            pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self { self.opts.reply_parameters = Some(rp); self }
            /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
            pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self { self.opts.reply_markup = Some(m.into()); self }
        }

        impl IntoFuture for $name {
            type Output = Result<$return_ty>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    match &self.file {
                        InputFile::Bytes { filename, data, mime_type } => {
                            let part = Part::bytes(data.clone())
                                .file_name(filename.clone())
                                .mime_str(mime_type)
                                .map_err(|e| crate::error::Error::Decode(e.to_string()))?;
                            let mut form = Form::new().part($field, part);
                            form = form.text("chat_id", self.chat_id.to_string());
                            if let Some(c) = &self.opts.caption { form = form.text("caption", c.clone()); }
                            if let Some(v) = self.opts.disable_notification { form = form.text("disable_notification", v.to_string()); }
                            if let Some(v) = &self.opts.reply_markup { form = form.text("reply_markup", serde_json::to_string(v).unwrap()); }
                            self.client.post_multipart($method, form).await
                        }
                        _ => {
                            let mut extra = serde_json::json!({});
                            $(
                                if let Some(ref v) = self.$extra_field {
                                    extra[stringify!($extra_field)] = serde_json::json!(v);
                                }
                            )*
                            let body = media_json_body(&self.chat_id, $field, self.file.as_str(), &self.opts, extra);
                            self.client.post_json($method, &body).await
                        }
                    }
                })
            }
        }
    };
}

media_sender!(
    /// Builder for the [`sendAudio`](https://core.telegram.org/bots/api#sendaudio) method.
    SendAudio,     "audio",      "sendAudio",     Message, [duration: u32, performer: String, title: String]);
media_sender!(
    /// Builder for the [`sendDocument`](https://core.telegram.org/bots/api#senddocument) method.
    SendDocument,  "document",   "sendDocument",  Message, [disable_content_type_detection: bool]);
media_sender!(
    /// Builder for the [`sendVideo`](https://core.telegram.org/bots/api#sendvideo) method.
    SendVideo,     "video",      "sendVideo",     Message, [duration: u32, width: u32, height: u32, supports_streaming: bool]);
media_sender!(
    /// Builder for the [`sendAnimation`](https://core.telegram.org/bots/api#sendanimation) method.
    SendAnimation, "animation",  "sendAnimation", Message, [duration: u32, width: u32, height: u32]);
media_sender!(
    /// Builder for the [`sendVoice`](https://core.telegram.org/bots/api#sendvoice) method.
    SendVoice,     "voice",      "sendVoice",     Message, [duration: u32]);
media_sender!(
    /// Builder for the [`sendVideoNote`](https://core.telegram.org/bots/api#sendvideonote) method.
    SendVideoNote, "video_note", "sendVideoNote", Message, [duration: u32, length: u32]);
media_sender!(
    /// Builder for the [`sendSticker`](https://core.telegram.org/bots/api#sendsticker) method.
    SendSticker,   "sticker",    "sendSticker",   Message, [emoji: String]);

// ─── deleteMessage / deleteMessages ──────────────────────────────────────────

#[derive(Serialize)]
struct DeleteMessageParams {
    chat_id: ChatId,
    message_id: i64,
}

/// Builder for the [`deleteMessage`](https://core.telegram.org/bots/api#deletemessage) method.
pub struct DeleteMessage {
    client: BotClient,
    params: DeleteMessageParams,
}
impl DeleteMessage {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: DeleteMessageParams {
                chat_id: chat_id.into(),
                message_id,
            },
        }
    }
}
impl_into_future!(DeleteMessage, bool, "deleteMessage");

#[derive(Serialize)]
struct DeleteMessagesParams {
    chat_id: ChatId,
    message_ids: Vec<i64>,
}

/// Builder for the [`deleteMessages`](https://core.telegram.org/bots/api#deletemessages) method.
pub struct DeleteMessages {
    client: BotClient,
    params: DeleteMessagesParams,
}
impl DeleteMessages {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            client,
            params: DeleteMessagesParams {
                chat_id: chat_id.into(),
                message_ids,
            },
        }
    }
}
impl_into_future!(DeleteMessages, bool, "deleteMessages");

// ─── stopPoll ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct StopPollParams {
    chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<rustigram_types::keyboard::InlineKeyboardMarkup>,
}

/// Builder for the [`stopPoll`](https://core.telegram.org/bots/api#stoppoll) method.
pub struct StopPoll {
    client: BotClient,
    params: StopPollParams,
}
impl StopPoll {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: StopPollParams {
                chat_id: chat_id.into(),
                message_id,
                reply_markup: None,
            },
        }
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: rustigram_types::keyboard::InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}
impl_into_future!(StopPoll, rustigram_types::poll::Poll, "stopPoll");

// ─── answerCallbackQuery ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct AnswerCallbackQueryParams {
    callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u32>,
}

/// Builder for the [`answerCallbackQuery`](https://core.telegram.org/bots/api#answercallbackquery) method.
pub struct AnswerCallbackQuery {
    client: BotClient,
    params: AnswerCallbackQueryParams,
}
impl AnswerCallbackQuery {
    pub(crate) fn new(client: BotClient, callback_query_id: impl Into<String>) -> Self {
        Self {
            client,
            params: AnswerCallbackQueryParams {
                callback_query_id: callback_query_id.into(),
                text: None,
                show_alert: None,
                url: None,
                cache_time: None,
            },
        }
    }
    /// The text of the notification shown to the user. 0–200 characters.
    pub fn text(mut self, t: impl Into<String>) -> Self {
        self.params.text = Some(t.into());
        self
    }
    /// Shows an alert dialog instead of a toast notification for the callback answer.
    pub fn show_alert(mut self, v: bool) -> Self {
        self.params.show_alert = Some(v);
        self
    }
    /// Sets the URL to open when the callback button answer is tapped.
    pub fn url(mut self, u: impl Into<String>) -> Self {
        self.params.url = Some(u.into());
        self
    }
    /// Sets how long the callback answer may be cached on the client in seconds.
    pub fn cache_time(mut self, secs: u32) -> Self {
        self.params.cache_time = Some(secs);
        self
    }
    /// Shorthand for `.text(t).show_alert(true)` — shows a popup alert to the user.
    pub fn alert(self, text: impl Into<String>) -> Self {
        self.text(text).show_alert(true)
    }
}
impl_into_future!(AnswerCallbackQuery, bool, "answerCallbackQuery");
// ─── forwardMessages ──────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ForwardMessagesParams {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
}

/// Builder for the [`forwardMessages`](https://core.telegram.org/bots/api#forwardmessages) method.
///
/// Forwards 1–100 messages at once, preserving album grouping.
/// Returns a `Vec<MessageId>` of the sent messages.
pub struct ForwardMessages {
    client: BotClient,
    params: ForwardMessagesParams,
}

impl ForwardMessages {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            client,
            params: ForwardMessagesParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_ids,
                message_thread_id: None,
                disable_notification: None,
                protect_content: None,
            },
        }
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Sends the messages silently — recipients receive no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the messages from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl_into_future!(
    ForwardMessages,
    Vec<rustigram_types::message::MessageId>,
    "forwardMessages"
);

// ─── copyMessages ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CopyMessagesParams {
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_caption: Option<bool>,
}

/// Builder for the [`copyMessages`](https://core.telegram.org/bots/api#copymessages) method.
///
/// Copies 1–100 messages without a forward link, preserving album grouping.
/// Returns a `Vec<MessageId>` of the sent messages.
pub struct CopyMessages {
    client: BotClient,
    params: CopyMessagesParams,
}

impl CopyMessages {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        from_chat_id: impl Into<ChatId>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            client,
            params: CopyMessagesParams {
                chat_id: chat_id.into(),
                from_chat_id: from_chat_id.into(),
                message_ids,
                message_thread_id: None,
                disable_notification: None,
                protect_content: None,
                remove_caption: None,
            },
        }
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Sends the messages silently — recipients receive no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the messages from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Copies the messages without their captions.
    pub fn remove_caption(mut self, v: bool) -> Self {
        self.params.remove_caption = Some(v);
        self
    }
}

impl_into_future!(
    CopyMessages,
    Vec<rustigram_types::message::MessageId>,
    "copyMessages"
);

// ─── sendVenue ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendVenueParams {
    chat_id: ChatId,
    latitude: f64,
    longitude: f64,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendVenue`](https://core.telegram.org/bots/api#sendvenue) method.
pub struct SendVenue {
    client: BotClient,
    params: SendVenueParams,
}

impl SendVenue {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SendVenueParams {
                chat_id: chat_id.into(),
                latitude,
                longitude,
                title: title.into(),
                address: address.into(),
                message_thread_id: None,
                foursquare_id: None,
                foursquare_type: None,
                google_place_id: None,
                google_place_type: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Sets the Foursquare identifier of the venue.
    pub fn foursquare_id(mut self, id: impl Into<String>) -> Self {
        self.params.foursquare_id = Some(id.into());
        self
    }
    /// Sets the Foursquare type of the venue (e.g. `"arts_entertainment/aquarium"`).
    pub fn foursquare_type(mut self, t: impl Into<String>) -> Self {
        self.params.foursquare_type = Some(t.into());
        self
    }
    /// Sets the Google Places identifier of the venue.
    pub fn google_place_id(mut self, id: impl Into<String>) -> Self {
        self.params.google_place_id = Some(id.into());
        self
    }
    /// Sets the Google Places type of the venue.
    pub fn google_place_type(mut self, t: impl Into<String>) -> Self {
        self.params.google_place_type = Some(t.into());
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(rp);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(SendVenue, Message, "sendVenue");

// ─── sendMediaGroup ───────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendMediaGroupParams {
    chat_id: ChatId,
    /// Array of `InputMedia` objects (photo, video, audio, or document).
    ///
    /// Uses `serde_json::Value` until the `InputMedia` enum is defined in
    /// Priority 4. Pass the result of `serde_json::to_value(&your_input_media_vec)`.
    media: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
}

/// Builder for the [`sendMediaGroup`](https://core.telegram.org/bots/api#sendmediagroup) method.
///
/// Sends a group of photos, videos, documents, or audios as an album (2–10 items).
///
/// The `media` parameter accepts `Vec<serde_json::Value>` until the `InputMedia`
/// enum is defined in Priority 4. Construct items with `serde_json::json!({...})`
/// or `serde_json::to_value(&input_media)`.
pub struct SendMediaGroup {
    client: BotClient,
    params: SendMediaGroupParams,
}

impl SendMediaGroup {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        media: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            client,
            params: SendMediaGroupParams {
                chat_id: chat_id.into(),
                media,
                message_thread_id: None,
                business_connection_id: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
            },
        }
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Business connection ID for sending on behalf of a business account.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Sends the messages silently — recipients receive no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the messages from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(rp);
        self
    }
}

impl_into_future!(SendMediaGroup, Vec<Message>, "sendMediaGroup");

// ─── sendPaidMedia ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendPaidMediaParams {
    chat_id: ChatId,
    star_count: u32,
    /// Array of `InputPaidMedia` objects (photo or video).
    ///
    /// Uses `serde_json::Value` until the `InputPaidMedia` enum is defined in
    /// Priority 4. Pass the result of `serde_json::to_value(&your_paid_media_vec)`.
    media: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

/// Builder for the [`sendPaidMedia`](https://core.telegram.org/bots/api#sendpaidmedia) method.
///
/// Sends paid media that users must pay Telegram Stars to view (up to 10 items).
///
/// The `media` parameter accepts `Vec<serde_json::Value>` until the `InputPaidMedia`
/// enum is defined in Priority 4.
pub struct SendPaidMedia {
    client: BotClient,
    params: SendPaidMediaParams,
}

impl SendPaidMedia {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        star_count: u32,
        media: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            client,
            params: SendPaidMediaParams {
                chat_id: chat_id.into(),
                star_count,
                media,
                business_connection_id: None,
                payload: None,
                caption: None,
                parse_mode: None,
                show_caption_above_media: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Business connection ID for sending on behalf of a business account.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Bot-defined paid media payload (0–128 bytes); not shown to the user.
    pub fn payload(mut self, p: impl Into<String>) -> Self {
        self.params.payload = Some(p.into());
        self
    }
    /// Sets the caption (0–1024 characters).
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
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(rp);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(m.into());
        self
    }
}

impl_into_future!(SendPaidMedia, Message, "sendPaidMedia");

// ─── sendGame ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendGameParams {
    chat_id: i64,
    game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<rustigram_types::keyboard::InlineKeyboardMarkup>,
}

/// Builder for the [`sendGame`](https://core.telegram.org/bots/api#sendgame) method.
///
/// Note: `chat_id` is an integer — games can't be sent to channel direct messages
/// chats or channel chats.
pub struct SendGame {
    client: BotClient,
    params: SendGameParams,
}

impl SendGame {
    pub(crate) fn new(client: BotClient, chat_id: i64, game_short_name: impl Into<String>) -> Self {
        Self {
            client,
            params: SendGameParams {
                chat_id,
                game_short_name: game_short_name.into(),
                business_connection_id: None,
                message_thread_id: None,
                disable_notification: None,
                protect_content: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Business connection ID for sending on behalf of a business account.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(rp);
        self
    }
    /// Attaches an inline keyboard. The first button must launch the game.
    pub fn reply_markup(mut self, m: rustigram_types::keyboard::InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(SendGame, Message, "sendGame");

// ─── sendChecklist ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendChecklistParams {
    business_connection_id: String,
    chat_id: i64,
    checklist: rustigram_types::checklist::InputChecklist,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<rustigram_types::keyboard::InlineKeyboardMarkup>,
}

/// Builder for the [`sendChecklist`](https://core.telegram.org/bots/api#sendchecklist) method.
///
/// Business bots only — sends a checklist on behalf of a connected business account.
/// Requires the `can_reply` business bot right.
pub struct SendChecklist {
    client: BotClient,
    params: SendChecklistParams,
}

impl SendChecklist {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: rustigram_types::checklist::InputChecklist,
    ) -> Self {
        Self {
            client,
            params: SendChecklistParams {
                business_connection_id: business_connection_id.into(),
                chat_id,
                checklist,
                disable_notification: None,
                protect_content: None,
                message_effect_id: None,
                reply_parameters: None,
                reply_markup: None,
            },
        }
    }
    /// Sends the message silently — the recipient receives no notification sound.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
    /// Protects the message from being forwarded or saved.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
    /// Unique identifier of the message effect to add to the message.
    pub fn message_effect_id(mut self, id: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(id.into());
        self
    }
    /// Reply parameters for this message.
    pub fn reply_parameters(mut self, rp: ReplyParameters) -> Self {
        self.params.reply_parameters = Some(rp);
        self
    }
    /// Attaches an inline keyboard to the message.
    pub fn reply_markup(mut self, m: rustigram_types::keyboard::InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}

impl_into_future!(SendChecklist, Message, "sendChecklist");
