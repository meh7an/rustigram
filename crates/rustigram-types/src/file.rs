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

/// This object represents a file ready to be downloaded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// Telegram file identifier.
    pub file_id: String,
    /// Unique file identifier, stable across bots and time.
    pub file_unique_id: String,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    /// Relative file path — use with
    /// `https://api.telegram.org/file/bot<TOKEN>/<file_path>`.
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

/// General file (not photo, voice, audio or video).
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
    /// Cover image for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<Vec<PhotoSize>>,
    /// Start timestamp for video chapters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<u32>,
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
    /// Upload new file bytes. The `filename` is sent in the Content-Disposition header.
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
