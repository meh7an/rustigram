use crate::file::PhotoSize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A Telegram sticker.
pub struct Sticker {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Sticker type.
    #[serde(rename = "type")]
    pub kind: StickerType,
    /// Sticker width in pixels.
    pub width: u32,
    /// Sticker height in pixels.
    pub height: u32,
    /// `true` if the sticker is animated (TGS format).
    pub is_animated: bool,
    /// `true` if the sticker is a video sticker (WEBM format).
    pub is_video: bool,
    /// Sticker thumbnail.
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
    /// Name of the sticker set this sticker belongs to.
    pub set_name: Option<String>,
    /// Premium animation for the sticker.
    pub premium_animation: Option<crate::file::File>,
    /// Mask position for mask stickers.
    pub mask_position: Option<MaskPosition>,
    /// Custom emoji identifier for custom emoji stickers.
    pub custom_emoji_id: Option<String>,
    /// `true` if the sticker must be repainted to a text color in messages.
    pub needs_repainting: Option<bool>,
    /// File size in bytes.
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The type of sticker.
pub enum StickerType {
    /// A regular image sticker.
    Regular,
    /// A mask sticker (placed on photos).
    Mask,
    /// A custom emoji sticker.
    CustomEmoji,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The position of a mask sticker on a face.
pub struct MaskPosition {
    /// The part of the face where the mask is placed.
    pub point: MaskPoint,
    /// Shift by X-axis measured in widths of the mask, scaled to the face size.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask, scaled to the face size.
    pub y_shift: f64,
    /// Mask scaling coefficient.
    pub scale: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The specific point on a face where the mask is placed.
pub enum MaskPoint {
    /// Placed on the forehead.
    Forehead,
    /// Placed over the eyes.
    Eyes,
    /// Placed over the mouth.
    Mouth,
    /// Placed on the chin.
    Chin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A sticker set (collection of stickers).
pub struct StickerSet {
    /// Sticker set name.
    pub name: String,
    /// Sticker set title.
    pub title: String,
    /// Type of stickers in the set.
    pub sticker_type: StickerType,
    /// List of stickers in the set.
    pub stickers: Vec<Sticker>,
    /// Sticker set thumbnail.
    pub thumbnail: Option<PhotoSize>,
}

/// Input sticker for creating or extending sticker sets.
///
/// `sticker` holds a `file_id`, HTTP URL, or `attach://<name>` reference.
/// Raw bytes are handled at the API layer via multipart — this type is
/// serialisable so it can be embedded in JSON request bodies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSticker {
    /// File identifier, URL, or `attach://<n>` reference.
    pub sticker: String,
    /// Format of the sticker.
    pub format: StickerFormat,
    /// List of 1–20 emoji associated with the sticker.
    pub emoji_list: Vec<String>,
    /// Mask position for mask stickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// List of 0–20 search keywords for the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl InputSticker {
    /// Creates an `InputSticker` from a `file_id` or URL.
    pub fn new(
        sticker: impl Into<String>,
        format: StickerFormat,
        emoji_list: Vec<impl Into<String>>,
    ) -> Self {
        Self {
            sticker: sticker.into(),
            format,
            emoji_list: emoji_list.into_iter().map(Into::into).collect(),
            mask_position: None,
            keywords: None,
        }
    }
    /// Sets the mask position for this sticker.
    pub fn mask_position(mut self, mp: MaskPosition) -> Self {
        self.mask_position = Some(mp);
        self
    }
    /// Sets the search keywords for this sticker.
    pub fn keywords(mut self, kw: Vec<impl Into<String>>) -> Self {
        self.keywords = Some(kw.into_iter().map(Into::into).collect());
        self
    }
}

/// The encoding format of a sticker file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StickerFormat {
    /// A static WEBP sticker.
    Static,
    /// An animated TGS sticker.
    Animated,
    /// A video WEBM sticker.
    Video,
}
