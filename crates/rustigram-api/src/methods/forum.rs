use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::forum::ForumTopic;
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── createForumTopic ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct CreateForumTopicParams {
    chat_id: ChatId,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}

/// Builder for the [`createForumTopic`](https://core.telegram.org/bots/api#createforumtopic) method.
pub struct CreateForumTopic {
    client: BotClient,
    params: CreateForumTopicParams,
}

impl CreateForumTopic {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: CreateForumTopicParams {
                chat_id: chat_id.into(),
                name: name.into(),
                icon_color: None,
                icon_custom_emoji_id: None,
            },
        }
    }
    /// Sets the colour of the topic icon. One of: `0x6FB9F0`, `0xFFD67E`,
    /// `0xCB86DB`, `0x8EEE98`, `0xFF93B2`, `0xFB6F5F`.
    pub fn icon_color(mut self, color: u32) -> Self {
        self.params.icon_color = Some(color);
        self
    }
    /// Sets a custom emoji as the topic icon. Pass an empty string to remove it.
    pub fn icon_custom_emoji_id(mut self, id: impl Into<String>) -> Self {
        self.params.icon_custom_emoji_id = Some(id.into());
        self
    }
}

impl IntoFuture for CreateForumTopic {
    type Output = Result<ForumTopic>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("createForumTopic", &self.params)
                .await
        })
    }
}

// ─── editForumTopic ───────────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditForumTopicParams {
    chat_id: ChatId,
    message_thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}

/// Builder for the [`editForumTopic`](https://core.telegram.org/bots/api#editforumtopic) method.
pub struct EditForumTopic {
    client: BotClient,
    params: EditForumTopicParams,
}

impl EditForumTopic {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        message_thread_id: i64,
    ) -> Self {
        Self {
            client,
            params: EditForumTopicParams {
                chat_id: chat_id.into(),
                message_thread_id,
                name: None,
                icon_custom_emoji_id: None,
            },
        }
    }
    /// Sets the new name for the forum topic (1–128 characters).
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.params.name = Some(n.into());
        self
    }
    /// Sets a new custom emoji for the topic icon. Pass an empty string to remove it.
    pub fn icon_custom_emoji_id(mut self, id: impl Into<String>) -> Self {
        self.params.icon_custom_emoji_id = Some(id.into());
        self
    }
}

impl IntoFuture for EditForumTopic {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("editForumTopic", &self.params).await })
    }
}

// ─── Thread-level forum actions (close/reopen/delete/unpin) ──────────────────

#[derive(Serialize)]
struct ForumThreadParams {
    chat_id: ChatId,
    message_thread_id: i64,
}

#[allow(dead_code)]
macro_rules! forum_thread_action {
    ($(#[$doc:meta])* $name:ident, $method:literal) => {
        $(#[$doc])*
        pub struct $name {
            client: BotClient,
            params: ForumThreadParams,
        }

        impl $name {
            /// Creates a builder for the method.
            pub fn new(
                client: BotClient,
                chat_id: impl Into<ChatId>,
                message_thread_id: i64,
            ) -> Self {
                Self {
                    client,
                    params: ForumThreadParams {
                        chat_id: chat_id.into(),
                        message_thread_id,
                    },
                }
            }
        }

        impl IntoFuture for $name {
            type Output = Result<bool>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.client.post_json($method, &self.params).await })
            }
        }
    };
}

forum_thread_action!(
    /// Builder for the [`closeForumTopic`](https://core.telegram.org/bots/api#closeforumtopic) method.
    CloseForumTopic, "closeForumTopic");
forum_thread_action!(
    /// Builder for the [`reopenForumTopic`](https://core.telegram.org/bots/api#reopenforumtopic) method.
    ReopenForumTopic, "reopenForumTopic");
forum_thread_action!(
    /// Builder for the [`deleteForumTopic`](https://core.telegram.org/bots/api#deleteforumtopic) method.
    DeleteForumTopic, "deleteForumTopic");
forum_thread_action!(
    /// Builder for the [`unpinAllForumTopicMessages`](https://core.telegram.org/bots/api#unpinallforumtopicmessages) method.
    UnpinAllForumTopicMessages, "unpinAllForumTopicMessages");

// ─── General forum topic (chat-level) ─────────────────────────────────────────

#[derive(Serialize)]
struct ChatOnlyParams {
    chat_id: ChatId,
}

macro_rules! chat_only_action {
    ($(#[$doc:meta])* $name:ident, $method:literal) => {
        $(#[$doc])*
        pub struct $name {
            client: BotClient,
            params: ChatOnlyParams,
        }
        impl $name {
            // Creates a builder for the method.
            pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
                Self {
                    client,
                    params: ChatOnlyParams {
                        chat_id: chat_id.into(),
                    },
                }
            }
        }
        impl IntoFuture for $name {
            type Output = Result<bool>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.client.post_json($method, &self.params).await })
            }
        }
    };
}

chat_only_action!(
    /// Builder for the [`closeGeneralForumTopic`](https://core.telegram.org/bots/api#closegeneralforumtopic) method.
    CloseGeneralForumTopic, "closeGeneralForumTopic");
chat_only_action!(
    /// Builder for the [`reopenGeneralForumTopic`](https://core.telegram.org/bots/api#reopengeneralforumtopic) method.
    ReopenGeneralForumTopic, "reopenGeneralForumTopic");
chat_only_action!(
    /// Builder for the [`hideGeneralForumTopic`](https://core.telegram.org/bots/api#hidegeneralforumtopic) method.
    HideGeneralForumTopic, "hideGeneralForumTopic");
chat_only_action!(
    /// Builder for the [`unhideGeneralForumTopic`](https://core.telegram.org/bots/api#unhidegeneralforumtopic) method.
    UnhideGeneralForumTopic, "unhideGeneralForumTopic");

// ─── editGeneralForumTopic ────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditGeneralForumTopicParams {
    chat_id: ChatId,
    name: String,
}

/// Builder for the [`editGeneralForumTopic`](https://core.telegram.org/bots/api#editgeneralforumtopic) method.
pub struct EditGeneralForumTopic {
    client: BotClient,
    params: EditGeneralForumTopicParams,
}

impl EditGeneralForumTopic {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: EditGeneralForumTopicParams {
                chat_id: chat_id.into(),
                name: name.into(),
            },
        }
    }
}

impl IntoFuture for EditGeneralForumTopic {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("editGeneralForumTopic", &self.params)
                .await
        })
    }
}

chat_only_action!(
    /// Builder for the [`unpinAllGeneralForumTopicMessages`](https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages) method.
    ///
    /// Clears all pinned messages in the General forum topic.
    /// Requires the `can_pin_messages` administrator right in the supergroup.
    UnpinAllGeneralForumTopicMessages, "unpinAllGeneralForumTopicMessages");
