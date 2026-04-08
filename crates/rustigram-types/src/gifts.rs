use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::sticker::Sticker;

// ─── Regular gift ─────────────────────────────────────────────────────────────

/// Background of a regular gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftBackground {
    /// Center color of the background in RGB format.
    pub center_color: u32,
    /// Edge color of the background in RGB format.
    pub edge_color: u32,
    /// Text color of the background in RGB format.
    pub text_color: u32,
}

/// A Telegram gift that can be sent to users.
///
/// Retrieve available gifts with [`getAvailableGifts`](https://core.telegram.org/bots/api#getavailablegifts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gift {
    /// Unique gift identifier.
    pub id: String,
    /// The sticker that represents this gift.
    pub sticker: Sticker,
    /// Number of Telegram Stars that must be paid to send this gift.
    pub star_count: u32,
    /// Number of Telegram Stars required to upgrade this gift to a unique gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_star_count: Option<u32>,
    /// `true` if the gift can only be purchased by Telegram Premium subscribers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// `true` if the gift can be used to customize a user's appearance after upgrading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_colors: Option<bool>,
    /// Total number of gifts of this type that can be sent; absent if unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u32>,
    /// Number of remaining gifts of this type available to all users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<u32>,
    /// Total number of gifts of this type the bot can send; for limited gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_total_count: Option<u32>,
    /// Number of remaining gifts of this type the bot can send; for limited gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_remaining_count: Option<u32>,
    /// Background of the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<GiftBackground>,
    /// Total number of unique gift variants obtainable by upgrading this gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_gift_variant_count: Option<u32>,
    /// Information about the chat that published the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Chat>,
}

/// Collection of available gifts returned by `getAvailableGifts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gifts {
    /// The list of available gifts.
    pub gifts: Vec<Gift>,
}

// ─── Unique gift ──────────────────────────────────────────────────────────────

/// Model of a unique gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueGiftModel {
    /// Name of the model.
    pub name: String,
    /// The sticker that represents this model.
    pub sticker: Sticker,
    /// Number of unique gifts receiving this model per 1000 upgrades. Always `0` for crafted gifts.
    pub rarity_per_mille: u32,
    /// Rarity tier for crafted models: `"uncommon"`, `"rare"`, `"epic"`, or `"legendary"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>,
}

/// Symbol shown on the pattern of a unique gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol.
    pub name: String,
    /// The sticker that represents this symbol.
    pub sticker: Sticker,
    /// Number of unique gifts receiving this symbol per 1000 upgrades.
    pub rarity_per_mille: u32,
}

/// Colors of the backdrop of a unique gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueGiftBackdropColors {
    /// Center color of the backdrop in RGB format.
    pub center_color: u32,
    /// Edge color of the backdrop in RGB format.
    pub edge_color: u32,
    /// Color applied to the symbol in RGB format.
    pub symbol_color: u32,
    /// Text color of the backdrop in RGB format.
    pub text_color: u32,
}

/// Backdrop of a unique gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop.
    pub name: String,
    /// Colors of the backdrop.
    pub colors: UniqueGiftBackdropColors,
    /// Number of unique gifts receiving this backdrop per 1000 upgrades.
    pub rarity_per_mille: u32,
}

/// Color scheme for a user's name, replies, and link previews based on a unique gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the gift's model.
    pub model_custom_emoji_id: String,
    /// Custom emoji identifier of the gift's symbol.
    pub symbol_custom_emoji_id: String,
    /// Main color in light themes in RGB format.
    pub light_theme_main_color: u32,
    /// Up to 3 additional colors in light themes in RGB format.
    pub light_theme_other_colors: Vec<u32>,
    /// Main color in dark themes in RGB format.
    pub dark_theme_main_color: u32,
    /// Up to 3 additional colors in dark themes in RGB format.
    pub dark_theme_other_colors: Vec<u32>,
}

/// A unique gift upgraded from a regular gift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which this was upgraded.
    pub gift_id: String,
    /// Human-readable name of the original regular gift.
    pub base_name: String,
    /// Unique name usable in `https://t.me/nft/...` links and story areas.
    pub name: String,
    /// Unique number among gifts upgraded from the same regular gift.
    pub number: u32,
    /// Model of the gift.
    pub model: UniqueGiftModel,
    /// Symbol of the gift.
    pub symbol: UniqueGiftSymbol,
    /// Backdrop of the gift.
    pub backdrop: UniqueGiftBackdrop,
    /// `true` if the original regular gift was exclusively purchaseable by Premium subscribers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// `true` if the gift was used to craft another gift and is no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_burned: Option<bool>,
    /// `true` if the gift is from the TON blockchain and cannot be resold or transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_blockchain: Option<bool>,
    /// Color scheme for the owner's chat name, replies, and link previews.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<UniqueGiftColors>,
    /// Information about the chat that published the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Chat>,
}
