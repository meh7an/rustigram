use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A Telegram Story posted by a user or channel.
pub struct Story {
    /// The chat that posted the story.
    pub chat: crate::chat::Chat,
    /// Unique identifier of the story in that chat.
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The position and size of a story area overlay.
pub struct StoryAreaPosition {
    /// Horizontal position as a percentage of the story width (0–100).
    pub x_percentage: f64,
    /// Vertical position as a percentage of the story height (0–100).
    pub y_percentage: f64,
    /// Width of the area as a percentage of the story width (0–100).
    pub width_percentage: f64,
    /// Height of the area as a percentage of the story height (0–100).
    pub height_percentage: f64,
    /// Rotation angle of the area in degrees (0–360).
    pub rotation_angle: f64,
    /// Corner radius of the area as a percentage of the shorter side (0–100).
    pub corner_radius_percentage: f64,
}

/// The type of interactive area overlaid on a story.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryAreaType {
    /// An area pointing to a geographic location.
    Location {
        /// The geographic location.
        location: crate::chat::Location,
        /// Optional human-readable address.
        address: Option<LocationAddress>,
    },
    /// An area for a suggested emoji reaction.
    SuggestedReaction {
        /// The reaction type.
        reaction_type: crate::message::ReactionType,
        /// `true` to display a dark background for the reaction.
        is_dark: Option<bool>,
        /// `true` to mirror the reaction emoji.
        is_flipped: Option<bool>,
    },
    /// An area that links to a URL.
    Link {
        /// The URL to open.
        url: String,
    },
    /// An area showing weather information.
    Weather {
        /// Temperature in Celsius.
        temperature: f64,
        /// Weather emoji.
        emoji: String,
        /// Background color as an ARGB integer.
        background_color: i32,
    },
    /// An area highlighting a unique gift.
    UniqueGift {
        /// Name of the unique gift.
        name: String,
    },
}

/// A human-readable address associated with a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code.
    pub country_code: String,
    /// State or region name.
    pub state: Option<String>,
    /// City name.
    pub city: Option<String>,
    /// Street name and number.
    pub street: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An interactive area overlaid on a story frame.
///
/// Areas can represent locations, reactions, links, weather widgets, or
/// unique gift labels. Used when posting stories via the Business API.
pub struct StoryArea {
    /// Position and size of the area.
    pub position: StoryAreaPosition,
    /// Type and content of the area.
    #[serde(rename = "type")]
    pub kind: StoryAreaType,
}

// ─── InputStoryContent ────────────────────────────────────────────────────────

/// A photo to post as a story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputStoryContentPhoto {
    /// The photo to post.
    ///
    /// Must be 1080×1920 pixels, ≤10 MB.
    /// Pass a `file_id`, HTTP URL, or `"attach://<name>"` for a multipart upload.
    pub photo: String,
}

/// A video to post as a story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputStoryContentVideo {
    /// The video to post.
    ///
    /// Must be 720×1280, streamable H.265, with key frames every second, ≤30 MB.
    /// Pass a `file_id`, HTTP URL, or `"attach://<name>"` for a multipart upload.
    pub video: String,
    /// Precise duration of the video in seconds (0–60).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Timestamp in seconds of the frame to use as the static cover. Defaults to `0.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_frame_timestamp: Option<f64>,
    /// Pass `true` if the video has no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_animation: Option<bool>,
}

/// The content of a story to post or edit.
///
/// Pass to [`postStory`](https://core.telegram.org/bots/api#poststory) and
/// [`editStory`](https://core.telegram.org/bots/api#editstory) as a
/// `serde_json::Value` — use `serde_json::to_value(&content)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputStoryContent {
    /// A photo story.
    Photo(InputStoryContentPhoto),
    /// A video story.
    Video(InputStoryContentVideo),
}
