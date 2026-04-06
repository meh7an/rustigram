use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── verifyUser ───────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct VerifyUserParams {
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_description: Option<String>,
}

/// Builder for the [`verifyUser`](https://core.telegram.org/bots/api#verifyuser) method.
pub struct VerifyUser {
    client: BotClient,
    params: VerifyUserParams,
}

impl VerifyUser {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: VerifyUserParams {
                user_id,
                custom_description: None,
            },
        }
    }
    /// Sets a custom description shown alongside the verification badge (up to 70 characters).
    pub fn custom_description(mut self, d: impl Into<String>) -> Self {
        self.params.custom_description = Some(d.into());
        self
    }
}

impl IntoFuture for VerifyUser {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("verifyUser", &self.params).await })
    }
}

// ─── verifyChat ───────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct VerifyChatParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_description: Option<String>,
}

/// Builder for the [`verifyChat`](https://core.telegram.org/bots/api#verifychat) method.
pub struct VerifyChat {
    client: BotClient,
    params: VerifyChatParams,
}

impl VerifyChat {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: VerifyChatParams {
                chat_id: chat_id.into(),
                custom_description: None,
            },
        }
    }
    /// Sets a custom description shown alongside the verification badge. (up to 70 characters)
    pub fn custom_description(mut self, d: impl Into<String>) -> Self {
        self.params.custom_description = Some(d.into());
        self
    }
}

impl IntoFuture for VerifyChat {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("verifyChat", &self.params).await })
    }
}

// ─── removeUserVerification ───────────────────────────────────────────────────

#[derive(Serialize)]
struct RemoveUserVerificationParams {
    user_id: i64,
}

/// Builder for the [`removeUserVerification`](https://core.telegram.org/bots/api#removeuserverification) method.
pub struct RemoveUserVerification {
    client: BotClient,
    params: RemoveUserVerificationParams,
}

impl RemoveUserVerification {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: RemoveUserVerificationParams { user_id },
        }
    }
}

impl IntoFuture for RemoveUserVerification {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("removeUserVerification", &self.params)
                .await
        })
    }
}

// ─── removeChatVerification ───────────────────────────────────────────────────

#[derive(Serialize)]
struct RemoveChatVerificationParams {
    chat_id: ChatId,
}

/// Builder for the [`removeChatVerification`](https://core.telegram.org/bots/api#removechatverification) method.
pub struct RemoveChatVerification {
    client: BotClient,
    params: RemoveChatVerificationParams,
}

impl RemoveChatVerification {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: RemoveChatVerificationParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

impl IntoFuture for RemoveChatVerification {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("removeChatVerification", &self.params)
                .await
        })
    }
}
