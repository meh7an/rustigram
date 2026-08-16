use serde::{Deserialize, Serialize};

use crate::file::Document;

/// How a chat background is filled with colour.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundFill {
    /// A single colour.
    Solid(BackgroundFillSolid),
    /// A two-colour gradient.
    Gradient(BackgroundFillGradient),
    /// A freeform gradient rotating between three or four colours.
    FreeformGradient(BackgroundFillFreeformGradient),
}

/// A background filled with a single colour.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundFillSolid {
    /// The fill colour in RGB24 format.
    pub color: u32,
}

/// A background filled with a two-colour gradient.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundFillGradient {
    /// Top colour of the gradient in RGB24 format.
    pub top_color: u32,
    /// Bottom colour of the gradient in RGB24 format.
    pub bottom_color: u32,
    /// Clockwise rotation angle of the background fill, in degrees (0–359).
    pub rotation_angle: u16,
}

/// A background filled with a freeform gradient that rotates after every message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundFillFreeformGradient {
    /// Three or four base colours used to generate the gradient, in RGB24 format.
    pub colors: Vec<u32>,
}

/// The type of a chat background.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundType {
    /// Automatically filled based on colours.
    Fill(BackgroundTypeFill),
    /// A wallpaper image in JPEG format.
    Wallpaper(BackgroundTypeWallpaper),
    /// A PNG or TGV pattern filled with a colour.
    Pattern(BackgroundTypePattern),
    /// One of the default chat themes.
    ChatTheme(BackgroundTypeChatTheme),
}

/// A background automatically filled based on colours.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundTypeFill {
    /// The background fill.
    pub fill: BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage (0–100).
    pub dark_theme_dimming: u8,
}

/// A background that is a wallpaper image in JPEG format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundTypeWallpaper {
    /// Document with the wallpaper.
    pub document: Document,
    /// Dimming of the background in dark themes, as a percentage (0–100).
    pub dark_theme_dimming: u8,
    /// `true` if the wallpaper is downscaled to fit in a 450×450 square and
    /// then box-blurred with a radius of 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,
    /// `true` if the background moves slightly when the device is tilted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}

/// A background that is a PNG or TGV pattern filled with a colour.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundTypePattern {
    /// Document with the pattern.
    pub document: Document,
    /// The background fill used to combine with the pattern.
    pub fill: BackgroundFill,
    /// Intensity of the pattern when it is shown above the filled background,
    /// as a percentage (0–100).
    pub intensity: u8,
    /// `true` if the background fill must be applied only to the pattern itself.
    /// All other pixels are black in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inverted: Option<bool>,
    /// `true` if the background moves slightly when the device is tilted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}

/// A background taken from one of the default chat themes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackgroundTypeChatTheme {
    /// Name of the chat theme, which is usually an emoji.
    pub theme_name: String,
}

/// A chat background.
///
/// Sealed without [`Default`]: its required `type` field is a
/// [`BackgroundType`], and each variant describes a genuinely different kind of
/// background, so nominating one as the default would invent a value Telegram
/// never sends.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChatBackground {
    /// Type of the background.
    #[serde(rename = "type")]
    pub kind: BackgroundType,
}
