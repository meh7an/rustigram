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
