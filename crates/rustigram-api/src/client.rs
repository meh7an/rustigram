use std::sync::Arc;
use std::time::Duration;

use reqwest::multipart::{Form, Part};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, warn};

use crate::error::{Error, Result};
use crate::methods::bot_settings::*;
use crate::methods::business::*;
use crate::methods::chat_management::*;
use crate::methods::editing::*;
use crate::methods::forum::*;
use crate::methods::games::*;
use crate::methods::getters::*;
use crate::methods::gifts::*;
use crate::methods::inline::*;
use crate::methods::miniapp::*;
use crate::methods::passport::*;
use crate::methods::payments::*;
use crate::methods::reactions::*;
use crate::methods::sending::*;
use crate::methods::stickers::*;
use crate::methods::stories::*;
use crate::methods::updates::*;
use crate::methods::verification::*;

// ─── Wire-format API response ─────────────────────────────────────────────────

// #[serde(bound(...))] overrides the auto-generated bounds so serde does not
// require T: Default just because the `result` field uses #[serde(default)].
#[derive(serde::Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
struct ApiResponse<T> {
    ok: bool,
    #[serde(default)]
    result: Option<T>,
    description: Option<String>,
    error_code: Option<u16>,
    parameters: Option<ResponseParameters>,
}

#[derive(serde::Deserialize)]
struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<u32>,
}

// ─── ClientConfig ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
/// Configuration for [`BotClient`].
///
/// Use the builder methods to customise behaviour, then pass the config to
/// [`BotClient::new`].
///
/// # Example
///
/// ```rust,ignore
/// use std::time::Duration;
///
/// let config = ClientConfig::new("123456:ABC...")?
///     .api_base_url("http://localhost:8081") // local Bot API server
///     .timeout(Duration::from_secs(60))
///     .max_retries(5);
/// ```
pub struct ClientConfig {
    /// Bot token used to authenticate with the Telegram API.
    pub token: String,
    /// Base URL of the Bot API server (default: `https://api.telegram.org`).
    pub api_base_url: String,
    /// Per-request HTTP timeout.
    pub timeout: Duration,
    /// Maximum number of automatic retries on flood control responses.
    pub max_retries: u8,
}

impl ClientConfig {
    /// Creates a new `ClientConfig` with the given bot token and default settings.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        let token = token.into();
        validate_token(&token)?;
        Ok(Self {
            token,
            api_base_url: "https://api.telegram.org".to_owned(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        })
    }

    /// Sets a custom base URL for API requests, e.g. for a local Bot API server.
    #[must_use]
    pub fn api_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = url.into();
        self
    }

    /// Sets a custom timeout for API requests (default 30 seconds).
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the maximum number of retries on HTTP 429 (flood control) errors (default 3).
    #[must_use]
    pub fn max_retries(mut self, n: u8) -> Self {
        self.max_retries = n;
        self
    }
}

// ─── BotClient ────────────────────────────────────────────────────────────────

struct Inner {
    http: reqwest::Client,
    config: ClientConfig,
}

#[derive(Clone)]
/// The Telegram Bot API HTTP client.
///
/// `BotClient` is cheap to clone — all internal state is reference-counted.
/// It is safe to share across tasks and threads without additional
/// synchronisation.
///
/// # Creating a client
///
/// ```rust,ignore
/// // From a token string (simplest)
/// let client = BotClient::from_token("123456:ABC...")?;
///
/// // From a ClientConfig for advanced options
/// let config = ClientConfig::new("123456:ABC...")?
///     .api_base_url("http://localhost:8081")
///     .timeout(Duration::from_secs(60));
/// let client = BotClient::new(config)?;
/// ```
///
/// # Making API calls
///
/// Every Bot API method is available as a method on `BotClient`. Each method
/// returns a builder — set optional parameters with chained calls, then
/// `.await` to execute:
///
/// ```rust,ignore
/// client
///     .send_message(chat_id, "Hello!")
///     .parse_mode(ParseMode::HTML)
///     .disable_notification(true)
///     .await?;
/// ```
pub struct BotClient {
    inner: Arc<Inner>,
}

impl BotClient {
    /// Creates a new `BotClient` from a [`ClientConfig`].
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be initialised.
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(Error::Http)?;
        Ok(Self {
            inner: Arc::new(Inner { http, config }),
        })
    }

    /// Creates a `BotClient` directly from a bot token string.
    ///
    /// This is equivalent to `BotClient::new(ClientConfig::new(token)?)`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidToken`] if the token format is invalid.
    pub fn from_token(token: impl Into<String>) -> Result<Self> {
        Self::new(ClientConfig::new(token)?)
    }

    /// Returns the bot token used for authentication.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.inner.config.token
    }

    /// Returns the base URL used for API requests, defaulting to `https://api.telegram.org`.
    #[must_use]
    pub fn api_base_url(&self) -> &str {
        &self.inner.config.api_base_url
    }

    #[must_use]
    fn method_url(&self, method: &str) -> String {
        format!(
            "{}/bot{}/{}",
            self.inner.config.api_base_url, self.inner.config.token, method
        )
    }

    /// Sends a JSON POST request to a Bot API method and deserialises the result.
    ///
    /// Automatically retries on HTTP 429 (flood control) up to `max_retries`
    /// times, waiting the `retry_after` duration between attempts.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure, API error (`ok: false`), or
    /// deserialisation failure.
    pub async fn post_json<P, R>(&self, method: &str, params: &P) -> Result<R>
    where
        P: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.method_url(method);
        let body = serde_json::to_vec(params).map_err(Error::Serialization)?;
        let max_retries = self.inner.config.max_retries;

        for attempt in 0..=max_retries {
            debug!("POST {} (attempt {})", method, attempt + 1);

            let resp = self
                .inner
                .http
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body.clone())
                .send()
                .await
                .map_err(Error::Http)?;

            let api_resp: ApiResponse<R> = resp
                .json()
                .await
                .map_err(|e| Error::Decode(e.to_string()))?;

            if api_resp.ok {
                return api_resp
                    .result
                    .ok_or_else(|| Error::Decode("ok=true but result is null".to_owned()));
            }

            let error_code = api_resp.error_code.unwrap_or(0);
            let description = api_resp
                .description
                .unwrap_or_else(|| "Unknown error".to_owned());
            let retry_after = api_resp.parameters.as_ref().and_then(|p| p.retry_after);
            let migrate_to_chat_id = api_resp
                .parameters
                .as_ref()
                .and_then(|p| p.migrate_to_chat_id);

            if error_code == 429 {
                let wait = retry_after.unwrap_or(1);
                if attempt < max_retries {
                    warn!(
                        "Flood control on {}: waiting {}s (attempt {}/{})",
                        method,
                        wait,
                        attempt + 1,
                        max_retries
                    );
                    tokio::time::sleep(Duration::from_secs(u64::from(wait))).await;
                    continue;
                }
                return Err(Error::RateLimit { retry_after: wait });
            }

            return Err(Error::Api {
                error_code,
                description,
                migrate_to_chat_id,
                retry_after,
            });
        }

        unreachable!()
    }

    /// Sends a multipart/form-data POST request to a Bot API method and deserialises the result.
    pub async fn post_multipart<R>(&self, method: &str, form: Form) -> Result<R>
    where
        R: DeserializeOwned,
    {
        let url = self.method_url(method);
        debug!("POST multipart {}", method);

        let resp = self
            .inner
            .http
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(Error::Http)?;

        let api_resp: ApiResponse<R> = resp
            .json()
            .await
            .map_err(|e| Error::Decode(e.to_string()))?;

        if api_resp.ok {
            return api_resp
                .result
                .ok_or_else(|| Error::Decode("ok=true but result is null".to_owned()));
        }

        let error_code = api_resp.error_code.unwrap_or(0);
        let description = api_resp
            .description
            .unwrap_or_else(|| "Unknown error".to_owned());
        let retry_after = api_resp.parameters.as_ref().and_then(|p| p.retry_after);
        let migrate_to_chat_id = api_resp
            .parameters
            .as_ref()
            .and_then(|p| p.migrate_to_chat_id);

        if error_code == 429 {
            return Err(Error::RateLimit {
                retry_after: retry_after.unwrap_or(1),
            });
        }

        Err(Error::Api {
            error_code,
            description,
            migrate_to_chat_id,
            retry_after,
        })
    }

    /// Downloads a file by its path as returned by [`BotClient::get_file`].
    ///
    /// The file path must be obtained by calling `get_file` first:
    ///
    /// ```rust,ignore
    /// let file = client.get_file(&document.file_id).await?;
    /// let bytes = client.download_file(&file.file_path.unwrap()).await?;
    /// ```
    ///
    /// Maximum file size via the Telegram cloud server is 20 MB.
    /// Use a [local Bot API server](https://github.com/tdlib/telegram-bot-api)
    /// to lift this restriction.
    pub async fn download_file(&self, file_path: &str) -> Result<bytes::Bytes> {
        let url = format!(
            "{}/file/bot{}/{}",
            self.inner.config.api_base_url, self.inner.config.token, file_path
        );
        self.inner
            .http
            .get(&url)
            .send()
            .await
            .map_err(Error::Http)?
            .bytes()
            .await
            .map_err(Error::Http)
    }

    // ── Update methods ────────────────────────────────────────────────────────

    /// Calls `getUpdates` — fetches a batch of incoming updates via long polling.
    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates::new(self.clone())
    }
    /// Calls `setWebhook` — registers a webhook URL with Telegram.
    pub fn set_webhook(&self, url: impl Into<String>) -> SetWebhook {
        SetWebhook::new(self.clone(), url)
    }
    /// Calls `deleteWebhook` — removes the webhook integration.
    pub fn delete_webhook(&self) -> DeleteWebhook {
        DeleteWebhook::new(self.clone())
    }
    /// Calls `getWebhookInfo` — returns the current webhook status.
    pub fn get_webhook_info(&self) -> GetWebhookInfo {
        GetWebhookInfo::new(self.clone())
    }

    // ── Getters ───────────────────────────────────────────────────────────────

    /// Calls `getMe` — returns basic information about the bot.
    pub fn get_me(&self) -> GetMe {
        GetMe::new(self.clone())
    }
    /// Calls `getChat` — returns detailed information about a chat.
    pub fn get_chat(&self, chat_id: impl Into<rustigram_types::user::ChatId>) -> GetChat {
        GetChat::new(self.clone(), chat_id)
    }
    /// Calls `getChatAdministrators` — returns a list of all chat administrators.
    pub fn get_chat_administrators(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> GetChatAdministrators {
        GetChatAdministrators::new(self.clone(), chat_id)
    }
    /// Calls `getChatMemberCount` — returns the number of members in a chat.
    pub fn get_chat_member_count(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> GetChatMemberCount {
        GetChatMemberCount::new(self.clone(), chat_id)
    }
    /// Calls `getChatMember` — returns information about a specific chat member.
    pub fn get_chat_member(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> GetChatMember {
        GetChatMember::new(self.clone(), chat_id, user_id)
    }
    /// Calls `getFile` — returns file metadata and a download path.
    pub fn get_file(&self, file_id: impl Into<String>) -> GetFile {
        GetFile::new(self.clone(), file_id)
    }
    /// Calls `getUserProfilePhotos` — returns a user's profile pictures.
    pub fn get_user_profile_photos(&self, user_id: i64) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self.clone(), user_id)
    }
    /// Calls `getUserProfileAudios` — returns audios displayed on a user's profile (Bot API 9.4).
    pub fn get_user_profile_audios(&self, user_id: i64) -> GetUserProfileAudios {
        GetUserProfileAudios::new(self.clone(), user_id)
    }
    /// Calls `getUserPersonalChatMessages` — returns the last messages from a user's personal chat (Bot API 9.7).
    ///
    /// `limit` must be between 1 and 20.
    pub fn get_user_personal_chat_messages(
        &self,
        user_id: i64,
        limit: u32,
    ) -> GetUserPersonalChatMessages {
        GetUserPersonalChatMessages::new(self.clone(), user_id, limit)
    }

    // ── Sending ───────────────────────────────────────────────────────────────

    /// Calls `sendMessage` — sends a text message to a chat.
    pub fn send_message(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        text: impl Into<String>,
    ) -> SendMessage {
        SendMessage::new(self.clone(), chat_id, text)
    }
    /// Calls `forwardMessage` — forwards a message from one chat to another.
    pub fn forward_message(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        from_chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> ForwardMessage {
        ForwardMessage::new(self.clone(), chat_id, from_chat_id, message_id)
    }
    /// Calls `copyMessage` — copies a message without the forward header.
    pub fn copy_message(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        from_chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> CopyMessage {
        CopyMessage::new(self.clone(), chat_id, from_chat_id, message_id)
    }
    /// Calls `sendChatAction` — displays a typing or upload indicator.
    pub fn send_chat_action(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        action: ChatAction,
    ) -> SendChatAction {
        SendChatAction::new(self.clone(), chat_id, action)
    }
    /// Calls `sendPhoto` — sends a photo.
    pub fn send_photo(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        photo: rustigram_types::file::InputFile,
    ) -> SendPhoto {
        SendPhoto::new(self.clone(), chat_id, photo)
    }
    /// Calls `sendAudio` — sends an audio file treated as music.
    pub fn send_audio(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        audio: rustigram_types::file::InputFile,
    ) -> SendAudio {
        SendAudio::new(self.clone(), chat_id, audio)
    }
    /// Calls `sendDocument` — sends a general file.
    pub fn send_document(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        document: rustigram_types::file::InputFile,
    ) -> SendDocument {
        SendDocument::new(self.clone(), chat_id, document)
    }
    /// Calls `sendVideo` — sends a video file.
    pub fn send_video(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        video: rustigram_types::file::InputFile,
    ) -> SendVideo {
        SendVideo::new(self.clone(), chat_id, video)
    }
    /// Calls `sendAnimation` — sends a GIF or silent H.264 video.
    pub fn send_animation(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        animation: rustigram_types::file::InputFile,
    ) -> SendAnimation {
        SendAnimation::new(self.clone(), chat_id, animation)
    }
    /// Calls `sendVoice` — sends a voice note.
    pub fn send_voice(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        voice: rustigram_types::file::InputFile,
    ) -> SendVoice {
        SendVoice::new(self.clone(), chat_id, voice)
    }
    /// Calls `sendVideoNote` — sends a rounded-square video.
    pub fn send_video_note(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        video_note: rustigram_types::file::InputFile,
    ) -> SendVideoNote {
        SendVideoNote::new(self.clone(), chat_id, video_note)
    }
    /// Calls `sendSticker` — sends a sticker.
    pub fn send_sticker(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        sticker: rustigram_types::file::InputFile,
    ) -> SendSticker {
        SendSticker::new(self.clone(), chat_id, sticker)
    }
    /// Calls `sendLocation` — sends a geographic location, optionally live.
    pub fn send_location(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        latitude: f64,
        longitude: f64,
    ) -> SendLocation {
        SendLocation::new(self.clone(), chat_id, latitude, longitude)
    }
    /// Calls `sendContact` — sends a phone contact.
    pub fn send_contact(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SendContact {
        SendContact::new(self.clone(), chat_id, phone_number, first_name)
    }
    /// Calls `sendPoll` — sends a native poll or quiz.
    pub fn send_poll(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        question: impl Into<String>,
        options: Vec<rustigram_types::poll::InputPollOption>,
    ) -> SendPoll {
        SendPoll::new(self.clone(), chat_id, question, options)
    }
    /// Calls `sendDice` — sends an animated random emoji.
    pub fn send_dice(&self, chat_id: impl Into<rustigram_types::user::ChatId>) -> SendDice {
        SendDice::new(self.clone(), chat_id)
    }
    /// Calls `sendVenue` — sends information about a venue.
    pub fn send_venue(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenue {
        SendVenue::new(self.clone(), chat_id, latitude, longitude, title, address)
    }
    /// Calls `forwardMessages` — forwards 1–100 messages at once, preserving album grouping.
    pub fn forward_messages(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        from_chat_id: impl Into<rustigram_types::user::ChatId>,
        message_ids: Vec<i64>,
    ) -> ForwardMessages {
        ForwardMessages::new(self.clone(), chat_id, from_chat_id, message_ids)
    }
    /// Calls `copyMessages` — copies 1–100 messages without a forward link, preserving album grouping.
    pub fn copy_messages(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        from_chat_id: impl Into<rustigram_types::user::ChatId>,
        message_ids: Vec<i64>,
    ) -> CopyMessages {
        CopyMessages::new(self.clone(), chat_id, from_chat_id, message_ids)
    }
    /// Calls `sendMediaGroup` — sends 2–10 photos, videos, documents, or audios as an album.
    ///
    /// The `media` parameter accepts `Vec<serde_json::Value>` until the `InputMedia` enum
    /// is defined in Priority 4.
    pub fn send_media_group(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        media: Vec<serde_json::Value>,
    ) -> SendMediaGroup {
        SendMediaGroup::new(self.clone(), chat_id, media)
    }
    /// Calls `sendPaidMedia` — sends paid media requiring Telegram Stars to view.
    ///
    /// The `media` parameter accepts `Vec<serde_json::Value>` until the `InputPaidMedia` enum
    /// is defined in Priority 4.
    pub fn send_paid_media(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        star_count: u32,
        media: Vec<serde_json::Value>,
    ) -> SendPaidMedia {
        SendPaidMedia::new(self.clone(), chat_id, star_count, media)
    }
    /// Calls `sendGame` — sends an HTML5 game.
    pub fn send_game(&self, chat_id: i64, game_short_name: impl Into<String>) -> SendGame {
        SendGame::new(self.clone(), chat_id, game_short_name)
    }
    /// Calls `sendChecklist` — sends a checklist on behalf of a business account.
    pub fn send_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        checklist: rustigram_types::checklist::InputChecklist,
    ) -> SendChecklist {
        SendChecklist::new(self.clone(), business_connection_id, chat_id, checklist)
    }
    /// Calls `sendMessageDraft` — streams a partial message (Bot API 9.5+).
    pub fn send_message_draft(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        draft_id: i64,
        text: impl Into<String>,
    ) -> SendMessageDraft {
        SendMessageDraft::new(self.clone(), chat_id, draft_id, text)
    }
    /// Calls `deleteMessage` — deletes a message.
    pub fn delete_message(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> DeleteMessage {
        DeleteMessage::new(self.clone(), chat_id, message_id)
    }
    /// Calls `deleteMessages` — deletes up to 100 messages at once.
    pub fn delete_messages(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_ids: Vec<i64>,
    ) -> DeleteMessages {
        DeleteMessages::new(self.clone(), chat_id, message_ids)
    }
    /// Calls `stopPoll` — stops an open poll.
    pub fn stop_poll(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> StopPoll {
        StopPoll::new(self.clone(), chat_id, message_id)
    }
    /// Calls `answerCallbackQuery` — acknowledges a callback button press.
    pub fn answer_callback_query(
        &self,
        callback_query_id: impl Into<String>,
    ) -> AnswerCallbackQuery {
        AnswerCallbackQuery::new(self.clone(), callback_query_id)
    }

    // ── Editing ───────────────────────────────────────────────────────────────

    /// Calls `editMessageText` — edits the text of a sent message.
    pub fn edit_message_text(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
        text: impl Into<String>,
    ) -> EditMessageText {
        EditMessageText::in_chat(self.clone(), chat_id, message_id, text)
    }
    /// Calls `editMessageText` for an inline message sent via inline mode.
    pub fn edit_inline_message_text(
        &self,
        inline_message_id: impl Into<String>,
        text: impl Into<String>,
    ) -> EditMessageText {
        EditMessageText::inline(self.clone(), inline_message_id, text)
    }
    /// Calls `editMessageCaption` — edits the caption of a media message.
    pub fn edit_message_caption(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> EditMessageCaption {
        EditMessageCaption::in_chat(self.clone(), chat_id, message_id)
    }
    /// Calls `editMessageCaption` for an inline message sent via inline mode.
    pub fn edit_inline_message_caption(
        &self,
        inline_message_id: impl Into<String>,
    ) -> EditMessageCaption {
        EditMessageCaption::inline(self.clone(), inline_message_id)
    }
    /// Calls `editMessageMedia` — replaces the media content of a message.
    ///
    /// The `media` parameter accepts `serde_json::Value` until `InputMedia` is
    /// defined in Priority 4.
    pub fn edit_message_media(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
        media: serde_json::Value,
    ) -> EditMessageMedia {
        EditMessageMedia::in_chat(self.clone(), chat_id, message_id, media)
    }
    /// Calls `editMessageMedia` for an inline message sent via inline mode.
    pub fn edit_inline_message_media(
        &self,
        inline_message_id: impl Into<String>,
        media: serde_json::Value,
    ) -> EditMessageMedia {
        EditMessageMedia::inline(self.clone(), inline_message_id, media)
    }
    /// Calls `editMessageReplyMarkup` — replaces the inline keyboard of a message.
    pub fn edit_message_reply_markup(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::in_chat(self.clone(), chat_id, message_id)
    }
    /// Calls `editMessageReplyMarkup` for an inline message sent via inline mode.
    pub fn edit_inline_message_reply_markup(
        &self,
        inline_message_id: impl Into<String>,
    ) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::inline(self.clone(), inline_message_id)
    }
    /// Calls `editMessageChecklist` — edits a checklist on behalf of a business account.
    pub fn edit_message_checklist(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: rustigram_types::checklist::InputChecklist,
    ) -> EditMessageChecklist {
        EditMessageChecklist::new(
            self.clone(),
            business_connection_id,
            chat_id,
            message_id,
            checklist,
        )
    }
    /// Calls `approveSuggestedPost` — approves a suggested post in a direct messages chat.
    pub fn approve_suggested_post(&self, chat_id: i64, message_id: i64) -> ApproveSuggestedPost {
        ApproveSuggestedPost::new(self.clone(), chat_id, message_id)
    }
    /// Calls `declineSuggestedPost` — declines a suggested post in a direct messages chat.
    pub fn decline_suggested_post(&self, chat_id: i64, message_id: i64) -> DeclineSuggestedPost {
        DeclineSuggestedPost::new(self.clone(), chat_id, message_id)
    }
    /// Calls `editMessageLiveLocation` — updates the position of a live location.
    pub fn edit_message_live_location(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
        latitude: f64,
        longitude: f64,
    ) -> EditMessageLiveLocation {
        EditMessageLiveLocation::in_chat(self.clone(), chat_id, message_id, latitude, longitude)
    }
    /// Calls `editMessageLiveLocation` for an inline message sent via inline mode.
    pub fn edit_inline_message_live_location(
        &self,
        inline_message_id: impl Into<String>,
        latitude: f64,
        longitude: f64,
    ) -> EditMessageLiveLocation {
        EditMessageLiveLocation::inline(self.clone(), inline_message_id, latitude, longitude)
    }
    /// Calls `stopMessageLiveLocation` — stops a live location from updating.
    pub fn stop_message_live_location(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> StopMessageLiveLocation {
        StopMessageLiveLocation::in_chat(self.clone(), chat_id, message_id)
    }
    /// Calls `stopMessageLiveLocation` for an inline message sent via inline mode.
    pub fn stop_inline_message_live_location(
        &self,
        inline_message_id: impl Into<String>,
    ) -> StopMessageLiveLocation {
        StopMessageLiveLocation::inline(self.clone(), inline_message_id)
    }

    // ── Chat management ───────────────────────────────────────────────────────

    /// Calls `banChatMember` — bans a user from a chat.
    pub fn ban_chat_member(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> BanChatMember {
        BanChatMember::new(self.clone(), chat_id, user_id)
    }
    /// Calls `unbanChatMember` — lifts a ban from a user.
    pub fn unban_chat_member(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> UnbanChatMember {
        UnbanChatMember::new(self.clone(), chat_id, user_id)
    }
    /// Calls `restrictChatMember` — restricts what a user can do in a chat.
    pub fn restrict_chat_member(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
        permissions: rustigram_types::chat::ChatPermissions,
    ) -> RestrictChatMember {
        RestrictChatMember::new(self.clone(), chat_id, user_id, permissions)
    }
    /// Calls `promoteChatMember` — grants or revokes admin privileges.
    pub fn promote_chat_member(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> PromoteChatMember {
        PromoteChatMember::new(self.clone(), chat_id, user_id)
    }
    /// Calls `setChatAdministratorCustomTitle` — sets a custom title for an admin.
    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> SetChatAdministratorCustomTitle {
        SetChatAdministratorCustomTitle::new(self.clone(), chat_id, user_id, custom_title)
    }
    /// Calls `setChatMemberTag` — sets a tag for a regular member (Bot API 9.5).
    pub fn set_chat_member_tag(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> SetChatMemberTag {
        SetChatMemberTag::new(self.clone(), chat_id, user_id)
    }
    /// Calls `setChatPermissions` — sets default chat permissions for all members.
    pub fn set_chat_permissions(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        permissions: rustigram_types::chat::ChatPermissions,
    ) -> SetChatPermissions {
        SetChatPermissions::new(self.clone(), chat_id, permissions)
    }
    /// Calls `exportChatInviteLink` — generates a new primary invite link, revoking the old one.
    pub fn export_chat_invite_link(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> ExportChatInviteLink {
        ExportChatInviteLink::new(self.clone(), chat_id)
    }
    /// Calls `createChatInviteLink` — generates a new additional invite link.
    pub fn create_chat_invite_link(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> CreateChatInviteLink {
        CreateChatInviteLink::new(self.clone(), chat_id)
    }
    /// Calls `editChatInviteLink` — edits a non-primary invite link created by the bot.
    pub fn edit_chat_invite_link(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLink {
        EditChatInviteLink::new(self.clone(), chat_id, invite_link)
    }
    /// Calls `revokeChatInviteLink` — revokes an invite link created by the bot.
    pub fn revoke_chat_invite_link(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLink {
        RevokeChatInviteLink::new(self.clone(), chat_id, invite_link)
    }
    /// Calls `createChatSubscriptionInviteLink` — creates a subscription invite link for a channel.
    pub fn create_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        subscription_period: u32,
        subscription_price: u32,
    ) -> CreateChatSubscriptionInviteLink {
        CreateChatSubscriptionInviteLink::new(
            self.clone(),
            chat_id,
            subscription_period,
            subscription_price,
        )
    }
    /// Calls `editChatSubscriptionInviteLink` — edits a subscription invite link.
    pub fn edit_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatSubscriptionInviteLink {
        EditChatSubscriptionInviteLink::new(self.clone(), chat_id, invite_link)
    }
    /// Calls `approveChatJoinRequest` — approves a pending join request.
    pub fn approve_chat_join_request(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> ApproveChatJoinRequest {
        ApproveChatJoinRequest::new(self.clone(), chat_id, user_id)
    }
    /// Calls `declineChatJoinRequest` — declines a pending join request.
    pub fn decline_chat_join_request(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> DeclineChatJoinRequest {
        DeclineChatJoinRequest::new(self.clone(), chat_id, user_id)
    }
    /// Calls `banChatSenderChat` — bans a channel chat from sending in a supergroup or channel.
    pub fn ban_chat_sender_chat(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        sender_chat_id: i64,
    ) -> BanChatSenderChat {
        BanChatSenderChat::new(self.clone(), chat_id, sender_chat_id)
    }
    /// Calls `unbanChatSenderChat` — unbans a previously banned channel chat.
    pub fn unban_chat_sender_chat(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        sender_chat_id: i64,
    ) -> UnbanChatSenderChat {
        UnbanChatSenderChat::new(self.clone(), chat_id, sender_chat_id)
    }
    /// Calls `unpinAllChatMessages` — clears all pinned messages in a chat.
    pub fn unpin_all_chat_messages(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> UnpinAllChatMessages {
        UnpinAllChatMessages::new(self.clone(), chat_id)
    }
    /// Calls `setChatPhoto` — sets a new profile photo for the chat.
    pub fn set_chat_photo(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        photo: rustigram_types::file::InputFile,
    ) -> SetChatPhoto {
        SetChatPhoto::new(self.clone(), chat_id, photo)
    }
    /// Calls `deleteChatPhoto` — deletes the chat photo.
    pub fn delete_chat_photo(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> DeleteChatPhoto {
        DeleteChatPhoto::new(self.clone(), chat_id)
    }
    /// Calls `setChatTitle` — changes the title of a chat.
    pub fn set_chat_title(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        title: impl Into<String>,
    ) -> SetChatTitle {
        SetChatTitle::new(self.clone(), chat_id, title)
    }
    /// Calls `setChatDescription` — changes the description of a group, supergroup, or channel.
    pub fn set_chat_description(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> SetChatDescription {
        SetChatDescription::new(self.clone(), chat_id)
    }
    /// Calls `setChatStickerSet` — sets the sticker set for a supergroup.
    pub fn set_chat_sticker_set(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> SetChatStickerSet {
        SetChatStickerSet::new(self.clone(), chat_id, sticker_set_name)
    }
    /// Calls `deleteChatStickerSet` — removes the sticker set from a supergroup.
    pub fn delete_chat_sticker_set(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> DeleteChatStickerSet {
        DeleteChatStickerSet::new(self.clone(), chat_id)
    }
    /// Calls `leaveChat` — makes the bot leave a group, supergroup, or channel.
    pub fn leave_chat(&self, chat_id: impl Into<rustigram_types::user::ChatId>) -> LeaveChat {
        LeaveChat::new(self.clone(), chat_id)
    }
    /// Calls `getUserChatBoosts` — returns the boosts added to a chat by a user.
    pub fn get_user_chat_boosts(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        user_id: i64,
    ) -> GetUserChatBoosts {
        GetUserChatBoosts::new(self.clone(), chat_id, user_id)
    }
    /// Calls `pinChatMessage` — pins a message in a chat.
    pub fn pin_chat_message(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> PinChatMessage {
        PinChatMessage::new(self.clone(), chat_id, message_id)
    }
    /// Calls `unpinChatMessage` — unpins a message in a chat.
    pub fn unpin_chat_message(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> UnpinChatMessage {
        UnpinChatMessage::new(self.clone(), chat_id)
    }

    // ── Bot settings ──────────────────────────────────────────────────────────

    /// Calls `logOut` — logs the bot out of the cloud Bot API server.
    pub fn log_out(&self) -> LogOut {
        LogOut::new(self.clone())
    }
    /// Calls `close` — closes the bot instance before moving it to another server.
    pub fn close(&self) -> Close {
        Close::new(self.clone())
    }
    /// Calls `setMyCommands` — sets the bot's command list.
    pub fn set_my_commands(
        &self,
        commands: Vec<rustigram_types::user::BotCommand>,
    ) -> SetMyCommands {
        SetMyCommands::new(self.clone(), commands)
    }
    /// Calls `deleteMyCommands` — deletes the bot's command list for a given scope and language.
    pub fn delete_my_commands(&self) -> DeleteMyCommands {
        DeleteMyCommands::new(self.clone())
    }
    /// Calls `getMyCommands` — returns the bot's current command list.
    pub fn get_my_commands(&self) -> GetMyCommands {
        GetMyCommands::new(self.clone())
    }
    /// Calls `setMyName` — changes the bot's display name.
    pub fn set_my_name(&self) -> SetMyName {
        SetMyName::new(self.clone())
    }
    /// Calls `getMyName` — returns the bot's current display name.
    pub fn get_my_name(&self) -> GetMyName {
        GetMyName::new(self.clone())
    }
    /// Calls `setMyDescription` — changes the bot's profile description.
    pub fn set_my_description(&self) -> SetMyDescription {
        SetMyDescription::new(self.clone())
    }
    /// Calls `getMyDescription` — returns the bot's current profile description.
    pub fn get_my_description(&self) -> GetMyDescription {
        GetMyDescription::new(self.clone())
    }
    /// Calls `setMyShortDescription` — changes the bot's short description.
    pub fn set_my_short_description(&self) -> SetMyShortDescription {
        SetMyShortDescription::new(self.clone())
    }
    /// Calls `getMyShortDescription` — returns the bot's current short description.
    pub fn get_my_short_description(&self) -> GetMyShortDescription {
        GetMyShortDescription::new(self.clone())
    }
    /// Calls `setMyDefaultAdministratorRights` — sets the default admin rights suggested to users.
    pub fn set_my_default_administrator_rights(&self) -> SetMyDefaultAdministratorRights {
        SetMyDefaultAdministratorRights::new(self.clone())
    }
    /// Calls `getMyDefaultAdministratorRights` — returns the bot's current default admin rights.
    pub fn get_my_default_administrator_rights(&self) -> GetMyDefaultAdministratorRights {
        GetMyDefaultAdministratorRights::new(self.clone())
    }
    /// Calls `getChatMenuButton` — returns the current menu button for a private chat.
    pub fn get_chat_menu_button(&self) -> GetChatMenuButton {
        GetChatMenuButton::new(self.clone())
    }
    /// Calls `setChatMenuButton` — changes the bot's menu button in a private chat or globally.
    pub fn set_chat_menu_button(&self) -> SetChatMenuButton {
        SetChatMenuButton::new(self.clone())
    }
    /// Calls `setMyProfilePhoto` — changes the bot's profile photo (Bot API 9.4).
    ///
    /// Pass a pre-serialised `InputProfilePhoto` JSON string.
    pub fn set_my_profile_photo(&self, photo_json: impl Into<String>) -> SetMyProfilePhoto {
        SetMyProfilePhoto::new(self.clone(), photo_json.into())
    }
    /// Calls `removeMyProfilePhoto` — removes the bot's current profile photo (Bot API 9.4).
    pub fn remove_my_profile_photo(&self) -> RemoveMyProfilePhoto {
        RemoveMyProfilePhoto::new(self.clone())
    }
    /// Calls `getManagedBotToken` — returns the token of a managed bot (Bot API 9.6).
    pub fn get_managed_bot_token(&self, user_id: i64) -> GetManagedBotToken {
        GetManagedBotToken::new(self.clone(), user_id)
    }
    /// Calls `replaceManagedBotToken` — revokes and regenerates a managed bot's token (Bot API 9.6).
    pub fn replace_managed_bot_token(&self, user_id: i64) -> ReplaceManagedBotToken {
        ReplaceManagedBotToken::new(self.clone(), user_id)
    }
    /// Calls `getManagedBotAccessSettings` — returns the access settings of a managed bot (Bot API 9.7).
    pub fn get_managed_bot_access_settings(&self, user_id: i64) -> GetManagedBotAccessSettings {
        GetManagedBotAccessSettings::new(self.clone(), user_id)
    }
    /// Calls `setManagedBotAccessSettings` — changes the access settings of a managed bot (Bot API 9.7).
    pub fn set_managed_bot_access_settings(
        &self,
        user_id: i64,
        is_access_restricted: bool,
    ) -> SetManagedBotAccessSettings {
        SetManagedBotAccessSettings::new(self.clone(), user_id, is_access_restricted)
    }

    // ── Stories (business bots) ───────────────────────────────────────────────

    /// Calls `postStory` — posts a story on behalf of a managed business account.
    ///
    /// `content` is `serde_json::Value` until `InputStoryContent` is defined in Priority 4.
    /// `active_period` must be one of `21600`, `43200`, `86400`, or `172800` seconds.
    pub fn post_story(
        &self,
        business_connection_id: impl Into<String>,
        content: serde_json::Value,
        active_period: u32,
    ) -> PostStory {
        PostStory::new(self.clone(), business_connection_id, content, active_period)
    }
    /// Calls `repostStory` — reposts a story from one managed business account to another.
    ///
    /// `active_period` must be one of `21600`, `43200`, `86400`, or `172800` seconds.
    pub fn repost_story(
        &self,
        business_connection_id: impl Into<String>,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: u32,
    ) -> RepostStory {
        RepostStory::new(
            self.clone(),
            business_connection_id,
            from_chat_id,
            from_story_id,
            active_period,
        )
    }
    /// Calls `editStory` — edits a story posted by the bot on behalf of a business account.
    ///
    /// `content` is `serde_json::Value` until `InputStoryContent` is defined in Priority 4.
    pub fn edit_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
        content: serde_json::Value,
    ) -> EditStory {
        EditStory::new(self.clone(), business_connection_id, story_id, content)
    }
    /// Calls `deleteStory` — deletes a story posted by the bot on behalf of a business account.
    pub fn delete_story(
        &self,
        business_connection_id: impl Into<String>,
        story_id: i64,
    ) -> DeleteStory {
        DeleteStory::new(self.clone(), business_connection_id, story_id)
    }

    // ── Gifts ─────────────────────────────────────────────────────────────────

    /// Calls `getAvailableGifts` — returns all gifts the bot can send.
    pub fn get_available_gifts(&self) -> GetAvailableGifts {
        GetAvailableGifts::new(self.clone())
    }
    /// Calls `sendGift` — sends a gift to a user or channel chat.
    ///
    /// Chain `.user_id(id)` or `.chat_id(id)` to specify the recipient.
    pub fn send_gift(&self, gift_id: impl Into<String>) -> SendGift {
        SendGift::new(self.clone(), gift_id)
    }
    /// Calls `giftPremiumSubscription` — gifts a Telegram Premium subscription to a user.
    ///
    /// `month_count` must be `3`, `6`, or `12`.
    /// `star_count` must be `1000`, `1500`, or `2500` respectively.
    pub fn gift_premium_subscription(
        &self,
        user_id: i64,
        month_count: u32,
        star_count: u32,
    ) -> GiftPremiumSubscription {
        GiftPremiumSubscription::new(self.clone(), user_id, month_count, star_count)
    }
    /// Calls `getBusinessAccountGifts` — returns gifts received by a managed business account.
    pub fn get_business_account_gifts(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessAccountGifts {
        GetBusinessAccountGifts::new(self.clone(), business_connection_id)
    }
    /// Calls `getUserGifts` — returns gifts owned by a user.
    pub fn get_user_gifts(&self, user_id: i64) -> GetUserGifts {
        GetUserGifts::new(self.clone(), user_id)
    }
    /// Calls `getChatGifts` — returns gifts owned by a channel chat.
    pub fn get_chat_gifts(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> GetChatGifts {
        GetChatGifts::new(self.clone(), chat_id)
    }
    /// Calls `convertGiftToStars` — converts a business account gift to Telegram Stars.
    pub fn convert_gift_to_stars(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> ConvertGiftToStars {
        ConvertGiftToStars::new(self.clone(), business_connection_id, owned_gift_id)
    }
    /// Calls `upgradeGift` — upgrades a regular gift to a unique gift.
    pub fn upgrade_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> UpgradeGift {
        UpgradeGift::new(self.clone(), business_connection_id, owned_gift_id)
    }
    /// Calls `transferGift` — transfers a unique gift to another user.
    pub fn transfer_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        new_owner_chat_id: i64,
    ) -> TransferGift {
        TransferGift::new(
            self.clone(),
            business_connection_id,
            owned_gift_id,
            new_owner_chat_id,
        )
    }

    // ── Reactions ─────────────────────────────────────────────────────────────

    /// Calls `setMessageReaction` — sets a reaction on a message.
    pub fn set_message_reaction(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> SetMessageReaction {
        SetMessageReaction::new(self.clone(), chat_id, message_id)
    }
    /// Calls `deleteMessageReaction` — removes a specific reaction from a message (Bot API 9.7).
    pub fn delete_message_reaction(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> DeleteMessageReaction {
        DeleteMessageReaction::new(self.clone(), chat_id, message_id)
    }
    /// Calls `deleteAllMessageReactions` — removes all recent reactions by a given user or chat (Bot API 9.7).
    pub fn delete_all_message_reactions(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> DeleteAllMessageReactions {
        DeleteAllMessageReactions::new(self.clone(), chat_id)
    }

    // ── Inline mode ───────────────────────────────────────────────────────────

    /// Calls `answerInlineQuery` — sends up to 50 results for an inline query.
    pub fn answer_inline_query(
        &self,
        inline_query_id: impl Into<String>,
        results: Vec<rustigram_types::inline::InlineQueryResult>,
    ) -> AnswerInlineQuery {
        AnswerInlineQuery::new(self.clone(), inline_query_id, results)
    }
    /// Calls `answerWebAppQuery` — sets the result of a Web App interaction and sends it to the chat.
    pub fn answer_web_app_query(
        &self,
        web_app_query_id: impl Into<String>,
        result: rustigram_types::inline::InlineQueryResult,
    ) -> AnswerWebAppQuery {
        AnswerWebAppQuery::new(self.clone(), web_app_query_id, result)
    }
    /// Calls `answerGuestQuery` — replies to a received guest message (Bot API 9.7).
    pub fn answer_guest_query(
        &self,
        guest_query_id: impl Into<String>,
        result: rustigram_types::inline::InlineQueryResult,
    ) -> AnswerGuestQuery {
        AnswerGuestQuery::new(self.clone(), guest_query_id, result)
    }
    /// Calls `savePreparedInlineMessage` — stores a message sendable by a Mini App user.
    pub fn save_prepared_inline_message(
        &self,
        user_id: i64,
        result: rustigram_types::inline::InlineQueryResult,
    ) -> SavePreparedInlineMessage {
        SavePreparedInlineMessage::new(self.clone(), user_id, result)
    }

    // ── Mini App ──────────────────────────────────────────────────────────────

    /// Calls `savePreparedKeyboardButton` — stores a keyboard button for use in a Mini App (Bot API 9.6).
    ///
    /// The button must be of type `request_users`, `request_chat`, or `request_managed_bot`.
    pub fn save_prepared_keyboard_button(
        &self,
        user_id: i64,
        button: rustigram_types::keyboard::KeyboardButton,
    ) -> SavePreparedKeyboardButton {
        SavePreparedKeyboardButton::new(self.clone(), user_id, button)
    }
    /// Calls `setUserEmojiStatus` — changes a user's emoji status via a Mini App.
    pub fn set_user_emoji_status(&self, user_id: i64) -> SetUserEmojiStatus {
        SetUserEmojiStatus::new(self.clone(), user_id)
    }

    // ── Passport ──────────────────────────────────────────────────────────────

    /// Calls `setPassportDataErrors` — reports errors in Telegram Passport elements.
    ///
    /// Each error is a `serde_json::Value` — serialise from
    /// `rustigram_types::passport::PassportElementError` variants.
    pub fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<serde_json::Value>,
    ) -> SetPassportDataErrors {
        SetPassportDataErrors::new(self.clone(), user_id, errors)
    }

    // ── Games ─────────────────────────────────────────────────────────────────

    /// Calls `setGameScore` — sets a user's score in a game.
    ///
    /// Chain `.chat_message(chat_id, message_id)` or `.inline_message_id(id)` to target the message.
    pub fn set_game_score(&self, user_id: i64, score: u32) -> SetGameScore {
        SetGameScore::new(self.clone(), user_id, score)
    }
    /// Calls `getGameHighScores` — returns high scores for a game.
    ///
    /// Chain `.chat_message(chat_id, message_id)` or `.inline_message_id(id)` to target the message.
    pub fn get_game_high_scores(&self, user_id: i64) -> GetGameHighScores {
        GetGameHighScores::new(self.clone(), user_id)
    }

    // ── Payments ──────────────────────────────────────────────────────────────

    /// Calls `sendInvoice` — sends a payment invoice.
    pub fn send_invoice(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<rustigram_types::payments::LabeledPrice>,
    ) -> SendInvoice {
        SendInvoice::new(
            self.clone(),
            chat_id,
            title,
            description,
            payload,
            currency,
            prices,
        )
    }
    /// Calls `createInvoiceLink` — creates a shareable payment link.
    pub fn create_invoice_link(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<rustigram_types::payments::LabeledPrice>,
    ) -> CreateInvoiceLink {
        CreateInvoiceLink::new(self.clone(), title, description, payload, currency, prices)
    }
    /// Calls `answerShippingQuery` — responds to a shipping query from a user.
    ///
    /// Pass `ok = true` and provide `shipping_options`; or `ok = false` with an `error_message`.
    pub fn answer_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        ok: bool,
    ) -> AnswerShippingQuery {
        AnswerShippingQuery::new(self.clone(), shipping_query_id, ok)
    }
    /// Calls `answerPreCheckoutQuery` — confirms or rejects a pre-checkout query.
    ///
    /// Must be called within **10 seconds** of receiving the query.
    pub fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
        ok: bool,
    ) -> AnswerPreCheckoutQuery {
        AnswerPreCheckoutQuery::new(self.clone(), pre_checkout_query_id, ok)
    }
    /// Calls `refundStarPayment` — refunds a successful Telegram Stars payment.
    pub fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
    ) -> RefundStarPayment {
        RefundStarPayment::new(self.clone(), user_id, telegram_payment_charge_id)
    }
    /// Calls `editUserStarSubscription` — cancels or re-enables a Stars subscription.
    pub fn edit_user_star_subscription(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> EditUserStarSubscription {
        EditUserStarSubscription::new(
            self.clone(),
            user_id,
            telegram_payment_charge_id,
            is_canceled,
        )
    }
    /// Calls `getMyStarBalance` — returns the bot's Telegram Star balance.
    pub fn get_my_star_balance(&self) -> GetMyStarBalance {
        GetMyStarBalance::new(self.clone())
    }
    /// Calls `getStarTransactions` — returns the bot's Star transaction history.
    pub fn get_star_transactions(&self) -> GetStarTransactions {
        GetStarTransactions::new(self.clone())
    }

    // ── Stickers ──────────────────────────────────────────────────────────────

    /// Calls `getStickerSet` — returns a sticker set by name.
    pub fn get_sticker_set(&self, name: impl Into<String>) -> GetStickerSet {
        GetStickerSet::new(self.clone(), name)
    }
    /// Calls `getCustomEmojiStickers` — returns stickers for the given custom emoji IDs.
    pub fn get_custom_emoji_stickers(&self, ids: Vec<impl Into<String>>) -> GetCustomEmojiStickers {
        GetCustomEmojiStickers::new(self.clone(), ids)
    }
    /// Calls `uploadStickerFile` — uploads a sticker file for later use in a set.
    pub fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: rustigram_types::file::InputFile,
        format: rustigram_types::sticker::StickerFormat,
    ) -> UploadStickerFile {
        UploadStickerFile::new(self.clone(), user_id, sticker, format)
    }
    /// Calls `createNewStickerSet` — creates a new sticker set owned by a user.
    pub fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: Vec<rustigram_types::sticker::InputSticker>,
    ) -> CreateNewStickerSet {
        CreateNewStickerSet::new(self.clone(), user_id, name, title, stickers)
    }
    /// Calls `addStickerToSet` — adds a new sticker to an existing set.
    pub fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        sticker: rustigram_types::sticker::InputSticker,
    ) -> AddStickerToSet {
        AddStickerToSet::new(self.clone(), user_id, name, sticker)
    }
    /// Calls `setStickerPositionInSet` — moves a sticker to a new position in its set.
    pub fn set_sticker_position_in_set(
        &self,
        sticker: impl Into<String>,
        position: u32,
    ) -> SetStickerPositionInSet {
        SetStickerPositionInSet::new(self.clone(), sticker, position)
    }
    /// Calls `deleteStickerFromSet` — removes a sticker from its set.
    pub fn delete_sticker_from_set(&self, sticker: impl Into<String>) -> DeleteStickerFromSet {
        DeleteStickerFromSet::new(self.clone(), sticker)
    }
    /// Calls `setStickerEmojiList` — updates the emoji list for a sticker.
    pub fn set_sticker_emoji_list(
        &self,
        sticker: impl Into<String>,
        emoji_list: Vec<impl Into<String>>,
    ) -> SetStickerEmojiList {
        SetStickerEmojiList::new(self.clone(), sticker, emoji_list)
    }
    /// Calls `setStickerKeywords` — updates the search keywords for a sticker.
    pub fn set_sticker_keywords(&self, sticker: impl Into<String>) -> SetStickerKeywords {
        SetStickerKeywords::new(self.clone(), sticker)
    }
    /// Calls `setStickerMaskPosition` — updates the mask position for a mask sticker.
    pub fn set_sticker_mask_position(&self, sticker: impl Into<String>) -> SetStickerMaskPosition {
        SetStickerMaskPosition::new(self.clone(), sticker)
    }
    /// Calls `setStickerSetTitle` — renames a sticker set.
    pub fn set_sticker_set_title(
        &self,
        name: impl Into<String>,
        title: impl Into<String>,
    ) -> SetStickerSetTitle {
        SetStickerSetTitle::new(self.clone(), name, title)
    }
    /// Calls `deleteStickerSet` — deletes a sticker set created by the bot.
    pub fn delete_sticker_set(&self, name: impl Into<String>) -> DeleteStickerSet {
        DeleteStickerSet::new(self.clone(), name)
    }
    /// Calls `replaceStickerInSet` — replaces an existing sticker in a set with a new one.
    pub fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: rustigram_types::sticker::InputSticker,
    ) -> ReplaceStickerInSet {
        ReplaceStickerInSet::new(self.clone(), user_id, name, old_sticker, sticker)
    }
    /// Calls `setStickerSetThumbnail` — sets the thumbnail of a regular or mask sticker set.
    ///
    /// `format` must be `"static"`, `"animated"`, or `"video"`.
    /// Chain `.thumbnail(file)` to set the thumbnail; omit to drop it.
    pub fn set_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
        user_id: i64,
        format: impl Into<String>,
    ) -> SetStickerSetThumbnail {
        SetStickerSetThumbnail::new(self.clone(), name, user_id, format)
    }
    /// Calls `setCustomEmojiStickerSetThumbnail` — sets the thumbnail of a custom emoji sticker set.
    ///
    /// Chain `.custom_emoji_id(id)` to set the thumbnail emoji; omit to use the first sticker.
    pub fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: impl Into<String>,
    ) -> SetCustomEmojiStickerSetThumbnail {
        SetCustomEmojiStickerSetThumbnail::new(self.clone(), name)
    }
    /// Calls `getForumTopicIconStickers` — returns all available forum topic icon stickers.
    pub fn get_forum_topic_icon_stickers(&self) -> GetForumTopicIconStickers {
        GetForumTopicIconStickers::new(self.clone())
    }

    // ── Forum topics ──────────────────────────────────────────────────────────

    /// Calls `createForumTopic` — creates a new topic in a forum supergroup.
    pub fn create_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        name: impl Into<String>,
    ) -> CreateForumTopic {
        CreateForumTopic::new(self.clone(), chat_id, name)
    }
    /// Calls `editForumTopic` — edits the name or icon of a forum topic.
    pub fn edit_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        thread_id: i64,
    ) -> EditForumTopic {
        EditForumTopic::new(self.clone(), chat_id, thread_id)
    }
    /// Calls `closeForumTopic` — closes an open forum topic.
    pub fn close_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        thread_id: i64,
    ) -> CloseForumTopic {
        CloseForumTopic::new(self.clone(), chat_id, thread_id)
    }
    /// Calls `reopenForumTopic` — reopens a closed forum topic.
    pub fn reopen_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        thread_id: i64,
    ) -> ReopenForumTopic {
        ReopenForumTopic::new(self.clone(), chat_id, thread_id)
    }
    /// Calls `deleteForumTopic` — deletes a forum topic and all its messages.
    pub fn delete_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        thread_id: i64,
    ) -> DeleteForumTopic {
        DeleteForumTopic::new(self.clone(), chat_id, thread_id)
    }
    /// Calls `editGeneralForumTopic` — renames the General topic.
    pub fn edit_general_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        name: impl Into<String>,
    ) -> EditGeneralForumTopic {
        EditGeneralForumTopic::new(self.clone(), chat_id, name)
    }
    /// Calls `closeGeneralForumTopic` — closes the General topic.
    pub fn close_general_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> CloseGeneralForumTopic {
        CloseGeneralForumTopic::new(self.clone(), chat_id)
    }
    /// Calls `reopenGeneralForumTopic` — reopens the General topic.
    pub fn reopen_general_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> ReopenGeneralForumTopic {
        ReopenGeneralForumTopic::new(self.clone(), chat_id)
    }
    /// Calls `hideGeneralForumTopic` — hides the General topic from the topic list.
    pub fn hide_general_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> HideGeneralForumTopic {
        HideGeneralForumTopic::new(self.clone(), chat_id)
    }
    /// Calls `unhideGeneralForumTopic` — makes the General topic visible again.
    pub fn unhide_general_forum_topic(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> UnhideGeneralForumTopic {
        UnhideGeneralForumTopic::new(self.clone(), chat_id)
    }
    /// Calls `unpinAllGeneralForumTopicMessages` — clears all pinned messages in the General forum topic.
    pub fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> UnpinAllGeneralForumTopicMessages {
        UnpinAllGeneralForumTopicMessages::new(self.clone(), chat_id)
    }

    // ── Verification ──────────────────────────────────────────────────────────

    /// Calls `verifyUser` — verifies a user on behalf of the organisation.
    pub fn verify_user(&self, user_id: i64) -> VerifyUser {
        VerifyUser::new(self.clone(), user_id)
    }
    /// Calls `verifyChat` — verifies a chat on behalf of the organisation.
    pub fn verify_chat(&self, chat_id: impl Into<rustigram_types::user::ChatId>) -> VerifyChat {
        VerifyChat::new(self.clone(), chat_id)
    }
    /// Calls `removeUserVerification` — removes verification from a user.
    pub fn remove_user_verification(&self, user_id: i64) -> RemoveUserVerification {
        RemoveUserVerification::new(self.clone(), user_id)
    }
    /// Calls `removeChatVerification` — removes verification from a chat.
    pub fn remove_chat_verification(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
    ) -> RemoveChatVerification {
        RemoveChatVerification::new(self.clone(), chat_id)
    }

    // ── Business account ──────────────────────────────────────────────────────

    /// Calls `getBusinessConnection` — returns business connection information.
    pub fn get_business_connection(&self, id: impl Into<String>) -> GetBusinessConnection {
        GetBusinessConnection::new(self.clone(), id)
    }
    /// Calls `readBusinessMessage` — marks a business account message as read.
    pub fn read_business_message(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        message_id: i64,
    ) -> ReadBusinessMessage {
        ReadBusinessMessage::new(self.clone(), business_connection_id, chat_id, message_id)
    }
    /// Calls `deleteBusinessMessages` — deletes messages from a business account.
    pub fn delete_business_messages(
        &self,
        business_connection_id: impl Into<String>,
        message_ids: Vec<i64>,
    ) -> DeleteBusinessMessages {
        DeleteBusinessMessages::new(self.clone(), business_connection_id, message_ids)
    }
    /// Calls `setBusinessAccountName` — sets the name of a managed business account.
    pub fn set_business_account_name(
        &self,
        business_connection_id: impl Into<String>,
        first_name: impl Into<String>,
        last_name: Option<String>,
    ) -> SetBusinessAccountName {
        SetBusinessAccountName::new(
            self.clone(),
            business_connection_id,
            first_name.into(),
            last_name,
        )
    }
    /// Calls `setBusinessAccountUsername` — sets the username of a managed business account.
    pub fn set_business_account_username(
        &self,
        business_connection_id: impl Into<String>,
        username: Option<String>,
    ) -> SetBusinessAccountUsername {
        SetBusinessAccountUsername::new(self.clone(), business_connection_id, username)
    }
    /// Calls `setBusinessAccountBio` — sets the bio of a managed business account.
    pub fn set_business_account_bio(
        &self,
        business_connection_id: impl Into<String>,
        bio: Option<String>,
    ) -> SetBusinessAccountBio {
        SetBusinessAccountBio::new(self.clone(), business_connection_id, bio)
    }
    /// Calls `getBusinessAccountStarBalance` — returns a business account's Star balance.
    pub fn get_business_account_star_balance(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessAccountStarBalance {
        GetBusinessAccountStarBalance::new(self.clone(), business_connection_id)
    }
    /// Calls `transferBusinessAccountStars` — transfers Stars from a business account to the bot.
    pub fn transfer_business_account_stars(
        &self,
        business_connection_id: impl Into<String>,
        star_count: u64,
    ) -> TransferBusinessAccountStars {
        TransferBusinessAccountStars::new(self.clone(), business_connection_id, star_count)
    }
    /// Calls `unpinAllForumTopicMessages` — clears all pinned messages in a forum topic.
    pub fn unpin_all_forum_topic_messages(
        &self,
        chat_id: impl Into<rustigram_types::user::ChatId>,
        thread_id: i64,
    ) -> UnpinAllForumTopicMessages {
        UnpinAllForumTopicMessages::new(self.clone(), chat_id, thread_id)
    }

    /// Calls `setBusinessAccountProfilePhoto` — sets the profile photo of a managed business account.
    ///
    /// Pass `photo` as `serde_json::to_value(&input_profile_photo)`.
    pub fn set_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
        photo: serde_json::Value,
    ) -> SetBusinessAccountProfilePhoto {
        SetBusinessAccountProfilePhoto::new(self.clone(), business_connection_id, photo)
    }

    /// Calls `removeBusinessAccountProfilePhoto` — removes the profile photo of a managed business account.
    pub fn remove_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
    ) -> RemoveBusinessAccountProfilePhoto {
        RemoveBusinessAccountProfilePhoto::new(self.clone(), business_connection_id)
    }

    /// Calls `setBusinessAccountGiftSettings` — changes gift privacy settings for a managed business account.
    pub fn set_business_account_gift_settings(
        &self,
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: rustigram_types::payments::AcceptedGiftTypes,
    ) -> SetBusinessAccountGiftSettings {
        SetBusinessAccountGiftSettings::new(
            self.clone(),
            business_connection_id,
            show_gift_button,
            accepted_gift_types,
        )
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

#[allow(dead_code)]
/// Converts an `InputFile::Bytes` into a multipart `Part` for file uploads.
pub(crate) fn input_file_to_part(file: rustigram_types::file::InputFile) -> Option<(String, Part)> {
    use rustigram_types::file::InputFile;
    match file {
        InputFile::Bytes {
            filename,
            data,
            mime_type,
        } => {
            let part = Part::bytes(data)
                .file_name(filename.clone())
                .mime_str(&mime_type)
                .ok()?;
            Some((filename, part))
        }
        _ => None,
    }
}

fn validate_token(token: &str) -> Result<()> {
    let colon = token.find(':').ok_or(Error::InvalidToken)?;
    let id_part = &token[..colon];
    if id_part.is_empty() || !id_part.chars().all(|c| c.is_ascii_digit()) {
        return Err(Error::InvalidToken);
    }
    if token[colon + 1..].is_empty() {
        return Err(Error::InvalidToken);
    }
    Ok(())
}
