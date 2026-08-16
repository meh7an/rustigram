use serde::{Deserialize, Serialize};

use crate::chat::Location;
use crate::sticker::Sticker;

/// The intro shown on a business account's profile.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BusinessIntro {
    /// Title text of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Message text of the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Sticker shown in the business intro.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
}

/// The physical location of a business account.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BusinessLocation {
    /// Address of the business.
    pub address: String,
    /// Location of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// A single interval in a business account's opening hours.
///
/// Minutes are counted from the start of the week — 0 is midnight on Monday in
/// the account's [`time_zone_name`](BusinessOpeningHours::time_zone_name).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BusinessOpeningHoursInterval {
    /// Minute of the week when the business opens.
    pub opening_minute: u32,
    /// Minute of the week when the business closes.
    pub closing_minute: u32,
}

/// The opening hours of a business account.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone the opening hours are defined in.
    pub time_zone_name: String,
    /// List of time intervals describing business opening hours.
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

/// A user's birthdate.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Birthdate {
    /// Day of the user's birth (1–31).
    pub day: u8,
    /// Month of the user's birth (1–12).
    pub month: u8,
    /// Year of the user's birth, if the user chose to share it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
}

/// A user's Telegram rating.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserRating {
    /// Current level of the user.
    pub level: i64,
    /// Current rating of the user.
    pub rating: i64,
    /// Rating required to reach the current level.
    pub current_level_rating: i64,
    /// Rating required to reach the next level; omitted at the maximum level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_level_rating: Option<i64>,
}
