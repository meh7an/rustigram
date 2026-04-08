use serde::{Deserialize, Serialize};

/// One size of a photo or file thumbnail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Photo width in pixels.
    pub width: u32,
    /// Photo height in pixels.
    pub height: u32,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// A file ready to be downloaded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    /// Relative file path for constructing the download URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl File {
    /// Constructs the full download URL for this file.
    #[must_use]
    pub fn url(&self, token: &str) -> Option<String> {
        self.file_path
            .as_ref()
            .map(|path| format!("https://api.telegram.org/file/bot{token}/{path}"))
    }
}

/// Audio file to be treated as music.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Duration of the audio in seconds.
    pub duration: u32,
    /// Performer of the audio as defined by the sender or audio tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Title of the audio as defined by the sender or audio tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Original filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    /// Thumbnail of the album cover.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

/// General file (not photo, voice, audio, or video).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Document thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Original filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// A video file of a specific quality.
///
/// Returned inside [`Video::qualities`] when multiple quality levels are available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoQuality {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Video width in pixels.
    pub width: u32,
    /// Video height in pixels.
    pub height: u32,
    /// Codec used to encode the video (e.g. `"h264"`, `"h265"`, `"av01"`).
    pub codec: String,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Video file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Video width in pixels.
    pub width: u32,
    /// Video height in pixels.
    pub height: u32,
    /// Duration of the video in seconds.
    pub duration: u32,
    /// Video thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Cover image for the video in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<Vec<PhotoSize>>,
    /// Start timestamp for video chapters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<u32>,
    /// Other available quality levels for this video (Bot API 9.4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<Vec<VideoQuality>>,
    /// Original filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Animation file (GIF or H.264/MPEG-4 AVC, no sound).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Animation width in pixels.
    pub width: u32,
    /// Animation height in pixels.
    pub height: u32,
    /// Duration of the animation in seconds.
    pub duration: u32,
    /// Animation thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// Original filename as defined by the sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// MIME type of the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Voice note (OGG/OPUS audio).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Duration of the voice note in seconds.
    pub duration: u32,
    /// MIME type of the voice note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Rounded-square MPEG4 video note.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNote {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// Video width and height (diameter of the circle).
    pub length: u32,
    /// Duration of the video in seconds.
    pub duration: u32,
    /// Video thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

/// Represents a file to be sent.
#[derive(Debug, Clone)]
pub enum InputFile {
    /// Send an existing file by its Telegram `file_id`.
    FileId(String),
    /// Send a file from a URL (photo ≤5 MB, others ≤20 MB).
    Url(String),
    /// Upload new file bytes.
    Bytes {
        /// Original filename sent in the Content-Disposition header.
        filename: String,
        /// Raw file bytes.
        data: Vec<u8>,
        /// MIME type of the file.
        mime_type: String,
    },
    /// Reference an already-included multipart attachment by `attach://<name>`.
    Attach(String),
}

impl InputFile {
    /// Returns the string representation for JSON/query-string fields.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::FileId(id) => id,
            Self::Url(url) => url,
            Self::Attach(name) => name,
            Self::Bytes { filename, .. } => filename,
        }
    }

    /// Returns `true` if this variant needs multipart form upload.
    #[must_use]
    pub fn requires_multipart(&self) -> bool {
        matches!(self, Self::Bytes { .. })
    }
}

/// Encrypted passport file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportFile {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// File size in bytes.
    pub file_size: u64,
    /// Unix timestamp when the file was uploaded.
    pub file_date: i64,
}

// ─── InputMedia ───────────────────────────────────────────────────────────────
//
// Used by `sendMediaGroup` and `editMessageMedia`. The `media` field accepts a
// `file_id`, HTTP URL, or `"attach://<n>"` for a multipart attachment.

/// A photo to include in a media group or replace an existing media message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaPhoto {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
    /// Caption (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// `true` if the photo needs a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// A video to include in a media group or replace an existing media message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaVideo {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
    /// Thumbnail: `file_id` or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Cover image: `file_id`, HTTP URL, or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// Start timestamp for the video in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Caption (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Video width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Video height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// `true` if the uploaded video is suitable for streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// `true` if the video needs a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// An animation (GIF or silent H.264) to include in a media group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaAnimation {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
    /// Thumbnail: `file_id` or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Caption (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Animation width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Animation height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Animation duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// `true` if the animation needs a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

/// An audio file to include in a media group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaAudio {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
    /// Thumbnail: `file_id` or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Caption (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Audio duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// Performer of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Title of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// A document (general file) to include in a media group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMediaDocument {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
    /// Thumbnail: `file_id` or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Caption (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` to disable automatic server-side content type detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

/// The media content of a message to be sent or edited.
///
/// Serialise with `serde_json::to_value(&media)` to pass to `sendMediaGroup`
/// or `editMessageMedia` until the API methods are updated to accept this type
/// directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    /// A photo.
    Photo(InputMediaPhoto),
    /// A video.
    Video(InputMediaVideo),
    /// An animation (GIF or silent H.264).
    Animation(InputMediaAnimation),
    /// An audio file treated as music.
    Audio(InputMediaAudio),
    /// A general document.
    Document(InputMediaDocument),
}

// ─── InputPaidMedia ───────────────────────────────────────────────────────────

/// A photo to send as paid media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPaidMediaPhoto {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
}

/// A video to send as paid media.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPaidMediaVideo {
    /// File to send: `file_id`, HTTP URL, or `"attach://<n>"`.
    pub media: String,
    /// Thumbnail: `file_id` or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// Cover image: `file_id`, HTTP URL, or `"attach://<n>"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// Start timestamp for the video in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Video width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Video height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
    /// `true` if the uploaded video is suitable for streaming.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

/// Paid media to send via `sendPaidMedia`.
///
/// Serialise with `serde_json::to_value(&media)` to pass to `sendPaidMedia`
/// until the method is updated to accept this type directly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    /// A photo.
    Photo(InputPaidMediaPhoto),
    /// A video.
    Video(InputPaidMediaVideo),
}

// ─── InputProfilePhoto ────────────────────────────────────────────────────────

/// A static profile photo in JPEG format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputProfilePhotoStatic {
    /// The photo file.
    ///
    /// Profile photos cannot be reused — upload as a new file using
    /// `"attach://<n>"` via multipart/form-data.
    pub photo: String,
}

/// An animated profile photo in MPEG4 format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputProfilePhotoAnimated {
    /// The animation file.
    ///
    /// Profile photos cannot be reused — upload as a new file using
    /// `"attach://<n>"` via multipart/form-data.
    pub animation: String,
    /// Timestamp in seconds of the frame to use as the static profile photo.
    /// Defaults to `0.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,
}

/// A profile photo to set via `setMyProfilePhoto` or `setBusinessAccountProfilePhoto`.
///
/// Serialise with `serde_json::to_string(&photo)` to pass to the relevant method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto {
    /// A static JPEG profile photo.
    Static(InputProfilePhotoStatic),
    /// An animated MPEG4 profile photo.
    Animated(InputProfilePhotoAnimated),
}
