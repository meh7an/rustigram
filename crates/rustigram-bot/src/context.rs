use std::sync::Arc;

use rustigram_api::BotClient;
use rustigram_types::inline::InlineQuery;
use rustigram_types::message::Message;
use rustigram_types::update::CallbackQuery;
use rustigram_types::update::{Update, UpdateKind};
use rustigram_types::user::ChatId;

/// The context object passed to every handler.
///
/// Contains the incoming [`Update`], the [`BotClient`] (ready to make API
/// calls), and a reference to any shared bot-level state.
#[derive(Clone)]
/// The context object passed to every handler.
///
/// `Context` bundles the incoming [`Update`] with the [`BotClient`] so
/// handlers have everything they need in a single value.
///
/// # Accessing the update
///
/// ```rust,ignore
/// async fn handler(ctx: Context) -> BotResult<()> {
///     // Convenience accessors
///     let text    = ctx.text();       // message text or caption
///     let cmd     = ctx.command();    // "/start" → Some("start")
///     let chat_id = ctx.chat_id();
///     let user_id = ctx.from_id();
///
///     // Raw update for anything not covered by the helpers
///     let update = &ctx.update;
///     Ok(())
/// }
/// ```
///
/// # Sending replies
///
/// `ctx.reply(text)` is a shortcut that sends a message to the current chat
/// and automatically sets `reply_to_message_id`. It returns `None` when the
/// update has no associated chat (e.g. inline queries).
///
/// ```rust,ignore
/// if let Some(reply) = ctx.reply("Got it!") {
///     reply.parse_mode(ParseMode::HTML).await?;
/// }
/// ```
pub struct Context {
    /// The incoming update that triggered this handler.
    pub update: Arc<Update>,

    /// The API client — call any Bot API method directly.
    pub bot: BotClient,
}

impl Context {
    /// Creates a new `Context`.
    #[must_use]
    pub fn new(update: Update, bot: BotClient) -> Self {
        Self {
            update: Arc::new(update),
            bot,
        }
    }

    /// Returns the update ID.
    #[must_use]
    pub fn update_id(&self) -> i64 {
        self.update.update_id
    }

    /// Returns the [`Message`] from a message, edited message, channel post,
    /// or callback query update. Returns `None` for all other update types.
    #[must_use]
    pub fn message(&self) -> Option<&Message> {
        match &self.update.kind {
            UpdateKind::Message(m)
            | UpdateKind::EditedMessage(m)
            | UpdateKind::ChannelPost(m)
            | UpdateKind::EditedChannelPost(m)
            | UpdateKind::BusinessMessage(m)
            | UpdateKind::EditedBusinessMessage(m) => Some(m),
            UpdateKind::CallbackQuery(q) => q.message.as_ref(),
            _ => None,
        }
    }

    /// Returns the chat ID from the current update as a [`ChatId`], if available.
    #[must_use]
    pub fn chat_id(&self) -> Option<ChatId> {
        self.update.chat_id().map(ChatId::Id)
    }

    /// Returns the sender's user ID, if available.
    #[must_use]
    pub fn from_id(&self) -> Option<i64> {
        self.update.from().map(|u| u.id)
    }

    /// Returns the [`CallbackQuery`] if this is a callback query update.
    #[must_use]
    pub fn callback_query(&self) -> Option<&CallbackQuery> {
        match &self.update.kind {
            UpdateKind::CallbackQuery(q) => Some(q),
            _ => None,
        }
    }

    /// Returns the [`InlineQuery`] if this is an inline query update.
    #[must_use]
    pub fn inline_query(&self) -> Option<&InlineQuery> {
        match &self.update.kind {
            UpdateKind::InlineQuery(q) => Some(q),
            _ => None,
        }
    }

    /// Returns the effective text of the message — [`Message::text`] if present,
    /// falling back to [`Message::caption`].
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.message().and_then(|m| m.effective_text())
    }

    /// Returns the command name if the message starts with a bot command entity.
    ///
    /// The leading `/` and optional `@BotName` suffix are stripped automatically.
    /// `/start@mybot` returns `Some("start")`.
    #[must_use]
    pub fn command(&self) -> Option<&str> {
        self.message().and_then(|m| m.command())
    }

    /// Sends a text reply to the current chat, automatically setting
    /// `reply_to_message_id` to the incoming message.
    ///
    /// Returns `None` when the update has no associated chat.
    pub fn reply(
        &self,
        text: impl Into<String>,
    ) -> Option<rustigram_api::methods::sending::SendMessage> {
        let chat_id = self.chat_id()?;
        let mut builder = self.bot.send_message(chat_id, text);
        if let Some(msg) = self.message() {
            builder = builder.reply_to(msg.message_id);
        }
        Some(builder)
    }
}
