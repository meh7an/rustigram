use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::message::{Message, ReplyParameters};
use rustigram_types::payments::{LabeledPrice, ShippingOption, StarAmount, StarTransactions};
use rustigram_types::suggested_post::SuggestedPostParameters;
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

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

// ─── sendInvoice ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendInvoiceParams {
    chat_id: ChatId,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_messages_topic_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<rustigram_types::keyboard::InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_post_parameters: Option<SuggestedPostParameters>,
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
                message_thread_id: None,
                direct_messages_topic_id: None,
                provider_token: None,
                max_tip_amount: None,
                suggested_tip_amounts: None,
                start_parameter: None,
                provider_data: None,
                photo_url: None,
                photo_size: None,
                photo_width: None,
                photo_height: None,
                need_name: None,
                need_phone_number: None,
                need_email: None,
                need_shipping_address: None,
                send_phone_number_to_provider: None,
                send_email_to_provider: None,
                is_flexible: None,
                disable_notification: None,
                protect_content: None,
                allow_paid_broadcast: None,
                reply_parameters: None,
                reply_markup: None,
                suggested_post_parameters: None,
            },
        }
    }
    /// Forum topic thread ID.
    pub fn message_thread_id(mut self, id: i64) -> Self {
        self.params.message_thread_id = Some(id);
        self
    }
    /// Identifier of a direct messages chat topic.
    pub fn direct_messages_topic_id(mut self, id: i64) -> Self {
        self.params.direct_messages_topic_id = Some(id);
        self
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
    /// Suggested post parameters for channel direct messages chats.
    pub fn suggested_post_parameters(mut self, params: SuggestedPostParameters) -> Self {
        self.params.suggested_post_parameters = Some(params);
        self
    }
}

impl_into_future!(SendInvoice, Message, "sendInvoice");

// ─── createInvoiceLink ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CreateInvoiceLinkParams {
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
}

/// Builder for the [`createInvoiceLink`](https://core.telegram.org/bots/api#createinvoicelink) method.
///
/// Creates a shareable payment link. Returns the link as a `String`.
pub struct CreateInvoiceLink {
    client: BotClient,
    params: CreateInvoiceLinkParams,
}

impl CreateInvoiceLink {
    pub(crate) fn new(
        client: BotClient,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            client,
            params: CreateInvoiceLinkParams {
                title: title.into(),
                description: description.into(),
                payload: payload.into(),
                currency: currency.into(),
                prices,
                business_connection_id: None,
                provider_token: None,
                subscription_period: None,
                max_tip_amount: None,
                suggested_tip_amounts: None,
                provider_data: None,
                photo_url: None,
                photo_size: None,
                photo_width: None,
                photo_height: None,
                need_name: None,
                need_phone_number: None,
                need_email: None,
                need_shipping_address: None,
                send_phone_number_to_provider: None,
                send_email_to_provider: None,
                is_flexible: None,
            },
        }
    }
    /// Business connection ID; for Telegram Stars payments only.
    pub fn business_connection_id(mut self, id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(id.into());
        self
    }
    /// Sets the payment provider token. Not required for Telegram Stars (`XTR`).
    pub fn provider_token(mut self, t: impl Into<String>) -> Self {
        self.params.provider_token = Some(t.into());
        self
    }
    /// The number of seconds the subscription will be active before the next payment.
    /// Currently must always be `2592000` (30 days) if specified.
    pub fn subscription_period(mut self, secs: i64) -> Self {
        self.params.subscription_period = Some(secs);
        self
    }
    /// Sets the maximum accepted tip amount in the smallest currency units.
    pub fn max_tip_amount(mut self, v: i64) -> Self {
        self.params.max_tip_amount = Some(v);
        self
    }
    /// Sets the URL of the product photo.
    pub fn photo_url(mut self, url: impl Into<String>) -> Self {
        self.params.photo_url = Some(url.into());
        self
    }
    /// Requests the buyer's full name during checkout.
    pub fn need_name(mut self, v: bool) -> Self {
        self.params.need_name = Some(v);
        self
    }
    /// Requests the buyer's phone number during checkout.
    pub fn need_phone_number(mut self, v: bool) -> Self {
        self.params.need_phone_number = Some(v);
        self
    }
    /// Requests the buyer's email address during checkout.
    pub fn need_email(mut self, v: bool) -> Self {
        self.params.need_email = Some(v);
        self
    }
    /// Requests the buyer's shipping address during checkout.
    pub fn need_shipping_address(mut self, v: bool) -> Self {
        self.params.need_shipping_address = Some(v);
        self
    }
    /// Forwards the buyer's phone number to the provider.
    pub fn send_phone_number_to_provider(mut self, v: bool) -> Self {
        self.params.send_phone_number_to_provider = Some(v);
        self
    }
    /// Forwards the buyer's email address to the provider.
    pub fn send_email_to_provider(mut self, v: bool) -> Self {
        self.params.send_email_to_provider = Some(v);
        self
    }
    /// Indicates that the final price depends on the shipping method.
    pub fn is_flexible(mut self, v: bool) -> Self {
        self.params.is_flexible = Some(v);
        self
    }
}

impl_into_future!(CreateInvoiceLink, String, "createInvoiceLink");

// ─── answerShippingQuery ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct AnswerShippingQueryParams {
    shipping_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

/// Builder for the [`answerShippingQuery`](https://core.telegram.org/bots/api#answershippingquery) method.
///
/// Respond to a shipping query from a user. Must be called when the invoice
/// has `is_flexible = true`.
pub struct AnswerShippingQuery {
    client: BotClient,
    params: AnswerShippingQueryParams,
}

impl AnswerShippingQuery {
    pub(crate) fn new(client: BotClient, shipping_query_id: impl Into<String>, ok: bool) -> Self {
        Self {
            client,
            params: AnswerShippingQueryParams {
                shipping_query_id: shipping_query_id.into(),
                ok,
                shipping_options: None,
                error_message: None,
            },
        }
    }
    /// Required when `ok = true` — the available shipping options to present to the user.
    pub fn shipping_options(mut self, opts: Vec<ShippingOption>) -> Self {
        self.params.shipping_options = Some(opts);
        self
    }
    /// Required when `ok = false` — a human-readable reason why delivery is not possible.
    pub fn error_message(mut self, msg: impl Into<String>) -> Self {
        self.params.error_message = Some(msg.into());
        self
    }
}

impl_into_future!(AnswerShippingQuery, bool, "answerShippingQuery");

// ─── answerPreCheckoutQuery ───────────────────────────────────────────────────

#[derive(Serialize)]
struct AnswerPreCheckoutQueryParams {
    pre_checkout_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}

/// Builder for the [`answerPreCheckoutQuery`](https://core.telegram.org/bots/api#answerprecheckoutquery) method.
///
/// Confirm or reject a pre-checkout query. Must be called within **10 seconds**
/// of receiving the query — no exceptions.
pub struct AnswerPreCheckoutQuery {
    client: BotClient,
    params: AnswerPreCheckoutQueryParams,
}

impl AnswerPreCheckoutQuery {
    pub(crate) fn new(
        client: BotClient,
        pre_checkout_query_id: impl Into<String>,
        ok: bool,
    ) -> Self {
        Self {
            client,
            params: AnswerPreCheckoutQueryParams {
                pre_checkout_query_id: pre_checkout_query_id.into(),
                ok,
                error_message: None,
            },
        }
    }
    /// Required when `ok = false` — a human-readable reason for the failure
    /// displayed to the user (e.g. `"Sorry, the item is out of stock"`).
    pub fn error_message(mut self, msg: impl Into<String>) -> Self {
        self.params.error_message = Some(msg.into());
        self
    }
}

impl_into_future!(AnswerPreCheckoutQuery, bool, "answerPreCheckoutQuery");

// ─── refundStarPayment ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RefundStarPaymentParams {
    user_id: i64,
    telegram_payment_charge_id: String,
}

/// Builder for the [`refundStarPayment`](https://core.telegram.org/bots/api#refundstarpayment) method.
///
/// Refunds a successful Telegram Stars payment to the user.
pub struct RefundStarPayment {
    client: BotClient,
    params: RefundStarPaymentParams,
}

impl RefundStarPayment {
    pub(crate) fn new(
        client: BotClient,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: RefundStarPaymentParams {
                user_id,
                telegram_payment_charge_id: telegram_payment_charge_id.into(),
            },
        }
    }
}

impl_into_future!(RefundStarPayment, bool, "refundStarPayment");

// ─── editUserStarSubscription ─────────────────────────────────────────────────

#[derive(Serialize)]
struct EditUserStarSubscriptionParams {
    user_id: i64,
    telegram_payment_charge_id: String,
    is_canceled: bool,
}

/// Builder for the [`editUserStarSubscription`](https://core.telegram.org/bots/api#edituserstarsubscription) method.
///
/// Cancels or re-enables a Telegram Stars subscription.
/// Pass `is_canceled = true` to cancel; `false` to re-enable a previously
/// cancelled subscription. The subscription must be active until the end of
/// the current period to be cancelled.
pub struct EditUserStarSubscription {
    client: BotClient,
    params: EditUserStarSubscriptionParams,
}

impl EditUserStarSubscription {
    pub(crate) fn new(
        client: BotClient,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> Self {
        Self {
            client,
            params: EditUserStarSubscriptionParams {
                user_id,
                telegram_payment_charge_id: telegram_payment_charge_id.into(),
                is_canceled,
            },
        }
    }
}

impl_into_future!(EditUserStarSubscription, bool, "editUserStarSubscription");

// ─── getMyStarBalance ─────────────────────────────────────────────────────────

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

// ─── getStarTransactions ──────────────────────────────────────────────────────

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
    /// Limits the number of transactions returned (1–100, default 100).
    pub fn limit(mut self, v: u32) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl_into_future!(GetStarTransactions, StarTransactions, "getStarTransactions");
