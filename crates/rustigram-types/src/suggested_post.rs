use serde::{Deserialize, Serialize};

/// The price of a suggested post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostPrice {
    /// Currency in which the post will be paid.
    ///
    /// Currently `"XTR"` for Telegram Stars or `"TON"` for toncoins.
    pub currency: String,
    /// Amount in the smallest units of the currency.
    ///
    /// Stars must be between 5–100,000; nanotoncoins between 10,000,000–10,000,000,000,000.
    pub amount: i64,
}

/// Information about a suggested post pending review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post.
    ///
    /// One of `"pending"`, `"approved"`, or `"declined"`.
    pub state: String,
    /// Proposed price of the post; absent if the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,
    /// Proposed Unix timestamp when the post will be sent; absent if no date was specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

/// Parameters for a post being suggested by the bot.
///
/// Pass this in `suggested_post_parameters` on outgoing send methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostParameters {
    /// Proposed price for the post; absent if the post is unpaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,
    /// Proposed Unix send date; must be 300–2,678,400 seconds in the future if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

/// Service message: a suggested post was approved.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostApproved {
    /// Message containing the suggested post.
    ///
    /// Boxed to break the `Message → SuggestedPostApproved → Message` cycle.
    /// Will not contain `reply_to_message` even if the message itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::message::Message>>,
    /// Amount paid for the post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SuggestedPostPrice>,
    /// Unix timestamp when the post will be published.
    pub send_date: i64,
}

/// Service message: approval of a suggested post failed.
///
/// Currently only caused by insufficient user funds at the time of approval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostApprovalFailed {
    /// Message containing the suggested post whose approval failed.
    ///
    /// Boxed to break the recursive cycle. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::message::Message>>,
    /// Expected price of the post.
    pub price: SuggestedPostPrice,
}

/// Service message: a suggested post was declined.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post.
    ///
    /// Boxed to break the recursive cycle. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::message::Message>>,
    /// Comment with which the post was declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Service message: payment for a suggested post was received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostPaid {
    /// Message containing the suggested post.
    ///
    /// Boxed to break the recursive cycle. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::message::Message>>,
    /// Currency in which the payment was made (`"XTR"` or `"TON"`).
    pub currency: String,
    /// Amount received in nanotoncoins; present only for TON payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Amount of Telegram Stars received; present only for Star payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_amount: Option<crate::payments::StarAmount>,
}

/// Service message: payment for a suggested post was refunded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedPostRefunded {
    /// Message containing the suggested post.
    ///
    /// Boxed to break the recursive cycle. Will not contain `reply_to_message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_message: Option<Box<crate::message::Message>>,
    /// Reason for the refund.
    ///
    /// One of `"post_deleted"` or `"payment_refunded"`.
    pub reason: String,
}
