use crate::client::BotClient;
use rustigram_types::inline::InlineQueryResult;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Serialize)]
struct AnswerInlineQueryParams {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<serde_json::Value>,
}

/// Builder for the [`answerInlineQuery`](https://core.telegram.org/bots/api#answerinlinequery) method.
pub struct AnswerInlineQuery {
    client: BotClient,
    params: AnswerInlineQueryParams,
}
impl AnswerInlineQuery {
    pub(crate) fn new(
        client: BotClient,
        inline_query_id: impl Into<String>,
        results: Vec<InlineQueryResult>,
    ) -> Self {
        Self {
            client,
            params: AnswerInlineQueryParams {
                inline_query_id: inline_query_id.into(),
                results,
                cache_time: None,
                is_personal: None,
                next_offset: None,
                button: None,
            },
        }
    }
    /// Sets how many seconds the results may be cached on the client (default 300).
    pub fn cache_time(mut self, secs: u32) -> Self {
        self.params.cache_time = Some(secs);
        self
    }
    /// Makes the results personal to the user — disables shared caching.
    pub fn is_personal(mut self, v: bool) -> Self {
        self.params.is_personal = Some(v);
        self
    }
    /// Sets the offset for pagination when there are more results available.
    pub fn next_offset(mut self, o: impl Into<String>) -> Self {
        self.params.next_offset = Some(o.into());
        self
    }
}
impl IntoFuture for AnswerInlineQuery {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("answerInlineQuery", &self.params)
                .await
        })
    }
}

// ─── answerWebAppQuery ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct AnswerWebAppQueryParams {
    web_app_query_id: String,
    result: InlineQueryResult,
}

/// Builder for the [`answerWebAppQuery`](https://core.telegram.org/bots/api#answerwebappquery) method.
///
/// Sets the result of an interaction with a Web App and sends the corresponding
/// message on behalf of the user to the originating chat.
/// Returns a `SentWebAppMessage` object.
pub struct AnswerWebAppQuery {
    client: BotClient,
    params: AnswerWebAppQueryParams,
}

impl AnswerWebAppQuery {
    pub(crate) fn new(
        client: BotClient,
        web_app_query_id: impl Into<String>,
        result: InlineQueryResult,
    ) -> Self {
        Self {
            client,
            params: AnswerWebAppQueryParams {
                web_app_query_id: web_app_query_id.into(),
                result,
            },
        }
    }
}

impl IntoFuture for AnswerWebAppQuery {
    /// Returns `SentWebAppMessage` as `serde_json::Value` until the type is defined in Priority 4.
    type Output = crate::error::Result<serde_json::Value>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("answerWebAppQuery", &self.params)
                .await
        })
    }
}

// ─── savePreparedInlineMessage ────────────────────────────────────────────────

#[derive(Serialize)]
struct SavePreparedInlineMessageParams {
    user_id: i64,
    result: InlineQueryResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_channel_chats: Option<bool>,
}

/// Builder for the [`savePreparedInlineMessage`](https://core.telegram.org/bots/api#savepreparedinlinemessage) method.
///
/// Stores a message that can be sent by a user from a Mini App.
/// Returns a `PreparedInlineMessage` object as `serde_json::Value` until the type
/// is defined in Priority 4.
pub struct SavePreparedInlineMessage {
    client: BotClient,
    params: SavePreparedInlineMessageParams,
}

impl SavePreparedInlineMessage {
    pub(crate) fn new(client: BotClient, user_id: i64, result: InlineQueryResult) -> Self {
        Self {
            client,
            params: SavePreparedInlineMessageParams {
                user_id,
                result,
                allow_user_chats: None,
                allow_bot_chats: None,
                allow_group_chats: None,
                allow_channel_chats: None,
            },
        }
    }
    /// Allows the message to be sent to private chats with users.
    pub fn allow_user_chats(mut self, v: bool) -> Self {
        self.params.allow_user_chats = Some(v);
        self
    }
    /// Allows the message to be sent to private chats with bots.
    pub fn allow_bot_chats(mut self, v: bool) -> Self {
        self.params.allow_bot_chats = Some(v);
        self
    }
    /// Allows the message to be sent to group and supergroup chats.
    pub fn allow_group_chats(mut self, v: bool) -> Self {
        self.params.allow_group_chats = Some(v);
        self
    }
    /// Allows the message to be sent to channel chats.
    pub fn allow_channel_chats(mut self, v: bool) -> Self {
        self.params.allow_channel_chats = Some(v);
        self
    }
}

impl IntoFuture for SavePreparedInlineMessage {
    /// Returns `PreparedInlineMessage` as `serde_json::Value` until the type is defined in Priority 4.
    type Output = crate::error::Result<serde_json::Value>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("savePreparedInlineMessage", &self.params)
                .await
        })
    }
}
