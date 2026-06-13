//! Telegram Mini App safe area inset types.
//!
//! Safe area insets provide padding values that keep Mini App content clear
//! of system and Telegram UI elements — hardware notches, navigation bars,
//! the Telegram header, and the bottom bar. Both types are updated at runtime
//! via TMA events and should be consumed reactively rather than read once.
//!
//! @see <https://core.telegram.org/bots/webapps#safeareainset>
//! @see <https://core.telegram.org/bots/webapps#contentsafeareainset>

use serde::{Deserialize, Serialize};

/// Represents the system-defined safe area insets, providing padding values
/// to ensure content remains within visible boundaries, avoiding overlap with
/// system UI elements like notches or navigation bars.
///
/// Updated via the `safeAreaChanged` event.
///
/// @since Bot API 8.0
/// @see <https://core.telegram.org/bots/webapps#safeareainset>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct SafeAreaInset {
    /// Top inset in pixels.
    pub top: i32,
    /// Bottom inset in pixels.
    pub bottom: i32,
    /// Left inset in pixels.
    pub left: i32,
    /// Right inset in pixels.
    pub right: i32,
}

/// Represents the content-defined safe area insets, providing padding values
/// to ensure content remains within visible boundaries, avoiding overlap with
/// Telegram UI elements such as the header or bottom bar.
///
/// Updated via the `contentSafeAreaChanged` event.
///
/// @since Bot API 8.0
/// @see <https://core.telegram.org/bots/webapps#contentsafeareainset>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct ContentSafeAreaInset {
    /// Top inset in pixels.
    pub top: i32,
    /// Bottom inset in pixels.
    pub bottom: i32,
    /// Left inset in pixels.
    pub left: i32,
    /// Right inset in pixels.
    pub right: i32,
}
