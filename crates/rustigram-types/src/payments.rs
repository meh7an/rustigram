use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A portion of a price — label and amount in the smallest currency unit.
///
/// For Telegram Stars (`XTR`) the amount is in whole Stars.
/// For fiat currencies (e.g. `USD`) the amount is in cents.
pub struct LabeledPrice {
    /// Portion label shown to the user.
    pub label: String,
    /// Price in the smallest currency units.
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An invoice for a payment inside a message.
pub struct Invoice {
    /// Product name.
    pub title: String,
    /// Product description.
    pub description: String,
    /// Unique bot deep-linking parameter for the invoice.
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Total price in the smallest currency unit.
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A shipping address provided by the user during checkout.
pub struct ShippingAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// State, if applicable.
    pub state: String,
    /// City name.
    pub city: String,
    /// First line of the street address.
    pub street_line1: String,
    /// Second line of the street address.
    pub street_line2: String,
    /// Post code.
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Information about an order collected from the user during checkout.
pub struct OrderInfo {
    /// User's name.
    pub name: Option<String>,
    /// User's phone number.
    pub phone_number: Option<String>,
    /// User's email address.
    pub email: Option<String>,
    /// User's shipping address.
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One shipping option offered to the user during payment.
pub struct ShippingOption {
    /// Shipping option identifier.
    pub id: String,
    /// Shipping option title.
    pub title: String,
    /// List of price portions.
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Confirmation that a payment was completed successfully.
///
/// Delivered inside a [`Message`](crate::message::Message) after the buyer
/// confirms checkout.
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Total price in the smallest currency unit.
    pub total_amount: i64,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user.
    pub order_info: Option<OrderInfo>,
    /// Telegram payment charge identifier.
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier.
    pub provider_payment_charge_id: String,
    /// Expiration date of the subscription, as a Unix timestamp.
    pub subscription_expiration_date: Option<i64>,
    /// `true` if the payment is recurring.
    pub is_recurring: Option<bool>,
    /// `true` if this is the first payment for a subscription.
    pub is_first_recurring: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Information about a refunded payment.
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or `"XTR"` for Stars.
    pub currency: String,
    /// Total refunded price in the smallest currency unit.
    pub total_amount: i64,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Telegram payment charge identifier.
    pub telegram_payment_charge_id: String,
    /// Provider payment refund identifier.
    pub provider_payment_charge_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An incoming shipping query from a user.
///
/// Delivered when the invoice has `is_flexible = true`. Respond with
/// [`answerShippingQuery`](https://core.telegram.org/bots/api#answershippingquery).
pub struct ShippingQuery {
    /// Unique query identifier.
    pub id: String,
    /// The user who sent the query.
    pub from: User,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// User-specified shipping address.
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An incoming pre-checkout query.
///
/// Sent immediately before the payment confirmation screen. You must respond
/// with [`answerPreCheckoutQuery`](https://core.telegram.org/bots/api#answerprecheckoutquery)
/// within **10 seconds**.
pub struct PreCheckoutQuery {
    /// Unique query identifier.
    pub id: String,
    /// The user who sent the query.
    pub from: User,
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Total price in the smallest currency unit.
    pub total_amount: i64,
    /// Bot-specified invoice payload.
    pub invoice_payload: String,
    /// Identifier of the shipping option chosen by the user.
    pub shipping_option_id: Option<String>,
    /// Order info provided by the user.
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A list of Telegram Star transactions.
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A single Telegram Star transaction.
pub struct StarTransaction {
    /// Unique transaction identifier.
    pub id: String,
    /// Number of Telegram Stars transferred.
    pub amount: u64,
    /// Number of 1/1000000000 shares of Telegram Stars transferred.
    pub nanostar_amount: Option<u32>,
    /// Date the transaction was created, as a Unix timestamp.
    pub date: i64,
    /// Source of the transaction (for incoming transactions).
    pub source: Option<serde_json::Value>,
    /// Receiver of the transaction (for outgoing transactions).
    pub receiver: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An amount of Telegram Stars.
///
/// `amount` is the integer Star count. `nanostar_amount` is a fractional
/// component in nanostar units (1 Star = 1,000,000,000 nanostars), present
/// only in certain transaction contexts.
pub struct StarAmount {
    /// Integer Star amount.
    pub amount: u64,
    /// Fractional amount in nanostar units (1 Star = 1,000,000,000 nanostars).
    pub nanostar_amount: Option<u32>,
}
