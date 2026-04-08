use crate::chat::Chat;
use crate::file::PhotoSize;
use crate::gifts::{Gift, UniqueGift};
use crate::message::MessageEntity;
use crate::user::User;
use serde::{Deserialize, Serialize};

/// A portion of a price — label and amount in the smallest currency unit.
///
/// For Telegram Stars (`XTR`) the amount is in whole Stars.
/// For fiat currencies (e.g. `USD`) the amount is in cents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label shown to the user.
    pub label: String,
    /// Price in the smallest currency units.
    pub amount: i64,
}

/// An invoice for a payment inside a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// A shipping address provided by the user during checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Information about an order collected from the user during checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// One shipping option offered to the user during payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier.
    pub id: String,
    /// Shipping option title.
    pub title: String,
    /// List of price portions.
    pub prices: Vec<LabeledPrice>,
}

/// Confirmation that a payment was completed successfully.
///
/// Delivered inside a [`Message`](crate::message::Message) after the buyer
/// confirms checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Information about a refunded payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// An incoming shipping query from a user.
///
/// Delivered when the invoice has `is_flexible = true`. Respond with
/// [`answerShippingQuery`](https://core.telegram.org/bots/api#answershippingquery).
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// An incoming pre-checkout query.
///
/// Sent immediately before the payment confirmation screen. You must respond
/// with [`answerPreCheckoutQuery`](https://core.telegram.org/bots/api#answerprecheckoutquery)
/// within **10 seconds**.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// An amount of Telegram Stars.
///
/// `amount` is the integer Star count. `nanostar_amount` is a fractional
/// component in nanostar units (1 Star = 1,000,000,000 nanostars), present
/// only in certain transaction contexts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarAmount {
    /// Integer Star amount.
    pub amount: u64,
    /// Fractional amount in nanostar units (1 Star = 1,000,000,000 nanostars).
    pub nanostar_amount: Option<u32>,
}

// ─── Revenue withdrawal ───────────────────────────────────────────────────────

/// The state of a revenue withdrawal operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
    /// The withdrawal is in progress.
    Pending,
    /// The withdrawal succeeded.
    Succeeded {
        /// Date the withdrawal was completed in Unix time.
        date: i64,
        /// HTTPS URL to view the transaction details.
        url: String,
    },
    /// The withdrawal failed and the transaction was refunded.
    Failed,
}

// Affiliate

/// Information about the affiliate that received a commission via this transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffiliateInfo {
    /// The bot or user that received the affiliate commission, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_user: Option<User>,
    /// The chat that received the affiliate commission, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_chat: Option<Chat>,
    /// Stars received per 1000 Stars received by the affiliate program sponsor.
    pub commission_per_mille: i64,
    /// Integer amount of Stars received from the transaction; can be negative for refunds.
    pub amount: i64,
    /// Fractional nanostar amount; from -999,999,999 to 999,999,999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i32>,
}

// Transaction partners

/// The source or recipient of a Star transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPartner {
    /// A transaction with a user.
    User(TransactionPartnerUser),
    /// A transaction with a chat.
    Chat(TransactionPartnerChat),
    /// The affiliate program that issued the commission.
    AffiliateProgram(TransactionPartnerAffiliateProgram),
    /// A withdrawal transaction with Fragment.
    Fragment(TransactionPartnerFragment),
    /// A withdrawal transaction to the Telegram Ads platform.
    TelegramAds,
    /// A transaction for paid broadcasting.
    TelegramApi(TransactionPartnerTelegramApi),
    /// A transaction with an unknown source or recipient.
    Other,
}

/// A transaction with a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPartnerUser {
    /// Type of the transaction.
    ///
    /// One of `"invoice_payment"`, `"paid_media_payment"`, `"gift_purchase"`,
    /// `"premium_purchase"`, or `"business_account_transfer"`.
    pub transaction_type: String,
    /// The user involved in the transaction.
    pub user: User,
    /// Affiliate commission information; available for invoice and paid media payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<AffiliateInfo>,
    /// Bot-specified invoice payload; available for `"invoice_payment"` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<String>,
    /// Duration of the paid subscription; available for `"invoice_payment"` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// Paid media bought by the user; available for `"paid_media_payment"` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<Vec<serde_json::Value>>,
    /// Bot-specified paid media payload; available for `"paid_media_payment"` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<String>,
    /// The gift sent to the user; available for `"gift_purchase"` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,
    /// Months of Premium gifted; available for `"premium_purchase"` only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_duration: Option<i64>,
}

/// A transaction with a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPartnerChat {
    /// The chat involved in the transaction.
    pub chat: Chat,
    /// The gift sent to the chat by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Gift>,
}

/// The affiliate program that issued the commission received via this transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// The bot that sponsored the affiliate program, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor_user: Option<User>,
    /// Stars received by the bot per 1000 Stars received by the program sponsor.
    pub commission_per_mille: i64,
}

/// A withdrawal transaction with Fragment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPartnerFragment {
    /// State of the transaction if it is outgoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_state: Option<RevenueWithdrawalState>,
}

/// A transaction for paid broadcasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPartnerTelegramApi {
    /// Number of successful requests that exceeded regular limits and were billed.
    pub request_count: i64,
}

// Star transactions

/// A list of Telegram Star transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransactions {
    /// The list of transactions.
    pub transactions: Vec<StarTransaction>,
}

/// A single Telegram Star transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransaction {
    /// Unique transaction identifier.
    pub id: String,
    /// Number of Telegram Stars transferred.
    pub amount: u64,
    /// Number of 1/1000000000 shares of Telegram Stars transferred.
    pub nanostar_amount: Option<u32>,
    /// Date the transaction was created, as a Unix timestamp.
    pub date: i64,
    /// Source of an incoming transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,
    /// Receiver of an outgoing transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}

// Gifts

/// Types of gifts that can be gifted to a user or chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedGiftTypes {
    /// `true` if unlimited regular gifts are accepted.
    pub unlimited_gifts: bool,
    /// `true` if limited regular gifts are accepted.
    pub limited_gifts: bool,
    /// `true` if unique gifts or gifts upgradable to unique for free are accepted.
    pub unique_gifts: bool,
    /// `true` if a Telegram Premium subscription is accepted.
    pub premium_subscription: bool,
    /// `true` if transfers of unique gifts from channels are accepted.
    pub gifts_from_channels: bool,
}

/// A regular gift owned by a user or chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedGiftRegular {
    /// Information about the regular gift.
    pub gift: Gift,
    /// Unique identifier of the gift for the bot; for business account gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    /// Sender of the gift if it is a known user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<User>,
    /// Date the gift was sent, as a Unix timestamp.
    pub send_date: i64,
    /// Text of the message added to the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Special entities in the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// `true` if only the gift receiver can see the sender and text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// `true` if the gift is displayed on the account's profile page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// `true` if the gift can be upgraded to a unique gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_upgraded: Option<bool>,
    /// `true` if the gift was refunded and is no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    /// Stars that can be claimed instead of the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_star_count: Option<i64>,
    /// Stars prepaid for the ability to upgrade the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_upgrade_star_count: Option<i64>,
    /// `true` if the upgrade was purchased after the gift was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upgrade_separate: Option<bool>,
    /// Unique number reserved for this gift when upgraded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_number: Option<i64>,
}

/// A unique gift owned by a user or chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedGiftUnique {
    /// Information about the unique gift.
    pub gift: UniqueGift,
    /// Unique identifier of the gift for the bot; for business account gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,
    /// Sender of the gift if it is a known user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user: Option<User>,
    /// Date the gift was sent, as a Unix timestamp.
    pub send_date: i64,
    /// `true` if the gift is displayed on the account's profile page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_saved: Option<bool>,
    /// `true` if the gift can be transferred to another owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_be_transferred: Option<bool>,
    /// Stars required to transfer the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,
    /// Unix timestamp when the gift can next be transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,
}

/// A gift received and owned by a user or chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OwnedGift {
    /// A regular owned gift.
    Regular(OwnedGiftRegular),
    /// A unique owned gift.
    Unique(OwnedGiftUnique),
}

/// A paginated list of gifts owned by a user or chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedGifts {
    /// Total number of gifts owned by the user or chat.
    pub total_count: i64,
    /// The list of gifts.
    pub gifts: Vec<OwnedGift>,
    /// Offset for the next request; absent if there are no more results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
}

// Paid media

/// A photo available as paid media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidMediaPhoto {
    /// Available sizes of the photo.
    pub photo: Vec<PhotoSize>,
}

/// A preview shown before a user purchases paid media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidMediaPreview {
    /// Media width as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Media height as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Duration of the media in seconds as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
