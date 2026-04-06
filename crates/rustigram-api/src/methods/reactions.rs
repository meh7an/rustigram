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
