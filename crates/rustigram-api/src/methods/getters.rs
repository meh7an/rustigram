use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::chat::ChatFullInfo;
use rustigram_types::chat_member::ChatMember;
use rustigram_types::file::File;
use rustigram_types::user::{ChatId, User, UserProfilePhotos};
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

macro_rules! simple_getter {
    ($(#[$doc:meta])* $name:ident, $params_ty:ty, $return_ty:ty, $method:literal) => {
        $(#[$doc])*
        pub struct $name {
            client: BotClient,
            params: $params_ty,
        }
        impl IntoFuture for $name {
            type Output = Result<$return_ty>;
            type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.client.post_json($method, &self.params).await })
            }
        }
    };
}

/// Builder for the [`getMe`](https://core.telegram.org/bots/api#getme) method.
pub struct GetMe {
    client: BotClient,
}
impl GetMe {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}
impl IntoFuture for GetMe {
    type Output = Result<User>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("getMe", &serde_json::json!({})).await })
    }
}

#[derive(Serialize)]
struct GetChatParams {
    chat_id: ChatId,
}
simple_getter!(
    /// Builder for the [`getChat`](https://core.telegram.org/bots/api#getchat) method.
    GetChat,
    GetChatParams,
    ChatFullInfo,
    "getChat"
);
impl GetChat {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: GetChatParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

#[derive(Serialize)]
struct GetChatAdministratorsParams {
    chat_id: ChatId,
}
simple_getter!(
    /// Builder for the [`getChatAdministrators`](https://core.telegram.org/bots/api#getchatadministrators) method.
    GetChatAdministrators,
    GetChatAdministratorsParams,
    Vec<ChatMember>,
    "getChatAdministrators"
);
impl GetChatAdministrators {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: GetChatAdministratorsParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

#[derive(Serialize)]
struct GetChatMemberCountParams {
    chat_id: ChatId,
}
simple_getter!(
    /// Builder for the [`getChatMemberCount`](https://core.telegram.org/bots/api#getchatmembercount) method.
    GetChatMemberCount,
    GetChatMemberCountParams,
    u32,
    "getChatMemberCount"
);
impl GetChatMemberCount {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: GetChatMemberCountParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

#[derive(Serialize)]
struct GetChatMemberParams {
    chat_id: ChatId,
    user_id: i64,
}
simple_getter!(
    /// Builder for the [`getChatMember`](https://core.telegram.org/bots/api#getchatmember) method.
    GetChatMember,
    GetChatMemberParams,
    ChatMember,
    "getChatMember"
);
impl GetChatMember {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: GetChatMemberParams {
                chat_id: chat_id.into(),
                user_id,
            },
        }
    }
}

#[derive(Serialize)]
struct GetFileParams {
    file_id: String,
}
simple_getter!(
    /// Builder for the [`getFile`](https://core.telegram.org/bots/api#getfile) method.
    GetFile, GetFileParams, File, "getFile");
impl GetFile {
    pub(crate) fn new(client: BotClient, file_id: impl Into<String>) -> Self {
        Self {
            client,
            params: GetFileParams {
                file_id: file_id.into(),
            },
        }
    }
}

#[derive(Serialize)]
struct GetUserProfilePhotosParams {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

/// Builder for the [`getUserProfilePhotos`](https://core.telegram.org/bots/api#getuserprofilephotos) method.
pub struct GetUserProfilePhotos {
    client: BotClient,
    params: GetUserProfilePhotosParams,
}
impl GetUserProfilePhotos {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: GetUserProfilePhotosParams {
                user_id,
                offset: None,
                limit: None,
            },
        }
    }
    /// Skips the first N profile photos.
    pub fn offset(mut self, v: u32) -> Self {
        self.params.offset = Some(v);
        self
    }
    /// Limits the number of photos returned (1–100).
    pub fn limit(mut self, v: u8) -> Self {
        self.params.limit = Some(v);
        self
    }
}
impl IntoFuture for GetUserProfilePhotos {
    type Output = Result<UserProfilePhotos>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getUserProfilePhotos", &self.params)
                .await
        })
    }
}

// ─── getUserProfileAudios ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct GetUserProfileAudiosParams {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

/// Builder for the [`getUserProfileAudios`](https://core.telegram.org/bots/api#getuserprofileaudios) method (Bot API 9.4).
///
/// Returns the list of audios displayed on a user's profile.
pub struct GetUserProfileAudios {
    client: BotClient,
    params: GetUserProfileAudiosParams,
}

impl GetUserProfileAudios {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: GetUserProfileAudiosParams {
                user_id,
                offset: None,
                limit: None,
            },
        }
    }
    /// Skips the first N profile audios.
    pub fn offset(mut self, v: u32) -> Self {
        self.params.offset = Some(v);
        self
    }
    /// Limits the number of audios returned (1–100, default 100).
    pub fn limit(mut self, v: u8) -> Self {
        self.params.limit = Some(v);
        self
    }
}

impl IntoFuture for GetUserProfileAudios {
    type Output = Result<rustigram_types::user::UserProfileAudios>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getUserProfileAudios", &self.params)
                .await
        })
    }
}
