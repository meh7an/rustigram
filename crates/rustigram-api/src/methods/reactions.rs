use crate::client::BotClient;
use rustigram_types::message::ReactionType;
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Serialize)]
struct SetMessageReactionParams {
    chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reaction: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_big: Option<bool>,
}

/// Builder for the [`setMessageReaction`](https://core.telegram.org/bots/api#setmessagereaction) method.
pub struct SetMessageReaction {
    client: BotClient,
    params: SetMessageReactionParams,
}
impl SetMessageReaction {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: SetMessageReactionParams {
                chat_id: chat_id.into(),
                message_id,
                reaction: None,
                is_big: None,
            },
        }
    }
    /// Sets the list of reaction types to apply to the message.
    /// Pass an empty list to remove all reactions.
    pub fn reaction(mut self, r: Vec<ReactionType>) -> Self {
        self.params.reaction = Some(r);
        self
    }
    /// Shows a larger animated reaction when `true`.
    pub fn is_big(mut self, v: bool) -> Self {
        self.params.is_big = Some(v);
        self
    }
    /// Convenience shortcut — sets a single emoji reaction by its emoji string.
    pub fn emoji(self, emoji: impl Into<String>) -> Self {
        self.reaction(vec![ReactionType::Emoji {
            emoji: emoji.into(),
        }])
    }
}
impl IntoFuture for SetMessageReaction {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setMessageReaction", &self.params)
                .await
        })
    }
}

// ─── deleteMessageReaction ────────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteMessageReactionParams {
    chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor_chat_id: Option<i64>,
}

/// Builder for the [`deleteMessageReaction`](https://core.telegram.org/bots/api#deletemessagereaction) method.
///
/// Removes a reaction from a message in a group or supergroup. The bot must have the
/// `can_delete_messages` administrator right.
pub struct DeleteMessageReaction {
    client: BotClient,
    params: DeleteMessageReactionParams,
}

impl DeleteMessageReaction {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: DeleteMessageReactionParams {
                chat_id: chat_id.into(),
                message_id,
                user_id: None,
                actor_chat_id: None,
            },
        }
    }

    /// Identifier of the user whose reaction will be removed, if added by a user.
    pub fn user_id(mut self, id: i64) -> Self {
        self.params.user_id = Some(id);
        self
    }

    /// Identifier of the chat whose reaction will be removed, if added by a chat.
    pub fn actor_chat_id(mut self, id: i64) -> Self {
        self.params.actor_chat_id = Some(id);
        self
    }
}

impl IntoFuture for DeleteMessageReaction {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("deleteMessageReaction", &self.params)
                .await
        })
    }
}

// ─── deleteAllMessageReactions ────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteAllMessageReactionsParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor_chat_id: Option<i64>,
}

/// Builder for the [`deleteAllMessageReactions`](https://core.telegram.org/bots/api#deleteallmessagereactions) method.
///
/// Removes all recent reactions in a group or supergroup added by a given user or chat.
/// The bot must have the `can_delete_messages` administrator right.
pub struct DeleteAllMessageReactions {
    client: BotClient,
    params: DeleteAllMessageReactionsParams,
}

impl DeleteAllMessageReactions {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: DeleteAllMessageReactionsParams {
                chat_id: chat_id.into(),
                user_id: None,
                actor_chat_id: None,
            },
        }
    }

    /// Identifier of the user whose reactions will be removed, if added by a user.
    pub fn user_id(mut self, id: i64) -> Self {
        self.params.user_id = Some(id);
        self
    }

    /// Identifier of the chat whose reactions will be removed, if added by a chat.
    pub fn actor_chat_id(mut self, id: i64) -> Self {
        self.params.actor_chat_id = Some(id);
        self
    }
}

impl IntoFuture for DeleteAllMessageReactions {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("deleteAllMessageReactions", &self.params)
                .await
        })
    }
}
