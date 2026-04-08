use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::gifts::Gifts;
use rustigram_types::message::MessageEntity;
use rustigram_types::payments::OwnedGifts;
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

// ─── getAvailableGifts ────────────────────────────────────────────────────────

/// Builder for the [`getAvailableGifts`](https://core.telegram.org/bots/api#getavailablegifts) method.
///
/// Returns the list of gifts that can be sent by the bot. Requires no parameters.
pub struct GetAvailableGifts {
    client: BotClient,
}

impl GetAvailableGifts {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}

impl IntoFuture for GetAvailableGifts {
    type Output = Result<Gifts>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getAvailableGifts", &serde_json::json!({}))
                .await
        })
    }
}

// ─── sendGift ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SendGiftParams {
    gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_for_upgrade: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_parse_mode: Option<rustigram_types::message::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_entities: Option<Vec<MessageEntity>>,
}

/// Builder for the [`sendGift`](https://core.telegram.org/bots/api#sendgift) method.
///
/// Sends a gift to a user or channel chat. Either `user_id` or `chat_id` must be
/// set — use the corresponding builder method.
pub struct SendGift {
    client: BotClient,
    params: SendGiftParams,
}

impl SendGift {
    pub(crate) fn new(client: BotClient, gift_id: impl Into<String>) -> Self {
        Self {
            client,
            params: SendGiftParams {
                gift_id: gift_id.into(),
                user_id: None,
                chat_id: None,
                pay_for_upgrade: None,
                text: None,
                text_parse_mode: None,
                text_entities: None,
            },
        }
    }
    /// Sends the gift to a user. Required if `chat_id` is not set.
    pub fn user_id(mut self, id: i64) -> Self {
        self.params.user_id = Some(id);
        self
    }
    /// Sends the gift to a channel chat. Required if `user_id` is not set.
    /// Limited gifts cannot be sent to channel chats.
    pub fn chat_id(mut self, id: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(id.into());
        self
    }
    /// Pass `true` to pay for the upgrade from the bot's balance, making the upgrade free for the receiver.
    pub fn pay_for_upgrade(mut self, v: bool) -> Self {
        self.params.pay_for_upgrade = Some(v);
        self
    }
    /// Optional text message to accompany the gift (0–128 characters).
    pub fn text(mut self, t: impl Into<String>) -> Self {
        self.params.text = Some(t.into());
        self
    }
    /// Parse mode for entities in the gift text.
    pub fn text_parse_mode(mut self, m: rustigram_types::message::ParseMode) -> Self {
        self.params.text_parse_mode = Some(m);
        self
    }
    /// Special entities in the gift text; alternative to `text_parse_mode`.
    pub fn text_entities(mut self, e: Vec<MessageEntity>) -> Self {
        self.params.text_entities = Some(e);
        self
    }
}

impl_into_future!(SendGift, bool, "sendGift");

// ─── giftPremiumSubscription ──────────────────────────────────────────────────

#[derive(Serialize)]
struct GiftPremiumSubscriptionParams {
    user_id: i64,
    month_count: u32,
    star_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_parse_mode: Option<rustigram_types::message::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_entities: Option<Vec<MessageEntity>>,
}

/// Builder for the [`giftPremiumSubscription`](https://core.telegram.org/bots/api#giftpremiumsubscription) method.
///
/// Gifts a Telegram Premium subscription to a user.
/// `month_count` must be `3`, `6`, or `12`.
/// `star_count` must be `1000`, `1500`, or `2500` respectively.
pub struct GiftPremiumSubscription {
    client: BotClient,
    params: GiftPremiumSubscriptionParams,
}

impl GiftPremiumSubscription {
    pub(crate) fn new(client: BotClient, user_id: i64, month_count: u32, star_count: u32) -> Self {
        Self {
            client,
            params: GiftPremiumSubscriptionParams {
                user_id,
                month_count,
                star_count,
                text: None,
                text_parse_mode: None,
                text_entities: None,
            },
        }
    }
    /// Optional text to accompany the gift (0–128 characters).
    pub fn text(mut self, t: impl Into<String>) -> Self {
        self.params.text = Some(t.into());
        self
    }
    /// Parse mode for entities in the gift text.
    pub fn text_parse_mode(mut self, m: rustigram_types::message::ParseMode) -> Self {
        self.params.text_parse_mode = Some(m);
        self
    }
    /// Special entities in the gift text; alternative to `text_parse_mode`.
    pub fn text_entities(mut self, e: Vec<MessageEntity>) -> Self {
        self.params.text_entities = Some(e);
        self
    }
}

impl_into_future!(GiftPremiumSubscription, bool, "giftPremiumSubscription");

// ─── getBusinessAccountGifts ──────────────────────────────────────────────────

#[derive(Serialize)]
struct GetBusinessAccountGiftsParams {
    business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unsaved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_saved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unlimited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_limited_upgradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_limited_non_upgradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_from_blockchain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by_price: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

/// Builder for the [`getBusinessAccountGifts`](https://core.telegram.org/bots/api#getbusinessaccountgifts) method.
///
/// Returns the gifts received by a managed business account.
/// Requires the `can_view_gifts_and_stars` business bot right.
pub struct GetBusinessAccountGifts {
    client: BotClient,
    params: GetBusinessAccountGiftsParams,
}

impl GetBusinessAccountGifts {
    pub(crate) fn new(client: BotClient, business_connection_id: impl Into<String>) -> Self {
        Self {
            client,
            params: GetBusinessAccountGiftsParams {
                business_connection_id: business_connection_id.into(),
                exclude_unsaved: None,
                exclude_saved: None,
                exclude_unlimited: None,
                exclude_limited_upgradable: None,
                exclude_limited_non_upgradable: None,
                exclude_unique: None,
                exclude_from_blockchain: None,
                sort_by_price: None,
                offset: None,
                limit: None,
            },
        }
    }
    /// Excludes gifts not saved to the account's profile page.
    pub fn exclude_unsaved(mut self, v: bool) -> Self {
        self.params.exclude_unsaved = Some(v);
        self
    }
    /// Excludes gifts saved to the account's profile page.
    pub fn exclude_saved(mut self, v: bool) -> Self {
        self.params.exclude_saved = Some(v);
        self
    }
    /// Excludes unlimited regular gifts.
    pub fn exclude_unlimited(mut self, v: bool) -> Self {
        self.params.exclude_unlimited = Some(v);
        self
    }
    /// Excludes limited gifts that can be upgraded to unique.
    pub fn exclude_limited_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_upgradable = Some(v);
        self
    }
    /// Excludes limited gifts that cannot be upgraded to unique.
    pub fn exclude_limited_non_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_non_upgradable = Some(v);
        self
    }
    /// Excludes unique gifts.
    pub fn exclude_unique(mut self, v: bool) -> Self {
        self.params.exclude_unique = Some(v);
        self
    }
    /// Excludes gifts assigned from the TON blockchain.
    pub fn exclude_from_blockchain(mut self, v: bool) -> Self {
        self.params.exclude_from_blockchain = Some(v);
        self
    }
    /// Sorts results by price instead of send date.
    pub fn sort_by_price(mut self, v: bool) -> Self {
        self.params.sort_by_price = Some(v);
        self
    }
    /// Pagination offset from the previous response.
    pub fn offset(mut self, o: impl Into<String>) -> Self {
        self.params.offset = Some(o.into());
        self
    }
    /// Maximum number of gifts to return (1–100, default 100).
    pub fn limit(mut self, v: u32) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl_into_future!(
    GetBusinessAccountGifts,
    OwnedGifts,
    "getBusinessAccountGifts"
);

// ─── getUserGifts ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GetUserGiftsParams {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unlimited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_limited_upgradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_limited_non_upgradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_from_blockchain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by_price: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

/// Builder for the [`getUserGifts`](https://core.telegram.org/bots/api#getusergifts) method.
///
/// Returns the gifts owned and hosted by a user.
pub struct GetUserGifts {
    client: BotClient,
    params: GetUserGiftsParams,
}

impl GetUserGifts {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: GetUserGiftsParams {
                user_id,
                exclude_unlimited: None,
                exclude_limited_upgradable: None,
                exclude_limited_non_upgradable: None,
                exclude_unique: None,
                exclude_from_blockchain: None,
                sort_by_price: None,
                offset: None,
                limit: None,
            },
        }
    }
    /// Excludes unlimited regular gifts.
    pub fn exclude_unlimited(mut self, v: bool) -> Self {
        self.params.exclude_unlimited = Some(v);
        self
    }
    /// Excludes limited gifts that can be upgraded to unique.
    pub fn exclude_limited_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_upgradable = Some(v);
        self
    }
    /// Excludes limited gifts that cannot be upgraded to unique.
    pub fn exclude_limited_non_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_non_upgradable = Some(v);
        self
    }
    /// Excludes unique gifts.
    pub fn exclude_unique(mut self, v: bool) -> Self {
        self.params.exclude_unique = Some(v);
        self
    }
    /// Excludes gifts assigned from the TON blockchain.
    pub fn exclude_from_blockchain(mut self, v: bool) -> Self {
        self.params.exclude_from_blockchain = Some(v);
        self
    }
    /// Sorts results by price instead of send date.
    pub fn sort_by_price(mut self, v: bool) -> Self {
        self.params.sort_by_price = Some(v);
        self
    }
    /// Pagination offset from the previous response.
    pub fn offset(mut self, o: impl Into<String>) -> Self {
        self.params.offset = Some(o.into());
        self
    }
    /// Maximum number of gifts to return (1–100, default 100).
    pub fn limit(mut self, v: u32) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl_into_future!(GetUserGifts, OwnedGifts, "getUserGifts");

// ─── getChatGifts ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GetChatGiftsParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unsaved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_saved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unlimited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_limited_upgradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_limited_non_upgradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_from_blockchain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by_price: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

/// Builder for the [`getChatGifts`](https://core.telegram.org/bots/api#getchatgifts) method.
///
/// Returns the gifts owned by a channel chat.
pub struct GetChatGifts {
    client: BotClient,
    params: GetChatGiftsParams,
}

impl GetChatGifts {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: GetChatGiftsParams {
                chat_id: chat_id.into(),
                exclude_unsaved: None,
                exclude_saved: None,
                exclude_unlimited: None,
                exclude_limited_upgradable: None,
                exclude_limited_non_upgradable: None,
                exclude_unique: None,
                exclude_from_blockchain: None,
                sort_by_price: None,
                offset: None,
                limit: None,
            },
        }
    }
    /// Excludes gifts not saved to the chat's profile page.
    pub fn exclude_unsaved(mut self, v: bool) -> Self {
        self.params.exclude_unsaved = Some(v);
        self
    }
    /// Excludes gifts saved to the chat's profile page.
    pub fn exclude_saved(mut self, v: bool) -> Self {
        self.params.exclude_saved = Some(v);
        self
    }
    /// Excludes unlimited regular gifts.
    pub fn exclude_unlimited(mut self, v: bool) -> Self {
        self.params.exclude_unlimited = Some(v);
        self
    }
    /// Excludes limited gifts that can be upgraded to unique.
    pub fn exclude_limited_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_upgradable = Some(v);
        self
    }
    /// Excludes limited gifts that cannot be upgraded to unique.
    pub fn exclude_limited_non_upgradable(mut self, v: bool) -> Self {
        self.params.exclude_limited_non_upgradable = Some(v);
        self
    }
    /// Excludes unique gifts.
    pub fn exclude_unique(mut self, v: bool) -> Self {
        self.params.exclude_unique = Some(v);
        self
    }
    /// Excludes gifts assigned from the TON blockchain.
    pub fn exclude_from_blockchain(mut self, v: bool) -> Self {
        self.params.exclude_from_blockchain = Some(v);
        self
    }
    /// Sorts results by price instead of send date.
    pub fn sort_by_price(mut self, v: bool) -> Self {
        self.params.sort_by_price = Some(v);
        self
    }
    /// Pagination offset from the previous response.
    pub fn offset(mut self, o: impl Into<String>) -> Self {
        self.params.offset = Some(o.into());
        self
    }
    /// Maximum number of gifts to return (1–100, default 100).
    pub fn limit(mut self, v: u32) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl_into_future!(GetChatGifts, OwnedGifts, "getChatGifts");

// ─── convertGiftToStars ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct ConvertGiftToStarsParams {
    business_connection_id: String,
    owned_gift_id: String,
}

/// Builder for the [`convertGiftToStars`](https://core.telegram.org/bots/api#convertgifttostars) method.
///
/// Converts a regular gift owned by a business account to Telegram Stars.
/// Requires the `can_convert_gifts_to_stars` business bot right.
pub struct ConvertGiftToStars {
    client: BotClient,
    params: ConvertGiftToStarsParams,
}

impl ConvertGiftToStars {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: ConvertGiftToStarsParams {
                business_connection_id: business_connection_id.into(),
                owned_gift_id: owned_gift_id.into(),
            },
        }
    }
}

impl_into_future!(ConvertGiftToStars, bool, "convertGiftToStars");

// ─── upgradeGift ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct UpgradeGiftParams {
    business_connection_id: String,
    owned_gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_original_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    star_count: Option<u32>,
}

/// Builder for the [`upgradeGift`](https://core.telegram.org/bots/api#upgradegift) method.
///
/// Upgrades a regular gift to a unique gift.
/// Requires the `can_transfer_and_upgrade_gifts` business bot right.
/// Also requires `can_transfer_stars` if the upgrade is paid.
pub struct UpgradeGift {
    client: BotClient,
    params: UpgradeGiftParams,
}

impl UpgradeGift {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: UpgradeGiftParams {
                business_connection_id: business_connection_id.into(),
                owned_gift_id: owned_gift_id.into(),
                keep_original_details: None,
                star_count: None,
            },
        }
    }
    /// Pass `true` to preserve the original gift text, sender, and receiver in the upgraded gift.
    pub fn keep_original_details(mut self, v: bool) -> Self {
        self.params.keep_original_details = Some(v);
        self
    }
    /// Stars to pay for the upgrade from the business account balance.
    /// Pass `0` if `gift.prepaid_upgrade_star_count > 0`.
    pub fn star_count(mut self, v: u32) -> Self {
        self.params.star_count = Some(v);
        self
    }
}

impl_into_future!(UpgradeGift, bool, "upgradeGift");

// ─── transferGift ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct TransferGiftParams {
    business_connection_id: String,
    owned_gift_id: String,
    new_owner_chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    star_count: Option<u32>,
}

/// Builder for the [`transferGift`](https://core.telegram.org/bots/api#transfergift) method.
///
/// Transfers a unique gift to another user.
/// Requires the `can_transfer_and_upgrade_gifts` business bot right.
/// Also requires `can_transfer_stars` if the transfer is paid.
pub struct TransferGift {
    client: BotClient,
    params: TransferGiftParams,
}

impl TransferGift {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        new_owner_chat_id: i64,
    ) -> Self {
        Self {
            client,
            params: TransferGiftParams {
                business_connection_id: business_connection_id.into(),
                owned_gift_id: owned_gift_id.into(),
                new_owner_chat_id,
                star_count: None,
            },
        }
    }
    /// Stars to pay for the transfer from the business account balance.
    /// Required if the transfer is not free.
    pub fn star_count(mut self, v: u32) -> Self {
        self.params.star_count = Some(v);
        self
    }
}

impl_into_future!(TransferGift, bool, "transferGift");
