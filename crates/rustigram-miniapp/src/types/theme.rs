//! Telegram Mini App theme types.
//!
//! Typed representations of the theme parameters and color scheme Telegram
//! exposes through `window.Telegram.WebApp.themeParams` and
//! `window.Telegram.WebApp.colorScheme`. All color values are `#RRGGBB` hex
//! strings. Fields are optional because older clients omit tokens they do not
//! support — never treat any field as guaranteed.
//!
//! @see <https://core.telegram.org/bots/webapps#themeparams>

use serde::{Deserialize, Serialize};

/// Telegram UI color scheme reported by the client.
///
/// @see <https://core.telegram.org/bots/webapps#colorscheme>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub enum ColorScheme {
    /// Light theme.
    Light,
    /// Dark theme.
    Dark,
}

/// Contains the user's current theme settings used in the Telegram app.
/// Mini Apps can adjust the appearance of the interface to match the
/// Telegram user's app in real time.
///
/// All fields are optional — older clients omit fields they do not support.
/// Never treat any field as guaranteed to be present.
///
/// Color values are always in `#RRGGBB` hex format when present.
///
/// @see <https://core.telegram.org/bots/webapps#themeparams>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct ThemeParams {
    /// Background color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-bg-color)`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub bg_color: Option<String>,

    /// Main text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-text-color)`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub text_color: Option<String>,

    /// Hint text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-hint-color)`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub hint_color: Option<String>,

    /// Link color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-link-color)`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub link_color: Option<String>,

    /// Button color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-button-color)`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub button_color: Option<String>,

    /// Button text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-button-text-color)`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub button_text_color: Option<String>,

    /// Secondary background color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-secondary-bg-color)`
    ///
    /// @since Bot API 6.1
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub secondary_bg_color: Option<String>,

    /// Header background color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-header-bg-color)`
    ///
    /// @since Bot API 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub header_bg_color: Option<String>,

    /// Bottom bar background color in the `#RRGGBB` format. Also applied to
    /// the navigation bar on Android.
    /// CSS: `var(--tg-theme-bottom-bar-bg-color)`
    ///
    /// @since Bot API 7.10
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub bottom_bar_bg_color: Option<String>,

    /// Accent text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-accent-text-color)`
    ///
    /// @since Bot API 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub accent_text_color: Option<String>,

    /// Background color for sections in the `#RRGGBB` format. Recommended to
    /// use in conjunction with `secondary_bg_color`.
    /// CSS: `var(--tg-theme-section-bg-color)`
    ///
    /// @since Bot API 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub section_bg_color: Option<String>,

    /// Section header text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-section-header-text-color)`
    ///
    /// @since Bot API 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub section_header_text_color: Option<String>,

    /// Section separator color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-section-separator-color)`
    ///
    /// @since Bot API 7.6
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub section_separator_color: Option<String>,

    /// Subtitle text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-subtitle-text-color)`
    ///
    /// @since Bot API 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub subtitle_text_color: Option<String>,

    /// Destructive action text color in the `#RRGGBB` format.
    /// CSS: `var(--tg-theme-destructive-text-color)`
    ///
    /// @since Bot API 7.0
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts", ts(optional))]
    pub destructive_text_color: Option<String>,
}
