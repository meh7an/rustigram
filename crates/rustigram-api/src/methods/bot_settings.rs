use crate::client::BotClient;
use rustigram_types::keyboard::MenuButton;
use rustigram_types::user::{BotCommand, BotCommandScope, ChatId};
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// setMyCommands
#[derive(Serialize)]
struct SetMyCommandsParams {
    commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`setMyCommands`](https://core.telegram.org/bots/api#setmycommands) method.
pub struct SetMyCommands {
    client: BotClient,
    params: SetMyCommandsParams,
}
impl SetMyCommands {
    pub(crate) fn new(client: BotClient, commands: Vec<BotCommand>) -> Self {
        Self {
            client,
            params: SetMyCommandsParams {
                commands,
                scope: None,
                language_code: None,
            },
        }
    }
    /// Restricts these commands to a specific scope (chat type or individual chat).
    pub fn scope(mut self, s: BotCommandScope) -> Self {
        self.params.scope = Some(s);
        self
    }
    /// Sets the language code for localised command lists (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}
impl IntoFuture for SetMyCommands {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("setMyCommands", &self.params).await })
    }
}

// getMyCommands
#[derive(Serialize, Default)]
struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`getMyCommands`](https://core.telegram.org/bots/api#getmycommands) method.
pub struct GetMyCommands {
    client: BotClient,
    params: GetMyCommandsParams,
}
impl GetMyCommands {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Restricts the list of retrieved commands to a specific scope (chat type or individual chat).
    pub fn scope(mut self, s: BotCommandScope) -> Self {
        self.params.scope = Some(s);
        self
    }
    /// The language code for localised command lists (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}
impl IntoFuture for GetMyCommands {
    type Output = crate::error::Result<Vec<BotCommand>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("getMyCommands", &self.params).await })
    }
}

// setMyName
#[derive(Serialize, Default)]
struct SetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`setMyName`](https://core.telegram.org/bots/api#setmyname) method.
pub struct SetMyName {
    client: BotClient,
    params: SetMyNameParams,
}
impl SetMyName {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Sets the new bot name (up to 64 characters).
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.params.name = Some(n.into());
        self
    }
    /// Sets the language code for localised bot names (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}
impl IntoFuture for SetMyName {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("setMyName", &self.params).await })
    }
}

// setMyDescription
#[derive(Serialize, Default)]
struct SetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`setMyDescription`](https://core.telegram.org/bots/api#setmydescription) method.
pub struct SetMyDescription {
    client: BotClient,
    params: SetMyDescriptionParams,
}
impl SetMyDescription {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Sets the new bot description shown on the profile page (up to 512 characters).
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.params.description = Some(d.into());
        self
    }
    /// Sets the language code for localised bot descriptions (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}
impl IntoFuture for SetMyDescription {
    type Output = crate::error::Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setMyDescription", &self.params)
                .await
        })
    }
}

// getChatMenuButton
#[derive(Serialize, Default)]
struct GetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
}

/// Builder for the [`getChatMenuButton`](https://core.telegram.org/bots/api#getchatmenubutton) method.
pub struct GetChatMenuButton {
    client: BotClient,
    params: GetChatMenuButtonParams,
}
impl GetChatMenuButton {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Restricts the menu button query to a specific private chat.
    pub fn chat_id(mut self, id: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(id.into());
        self
    }
}
impl IntoFuture for GetChatMenuButton {
    type Output = crate::error::Result<MenuButton>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getChatMenuButton", &self.params)
                .await
        })
    }
}

// getManagedBotToken
#[derive(Serialize)]
struct GetManagedBotTokenParams {
    user_id: i64,
}

/// Builder for the [`getManagedBotToken`](https://core.telegram.org/bots/api#getmanagedbottoken) method (Bot API 9.6).
pub struct GetManagedBotToken {
    client: BotClient,
    params: GetManagedBotTokenParams,
}
impl GetManagedBotToken {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: GetManagedBotTokenParams { user_id },
        }
    }
}
impl IntoFuture for GetManagedBotToken {
    type Output = crate::error::Result<String>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getManagedBotToken", &self.params)
                .await
        })
    }
}
