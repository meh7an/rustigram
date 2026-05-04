use crate::chat::Location;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An incoming inline query.
///
/// Sent when a user types `@YourBot something` in any chat. Respond with
/// [`answerInlineQuery`](https://core.telegram.org/bots/api#answerinlinequery)
/// within 10 seconds.
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
    pub chat_type: Option<String>,
    /// Sender's location, if enabled.
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The result a user chose from an inline query.
///
/// Delivered only when the bot has been granted access to inline feedback.
/// Set up in [@BotFather](https://t.me/BotFather) under "Inline Feedback".
pub struct ChosenInlineResult {
    /// Identifier of the chosen result.
    pub result_id: String,
    /// The user who chose the result.
    pub from: User,
    /// Sender's location, if enabled.
    pub location: Option<Location>,
    /// Identifier of the sent inline message, if applicable.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents an article link in an inline query result.
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Title of the result.
    pub title: String,
    /// Content of the message to be sent.
    pub input_message_content: InputMessageContent,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// URL of the result.
    pub url: Option<String>,
    /// Short description of the result.
    pub description: Option<String>,
    /// URL of the thumbnail for the result.
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    pub thumbnail_height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A photo result in an inline query.
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL of the photo (JPEG, max 5 MB).
    pub photo_url: String,
    /// URL of the thumbnail for the photo.
    pub thumbnail_url: String,
    /// Photo width in pixels.
    pub photo_width: Option<u32>,
    /// Photo height in pixels.
    pub photo_height: Option<u32>,
    /// Title for the result.
    pub title: Option<String>,
    /// Short description of the result.
    pub description: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An animated GIF result in an inline query.
pub struct InlineQueryResultGif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the GIF file (max 1 MB).
    pub gif_url: String,
    /// URL of the static or animated thumbnail for the result.
    pub thumbnail_url: String,
    /// Width of the GIF in pixels.
    pub gif_width: Option<u32>,
    /// Height of the GIF in pixels.
    pub gif_height: Option<u32>,
    /// Duration of the GIF in seconds.
    pub gif_duration: Option<u32>,
    /// Title for the result.
    pub title: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// An MPEG4 animated GIF result in an inline query.
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the MPEG4 file (max 1 MB).
    pub mpeg4_url: String,
    /// URL of the static or animated thumbnail.
    pub thumbnail_url: String,
    /// Video width in pixels.
    pub mpeg4_width: Option<u32>,
    /// Video height in pixels.
    pub mpeg4_height: Option<u32>,
    /// Video duration in seconds.
    pub mpeg4_duration: Option<u32>,
    /// Title for the result.
    pub title: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A video result in an inline query.
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
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Video width in pixels.
    pub video_width: Option<u32>,
    /// Video height in pixels.
    pub video_height: Option<u32>,
    /// Video duration in seconds.
    pub video_duration: Option<u32>,
    /// Short description of the result.
    pub description: Option<String>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// An audio result in an inline query.
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the audio file.
    pub audio_url: String,
    /// Title.
    pub title: String,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Performer.
    pub performer: Option<String>,
    /// Audio duration in seconds.
    pub audio_duration: Option<u32>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A voice recording result in an inline query.
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid URL for the voice recording.
    pub voice_url: String,
    /// Recording title.
    pub title: String,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Recording duration in seconds.
    pub voice_duration: Option<u32>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A general file result in an inline query.
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Title for the result.
    pub title: String,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// A valid URL for the file.
    pub document_url: String,
    /// MIME type of the document (`application/pdf` or `application/zip`).
    pub mime_type: String,
    /// Short description of the result.
    pub description: Option<String>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    pub thumbnail_height: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A location result in an inline query.
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Location latitude in degrees.
    pub latitude: f64,
    /// Location longitude in degrees.
    pub longitude: f64,
    /// Location title.
    pub title: String,
    /// Radius of uncertainty for the location, in metres (0–1500).
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated (60–86400).
    pub live_period: Option<u32>,
    /// Direction of movement in degrees (1–360) for live locations.
    pub heading: Option<u16>,
    /// Maximum distance in metres for proximity alerts.
    pub proximity_alert_radius: Option<u32>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    pub thumbnail_height: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A venue result in an inline query.
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
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue.
    pub google_place_id: Option<String>,
    /// Google Places type of the venue.
    pub google_place_type: Option<String>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    pub thumbnail_height: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A contact result in an inline query.
pub struct InlineQueryResultContact {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Contact phone number.
    pub phone_number: String,
    /// Contact first name.
    pub first_name: String,
    /// Contact last name.
    pub last_name: Option<String>,
    /// Contact vCard (0–2048 bytes).
    pub vcard: Option<String>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// URL of the thumbnail for the result.
    pub thumbnail_url: Option<String>,
    /// Thumbnail width in pixels.
    pub thumbnail_width: Option<u32>,
    /// Thumbnail height in pixels.
    pub thumbnail_height: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A game result in an inline query.
pub struct InlineQueryResultGame {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// Short name of the game.
    pub game_short_name: String,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A photo result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub photo_file_id: String,
    /// Title for the result.
    pub title: Option<String>,
    /// Short description of the result.
    pub description: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// An animated GIF result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub gif_file_id: String,
    /// Title for the result.
    pub title: Option<String>,
    /// Short description of the result.
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// An MPEG4 GIF result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub mpeg4_file_id: String,
    /// Title for the result.
    pub title: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A sticker result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub sticker_file_id: String,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A document result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub title: String,
    /// A valid Telegram `file_id` of the cached media.
    pub document_file_id: String,
    /// Short description of the result.
    pub description: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A video result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedVideo {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub video_file_id: String,
    /// Title for the result.
    pub title: String,
    /// Short description of the result.
    pub description: Option<String>,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
    /// Show the caption above the media instead of below.
    pub show_caption_above_media: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// A voice result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub voice_file_id: String,
    /// Title for the result.
    pub title: String,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// An audio result using a previously uploaded Telegram `file_id`.
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result (1–64 bytes).
    pub id: String,
    /// A valid Telegram `file_id` of the cached media.
    pub audio_file_id: String,
    /// Caption of the media (0–1024 characters).
    pub caption: Option<String>,
    /// Parse mode for the caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<crate::keyboard::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the media.
    pub input_message_content: Option<InputMessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
/// The content of a message sent as the result of an inline query.
pub enum InputMessageContent {
    /// The message text.
    Text(InputTextMessageContent),
    /// A location on a map.
    Location(InputLocationMessageContent),
    /// A venue.
    Venue(InputVenueMessageContent),
    /// A contact.
    Contact(InputContactMessageContent),
    /// An invoice.
    Invoice(InputInvoiceMessageContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Text message content for an inline query result.
pub struct InputTextMessageContent {
    /// Text of the message (1–4096 characters).
    pub message_text: String,
    /// Parse mode for the message text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<crate::message::ParseMode>,
    /// Special entities in the message text.
    pub entities: Option<Vec<crate::message::MessageEntity>>,
    /// Options for link preview generation.
    pub link_preview_options: Option<crate::message::LinkPreviewOptions>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Live location content for an inline query result.
pub struct InputLocationMessageContent {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
    /// Radius of uncertainty for the location, in metres (0–1500).
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated (60–86400).
    pub live_period: Option<u32>,
    /// Direction of movement in degrees (1–360).
    pub heading: Option<u16>,
    /// Maximum distance for proximity alerts in metres.
    pub proximity_alert_radius: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Venue content for an inline query result.
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
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    pub foursquare_type: Option<String>,
    /// Google Places identifier.
    pub google_place_id: Option<String>,
    /// Google Places type.
    pub google_place_type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contact content for an inline query result.
pub struct InputContactMessageContent {
    /// Contact phone number.
    pub phone_number: String,
    /// Contact first name.
    pub first_name: String,
    /// Contact last name.
    pub last_name: Option<String>,
    /// Contact vCard (0–2048 bytes).
    pub vcard: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Invoice content for an inline query result.
pub struct InputInvoiceMessageContent {
    /// Product name (1–32 characters).
    pub title: String,
    /// Product description (1–255 characters).
    pub description: String,
    /// Bot-defined invoice payload (1–128 bytes).
    pub payload: String,
    /// Payment provider token. Not required for Telegram Stars.
    pub provider_token: Option<String>,
    /// Three-letter ISO 4217 currency code.
    pub currency: String,
    /// Price breakdown.
    pub prices: Vec<crate::payments::LabeledPrice>,
    /// Maximum accepted tip amount in the smallest currency unit.
    pub max_tip_amount: Option<u64>,
    /// Suggested tip amounts in the smallest currency unit.
    pub suggested_tip_amounts: Option<Vec<u64>>,
    /// JSON-encoded data about the invoice for the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo.
    pub photo_url: Option<String>,
    /// Photo size in bytes.
    pub photo_size: Option<u64>,
    /// Photo width in pixels.
    pub photo_width: Option<u32>,
    /// Photo height in pixels.
    pub photo_height: Option<u32>,
    /// Requests the buyer's full name.
    pub need_name: Option<bool>,
    /// Requests the buyer's phone number.
    pub need_phone_number: Option<bool>,
    /// Requests the buyer's email address.
    pub need_email: Option<bool>,
    /// Requests the buyer's shipping address.
    pub need_shipping_address: Option<bool>,
    /// Passes the buyer's phone number to the provider.
    pub send_phone_number_to_provider: Option<bool>,
    /// Passes the buyer's email address to the provider.
    pub send_email_to_provider: Option<bool>,
    /// `true` if the final price depends on the shipping method.
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A message sent from a Web App on behalf of the user.
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: Option<String>,
}

/// Describes an inline message sent by a guest bot.
///
/// Returned by [`answerGuestQuery`](https://core.telegram.org/bots/api#answerguestquery).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentGuestMessage {
    /// Identifier of the sent inline message.
    pub inline_message_id: String,
}
