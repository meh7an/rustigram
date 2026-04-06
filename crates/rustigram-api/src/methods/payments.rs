use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::payments::{LabeledPrice, StarAmount, StarTransactions};
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Serialize)]
struct SendInvoiceParams {
    chat_id: ChatId,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<rustigram_types::keyboard::InlineKeyboardMarkup>,
}

/// Builder for the [`sendInvoice`](https://core.telegram.org/bots/api#sendinvoice) method.
pub struct SendInvoice {
    client: BotClient,
    params: SendInvoiceParams,
}
impl SendInvoice {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            client,
            params: SendInvoiceParams {
                chat_id: chat_id.into(),
                title: title.into(),
                description: description.into(),
                payload: payload.into(),
                currency: currency.into(),
                prices,
                provider_token: None,
                max_tip_amount: None,
                suggested_tip_amounts: None,
                photo_url: None,
                need_name: None,
                need_phone_number: None,
                need_email: None,
                need_shipping_address: None,
                is_flexible: None,
                disable_notification: None,
                protect_content: None,
                reply_markup: None,
            },
        }
    }
    /// Sets the payment provider token. Not required for Telegram Stars (`XTR`).
    pub fn provider_token(mut self, t: impl Into<String>) -> Self {
        self.params.provider_token = Some(t.into());
        self
    }
    /// Requests the buyer's full name during checkout.
    pub fn need_name(mut self, v: bool) -> Self {
        self.params.need_name = Some(v);
        self
    }
    /// Requests the buyer's shipping address during checkout.
    pub fn need_shipping_address(mut self, v: bool) -> Self {
        self.params.need_shipping_address = Some(v);
        self
    }
    /// Indicates that the final price depends on the shipping method.
    pub fn is_flexible(mut self, v: bool) -> Self {
        self.params.is_flexible = Some(v);
        self
    }
    /// Attaches a reply markup (inline keyboard, reply keyboard, etc.).
    pub fn reply_markup(mut self, m: rustigram_types::keyboard::InlineKeyboardMarkup) -> Self {
        self.params.reply_markup = Some(m);
        self
    }
}
impl IntoFuture for SendInvoice {
    type Output = Result<rustigram_types::message::Message>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("sendInvoice", &self.params).await })
    }
}

/// Builder for the [`getMyStarBalance`](https://core.telegram.org/bots/api#getmystarbalance) method.
pub struct GetMyStarBalance {
    client: BotClient,
}
impl GetMyStarBalance {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}
impl IntoFuture for GetMyStarBalance {
    type Output = Result<StarAmount>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getMyStarBalance", &serde_json::json!({}))
                .await
        })
    }
}

#[derive(Serialize, Default)]
struct GetStarTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

/// Builder for the [`getStarTransactions`](https://core.telegram.org/bots/api#getstartransactions) method.
pub struct GetStarTransactions {
    client: BotClient,
    params: GetStarTransactionsParams,
}
impl GetStarTransactions {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Skips the first N transactions in the result.
    pub fn offset(mut self, v: u32) -> Self {
        self.params.offset = Some(v);
        self
    }
    /// Limits the number of transactions returned.
    pub fn limit(mut self, v: u32) -> Self {
        self.params.limit = Some(v);
        self
    }
}
impl IntoFuture for GetStarTransactions {
    type Output = Result<StarTransactions>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getStarTransactions", &self.params)
                .await
        })
    }
}
