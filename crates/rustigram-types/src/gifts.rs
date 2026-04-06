use crate::sticker::Sticker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A Telegram gift that can be sent to users.
///
/// Retrieve available gifts with [`getAvailableGifts`](https://core.telegram.org/bots/api#getavailablegifts).
pub struct Gift {
    /// Unique gift identifier.
    pub id: String,
    /// The sticker that represents this gift.
    pub sticker: Sticker,
    /// Number of Telegram Stars that must be paid to send this gift.
    pub star_count: u32,
    /// Number of Telegram Stars required to upgrade this gift to a unique gift.
    pub upgrade_star_count: Option<u32>,
    /// Total number of gifts of this type that can be sent; `None` if unlimited.
    pub total_count: Option<u32>,
    /// Number of remaining gifts of this type that can be sent.
    pub remaining_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Collection of available gifts returned by `getAvailableGifts`.
pub struct Gifts {
    /// The list of available gifts.
    pub gifts: Vec<Gift>,
}
