use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::message::{MessageEntity, ParseMode};
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── Helper macro ─────────────────────────────────────────────────────────────

/// Generates an `IntoFuture` impl that calls `BotClient::post_json`.
macro_rules! impl_into_future {
    ($builder:ident, $return_ty:ty, $method:literal) => {
        impl IntoFuture for $builder {
            type Output = Result<$return_ty>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.client.post_json($method, &self.params).await })
            }
        }
    };
}

// ─── postStory ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PostStoryParams {
    business_connection_id: String,
    /// Content of the story.
    ///
    /// Uses `serde_json::Value` until `InputStoryContent` is defined in Priority 4.
    /// Construct with `serde_json::json!({"type":"photo","photo":"<file_id>"})` or
    /// `serde_json::to_value(&your_input_story_content)`.
    content: serde_json::Value,
    /// Period in seconds after which the story moves to the archive.
    ///
    /// Must be one of `21600` (6h), `43200` (12h), `86400` (24h), or `172800` (48h).
    active_period: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<MessageEntity>>,
    /// Clickable areas to show on the story.
    ///
    /// Uses `serde_json::Value` until `StoryArea` is defined in Priority 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    areas: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_to_chat_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
}

/// Builder for the [`postStory`](https://core.telegram.org/bots/api#poststory) method.
///
/// Posts a story on behalf of a managed business account.
/// Requires the `can_manage_stories` business bot right.
///
/// `content` accepts `serde_json::Value` until `InputStoryContent` is defined in Priority 4.
/// `active_period` must be one of `21600`, `43200`, `86400`, or `172800` seconds.
pub struct PostStory {
    client: BotClient,
    params: PostStoryParams,
}

impl PostStory {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        content: serde_json::Value,
        active_period: u32,
    ) -> Self {
        Self {
            client,
            params: PostStoryParams {
                business_connection_id: business_connection_id.into(),
                content,
                active_period,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                areas: None,
                post_to_chat_page: None,
                protect_content: None,
            },
        }
    }
    /// Sets the story caption (0–2048 characters after entities parsing).
    pub fn caption(mut self, c: impl Into<String>) -> Self {
        self.params.caption = Some(c.into());
        self
    }
    /// Sets the caption parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.params.parse_mode = Some(m);
        self
    }
    /// Sets special entities in the caption; alternative to `parse_mode`.
    pub fn caption_entities(mut self, e: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(e);
        self
    }
    /// Sets clickable areas on the story.
    ///
    /// Each item is a `serde_json::Value` representing a `StoryArea` until Priority 4.
    pub fn areas(mut self, a: Vec<serde_json::Value>) -> Self {
        self.params.areas = Some(a);
        self
    }
    /// Pass `true` to keep the story accessible after it expires.
    pub fn post_to_chat_page(mut self, v: bool) -> Self {
        self.params.post_to_chat_page = Some(v);
        self
    }
    /// Pass `true` to protect the story content from forwarding and screenshotting.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl_into_future!(PostStory, serde_json::Value, "postStory");

// ─── repostStory ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RepostStoryParams {
    business_connection_id: String,
    from_chat_id: i64,
    from_story_id: i64,
    /// Period in seconds after which the story moves to the archive.
    ///
    /// Must be one of `21600` (6h), `43200` (12h), `86400` (24h), or `172800` (48h).
    active_period: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_to_chat_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
}

/// Builder for the [`repostStory`](https://core.telegram.org/bots/api#repoststory) method.
///
/// Reposts a story from one managed business account to another.
/// Both accounts must be managed by the same bot, and the story must have been
/// posted by the bot. Requires the `can_manage_stories` business bot right on both accounts.
///
/// `active_period` must be one of `21600`, `43200`, `86400`, or `172800` seconds.
pub struct RepostStory {
    client: BotClient,
    params: RepostStoryParams,
}

impl RepostStory {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: u32,
    ) -> Self {
        Self {
            client,
            params: RepostStoryParams {
                business_connection_id: business_connection_id.into(),
                from_chat_id,
                from_story_id,
                active_period,
                post_to_chat_page: None,
                protect_content: None,
            },
        }
    }
    /// Pass `true` to keep the story accessible after it expires.
    pub fn post_to_chat_page(mut self, v: bool) -> Self {
        self.params.post_to_chat_page = Some(v);
        self
    }
    /// Pass `true` to protect the story content from forwarding and screenshotting.
    pub fn protect_content(mut self, v: bool) -> Self {
        self.params.protect_content = Some(v);
        self
    }
}

impl_into_future!(RepostStory, serde_json::Value, "repostStory");

// ─── editStory ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditStoryParams {
    business_connection_id: String,
    story_id: i64,
    /// New content of the story.
    ///
    /// Uses `serde_json::Value` until `InputStoryContent` is defined in Priority 4.
    content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<MessageEntity>>,
    /// Clickable areas to show on the story.
    ///
    /// Uses `serde_json::Value` until `StoryArea` is defined in Priority 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    areas: Option<Vec<serde_json::Value>>,
}

/// Builder for the [`editStory`](https://core.telegram.org/bots/api#editstory) method.
///
/// Edits a story previously posted by the bot on behalf of a managed business account.
/// Requires the `can_manage_stories` business bot right.
///
/// `content` accepts `serde_json::Value` until `InputStoryContent` is defined in Priority 4.
pub struct EditStory {
    client: BotClient,
    params: EditStoryParams,
}

impl EditStory {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        story_id: i64,
        content: serde_json::Value,
    ) -> Self {
        Self {
            client,
            params: EditStoryParams {
                business_connection_id: business_connection_id.into(),
                story_id,
                content,
                caption: None,
                parse_mode: None,
                caption_entities: None,
                areas: None,
            },
        }
    }
    /// Sets the story caption (0–2048 characters after entities parsing).
    pub fn caption(mut self, c: impl Into<String>) -> Self {
        self.params.caption = Some(c.into());
        self
    }
    /// Sets the caption parse mode (`MarkdownV2`, `HTML`, or `Markdown`).
    pub fn parse_mode(mut self, m: ParseMode) -> Self {
        self.params.parse_mode = Some(m);
        self
    }
    /// Sets special entities in the caption; alternative to `parse_mode`.
    pub fn caption_entities(mut self, e: Vec<MessageEntity>) -> Self {
        self.params.caption_entities = Some(e);
        self
    }
    /// Sets clickable areas on the story.
    ///
    /// Each item is a `serde_json::Value` representing a `StoryArea` until Priority 4.
    pub fn areas(mut self, a: Vec<serde_json::Value>) -> Self {
        self.params.areas = Some(a);
        self
    }
}

impl_into_future!(EditStory, serde_json::Value, "editStory");

// ─── deleteStory ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteStoryParams {
    business_connection_id: String,
    story_id: i64,
}

/// Builder for the [`deleteStory`](https://core.telegram.org/bots/api#deletestory) method.
///
/// Deletes a story previously posted by the bot on behalf of a managed business account.
/// Requires the `can_manage_stories` business bot right.
pub struct DeleteStory {
    client: BotClient,
    params: DeleteStoryParams,
}

impl DeleteStory {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        story_id: i64,
    ) -> Self {
        Self {
            client,
            params: DeleteStoryParams {
                business_connection_id: business_connection_id.into(),
                story_id,
            },
        }
    }
}

impl_into_future!(DeleteStory, bool, "deleteStory");
