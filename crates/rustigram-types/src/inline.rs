use crate::chat::Location;
use crate::user::User;
use serde::{Deserialize, Serialize};

/// An incoming inline query sent when a user types `@YourBot something` in any chat.
///
/// Respond with [`answerInlineQuery`](https://core.telegram.org/bots/api#answerinlinequery)
/// within 10 seconds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query.
    pub id: String,
    /// The user who sent the query.
    pub from: User,
    /// Text of the query (up to 256 characters).
    pub query: String,
    /// Offset of the result to be returned.
    pub offset: String,
    /// Type of the chat from which the query was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// Sender's location, if the bot requests it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// The result a user chose from an inline query.
///
/// Delivered only when the bot has been granted access to inline feedback
/// via [@BotFather](https://t.me/BotFather) under "Inline Feedback".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// Identifier of the chosen result.
    pub result_id: String,
    /// The user who chose the result.
    pub from: User,
    /// Sender's location, if the bot requests it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Identifier of the sent inline message, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// The query used to obtain the result.
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
/// One result to show in an inline query answer.
///
/// Up to 50 results can be returned per [`answerInlineQuery`](https://core.telegram.org/bots/api#answerinlinequery) call.
/// Each variant corresponds to a different content type (article, photo,
/// video, etc.). Cached variants re-use a previously uploaded Telegram
/// `file_id` rather than a URL.
pub enum InlineQueryResult {
    /// A link to an article or web page.
    Article(InlineQueryResultArticle),
    /// A link to a photo.
    Photo(InlineQueryResultPhoto),
    /// A link to an animated GIF.
    Gif(InlineQueryResultGif),
    /// A link to a video animation (MPEG4 without sound).
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    /// A link to a video.
    Video(InlineQueryResultVideo),
    /// A link to an audio file.
    Audio(InlineQueryResultAudio),
    /// A link to a voice recording.
    Voice(InlineQueryResultVoice),
    /// A link to a general file.
    Document(InlineQueryResultDocument),
    /// A geographic location.
    Location(InlineQueryResultLocation),
    /// A venue.
    Venue(InlineQueryResultVenue),
    /// A contact.
    Contact(InlineQueryResultContact),
    /// A game.
    Game(InlineQueryResultGame),
    /// A photo from a Telegram `file_id`.
    CachedPhoto(InlineQueryResultCachedPhoto),
    /// A GIF from a Telegram `file_id`.
    CachedGif(InlineQueryResultCachedGif),
    /// An MPEG4 GIF from a Telegram `file_id`.
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    /// A sticker from a Telegram `file_id`.
    CachedSticker(InlineQueryResultCachedSticker),
    /// A document from a Telegram `file_id`.
    CachedDocument(InlineQueryResultCachedDocument),
    /// A video from a Telegram `file_id`.
    CachedVideo(InlineQueryResultCachedVideo),
    /// A voice message from a Telegram `file_id`.
    CachedVoice(InlineQueryResultCachedVoice),
    /// An audio file from a Telegram `file_id`.
    CachedAudio(InlineQueryResultCachedAudio),
}

// ─── URL-based results ────────────────────────────────────────────────────────

/// A link to an article or web page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Title of the result.
    pub title: String,
    /// Content of the message to be sent.
    pub input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

/// A link to a photo (JPEG, max 5 MB).
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A photo result in an inline query.
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL of the photo.
    pub photo_url: String,
    /// URL of the thumbnail for the photo.
    pub thumbnail_url: String,
    /// Photo width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,
    /// Photo height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,
    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the photo (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A link to an animated GIF file (max 1 MB).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the GIF file.
    pub gif_url: String,
    /// URL of the static or animated thumbnail for the result.
    pub thumbnail_url: String,
    /// Width of the GIF in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<u32>,
    /// Height of the GIF in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<u32>,
    /// Duration of the GIF in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<u32>,
    /// MIME type of the thumbnail (`image/jpeg`, `image/gif`, or `video/mp4`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the GIF (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the GIF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A link to a video animation (MPEG4 without sound, max 1 MB).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the MPEG4 file.
    pub mpeg4_url: String,
    /// URL of the static or animated thumbnail.
    pub thumbnail_url: String,
    /// Video width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<u32>,
    /// Video height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<u32>,
    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<u32>,
    /// MIME type of the thumbnail (`image/jpeg`, `image/gif`, or `video/mp4`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<String>,
    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the MPEG4 (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A link to a video file (`text/html` or `video/mp4`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the video file.
    pub video_url: String,
    /// MIME type of the video (`text/html` or `video/mp4`).
    pub mime_type: String,
    /// URL of the thumbnail for the video.
    pub thumbnail_url: String,
    /// Title for the result.
    pub title: String,
    /// Caption of the video (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Video width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<u32>,
    /// Video height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<u32>,
    /// Video duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<u32>,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A link to an audio file (`.mp3` or `.m4a`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the audio file.
    pub audio_url: String,
    /// Title.
    pub title: String,
    /// Caption of the audio (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Performer of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Audio duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<u32>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A link to a voice recording in `.ogg` format encoded with OPUS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the voice recording.
    pub voice_url: String,
    /// Recording title.
    pub title: String,
    /// Caption of the voice recording (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Recording duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<u32>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice recording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A link to a general file (`application/pdf` or `application/zip`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Title for the result.
    pub title: String,
    /// Caption of the document (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// A valid URL for the file.
    pub document_url: String,
    /// MIME type of the document (`application/pdf` or `application/zip`).
    pub mime_type: String,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

/// A geographic location on a map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Location latitude in degrees.
    pub latitude: f64,
    /// Location longitude in degrees.
    pub longitude: f64,
    /// Location title.
    pub title: String,
    /// Radius of uncertainty for the location in metres (0–1500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated (60–86400).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// Direction of movement in degrees (1–360) for live locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// Maximum distance in metres for proximity alerts about approaching another chat member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

/// A venue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Venue latitude in degrees.
    pub latitude: f64,
    /// Venue longitude in degrees.
    pub longitude: f64,
    /// Venue title.
    pub title: String,
    /// Venue address.
    pub address: String,
    /// Foursquare identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

/// A contact with a phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Contact phone number.
    pub phone_number: String,
    /// Contact first name.
    pub first_name: String,
    /// Contact last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Contact vCard (0–2048 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<u32>,
}

/// A game result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Short name of the game.
    pub game_short_name: String,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
}

// ─── Cached (file_id-based) results ──────────────────────────────────────────

/// A photo from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the photo.
    pub photo_file_id: String,
    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the photo (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// An animated GIF from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the GIF.
    pub gif_file_id: String,
    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the GIF (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the GIF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// An MPEG4 animation from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the MPEG4 animation.
    pub mpeg4_file_id: String,
    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Caption of the animation (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A sticker from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the sticker.
    pub sticker_file_id: String,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A document from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Title for the result.
    pub title: String,
    /// A valid Telegram `file_id` of the document.
    pub document_file_id: String,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the document (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A video from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVideo {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the video.
    pub video_file_id: String,
    /// Title for the result.
    pub title: String,
    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Caption of the video (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// `true` if the caption must be shown above the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// A voice message from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the voice message.
    pub voice_file_id: String,
    /// Title for the result.
    pub title: String,
    /// Caption of the voice message (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

/// An audio file from a previously uploaded Telegram file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the audio file.
    pub audio_file_id: String,
    /// Caption of the audio (0–1024 characters after entities parsing).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Parse mode for the caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the caption; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::message::MessageEntity>>,
    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}

// ─── InputMessageContent ──────────────────────────────────────────────────────

/// The content of a message sent as the result of an inline query.
///
/// # Variant order is load-bearing
///
/// This enum is `#[serde(untagged)]`, so serde takes the first variant that
/// matches, and it ignores fields the variant does not declare. `Venue`
/// requires everything `Location` requires (`latitude`, `longitude`) plus
/// `title` and `address` — a strict superset — so `Venue` must be tried first.
/// With `Location` first, every venue deserialized as a location and silently
/// lost its title and address.
///
/// Adding a variant whose required fields are a superset of an existing one
/// means placing it above that one. `Venue` and `Location` are the only such
/// pair today; the others have disjoint required fields and their order is free.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// The message text.
    Text(InputTextMessageContent),
    /// A rich formatted message.
    Rich(crate::rich_message::InputRichMessageContent),
    /// A venue. Declared before [`Location`](Self::Location) — see the note above.
    Venue(InputVenueMessageContent),
    /// A location on a map.
    Location(InputLocationMessageContent),
    /// A contact.
    Contact(InputContactMessageContent),
    /// An invoice.
    Invoice(InputInvoiceMessageContent),
}

/// A text message to send as an inline query result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTextMessageContent {
    /// Text of the message (1–4096 characters).
    pub message_text: String,
    /// Parse mode for the message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the message text; alternative to `parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::message::MessageEntity>>,
    /// Options for link preview generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<crate::message::LinkPreviewOptions>,
}

/// A live location message to send as an inline query result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
    /// Radius of uncertainty for the location in metres (0–1500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated (60–86400).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// Direction of movement in degrees (1–360).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// Maximum distance in metres for proximity alerts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
}

/// A venue message to send as an inline query result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputVenueMessageContent {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
    /// Venue name.
    pub title: String,
    /// Venue address.
    pub address: String,
    /// Foursquare identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

/// A contact message to send as an inline query result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    /// Contact phone number.
    pub phone_number: String,
    /// Contact first name.
    pub first_name: String,
    /// Contact last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Contact vCard (0–2048 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

/// An invoice message to send as an inline query result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputInvoiceMessageContent {
    /// Product name (1–32 characters).
    pub title: String,
    /// Product description (1–255 characters).
    pub description: String,
    /// Bot-defined invoice payload (1–128 bytes).
    pub payload: String,
    /// Payment provider token; not required for Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Price breakdown as a list of labeled portions.
    pub prices: Vec<crate::payments::LabeledPrice>,
    /// Maximum accepted tip amount in the smallest currency unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<u64>,
    /// Suggested tip amounts in the smallest currency unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<u64>>,
    /// JSON-encoded data about the invoice for the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<u64>,
    /// Photo width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<u32>,
    /// Photo height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<u32>,
    /// Requests the buyer's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Requests the buyer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Requests the buyer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Requests the buyer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Passes the buyer's phone number to the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Passes the buyer's email address to the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// `true` if the final price depends on the shipping method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

// ─── Misc ─────────────────────────────────────────────────────────────────────

/// A message sent from a Web App on behalf of the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message, if one was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// An inline message sent by a guest bot.
///
/// Returned by [`answerGuestQuery`](https://core.telegram.org/bots/api#answerguestquery).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: String,
}

/// A button shown above inline query results.
///
/// Exactly one of `web_app` or `start_parameter` should be set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InlineQueryResultsButton {
    /// Label text on the button.
    pub text: String,
    /// Description of the Web App launched when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<crate::message::WebAppInfo>,
    /// Deep-linking parameter for the /start message sent when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

/// An inline message prepared for sending by a Mini App.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message.
    pub id: String,
    /// Point in time when the prepared message can no longer be used, as a Unix timestamp.
    pub expiration_date: i64,
}
