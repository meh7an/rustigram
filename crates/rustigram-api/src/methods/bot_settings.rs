use crate::client::BotClient;
use crate::error::Result;
use reqwest::multipart::{Form, Part};
use rustigram_types::keyboard::MenuButton;
use rustigram_types::user::{
    BotCommand, BotCommandScope, BotDescription, BotName, BotShortDescription,
    ChatAdministratorRights, ChatId,
};
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

// ─── setMyCommands ────────────────────────────────────────────────────────────

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

impl_into_future!(SetMyCommands, bool, "setMyCommands");

// ─── deleteMyCommands ─────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct DeleteMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`deleteMyCommands`](https://core.telegram.org/bots/api#deletemycommands) method.
///
/// Deletes the bot's command list for the given scope and language.
/// After deletion, higher-level commands will be shown to affected users.
pub struct DeleteMyCommands {
    client: BotClient,
    params: DeleteMyCommandsParams,
}

impl DeleteMyCommands {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Restricts deletion to a specific scope.
    pub fn scope(mut self, s: BotCommandScope) -> Self {
        self.params.scope = Some(s);
        self
    }
    /// Restricts deletion to a specific language code (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}

impl_into_future!(DeleteMyCommands, bool, "deleteMyCommands");

// ─── getMyCommands ────────────────────────────────────────────────────────────

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

impl_into_future!(GetMyCommands, Vec<BotCommand>, "getMyCommands");

// ─── setMyName ────────────────────────────────────────────────────────────────

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

impl_into_future!(SetMyName, bool, "setMyName");

// ─── getMyName ────────────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct GetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`getMyName`](https://core.telegram.org/bots/api#getmyname) method.
pub struct GetMyName {
    client: BotClient,
    params: GetMyNameParams,
}

impl GetMyName {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// The language code for the localised bot name to retrieve (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}

impl_into_future!(GetMyName, BotName, "getMyName");

// ─── setMyDescription ─────────────────────────────────────────────────────────

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

impl_into_future!(SetMyDescription, bool, "setMyDescription");

// ─── getMyDescription ─────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct GetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`getMyDescription`](https://core.telegram.org/bots/api#getmydescription) method.
pub struct GetMyDescription {
    client: BotClient,
    params: GetMyDescriptionParams,
}

impl GetMyDescription {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// The language code for the localised bot description to retrieve (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}

impl_into_future!(GetMyDescription, BotDescription, "getMyDescription");

// ─── setMyShortDescription ────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct SetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`setMyShortDescription`](https://core.telegram.org/bots/api#setmyshortdescription) method.
///
/// The short description is shown on the bot's profile page and sent with
/// sharing links. Up to 120 characters; omit to remove the localised value.
pub struct SetMyShortDescription {
    client: BotClient,
    params: SetMyShortDescriptionParams,
}

impl SetMyShortDescription {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Sets the short description (0–120 characters). Omit to remove the dedicated value.
    pub fn short_description(mut self, d: impl Into<String>) -> Self {
        self.params.short_description = Some(d.into());
        self
    }
    /// Sets the language code for the localised short description (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}

impl_into_future!(SetMyShortDescription, bool, "setMyShortDescription");

// ─── getMyShortDescription ────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct GetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

/// Builder for the [`getMyShortDescription`](https://core.telegram.org/bots/api#getmyshortdescription) method.
pub struct GetMyShortDescription {
    client: BotClient,
    params: GetMyShortDescriptionParams,
}

impl GetMyShortDescription {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// The language code for the localised short description to retrieve (IETF tag, e.g. `"en"`).
    pub fn language_code(mut self, lc: impl Into<String>) -> Self {
        self.params.language_code = Some(lc.into());
        self
    }
}

impl_into_future!(
    GetMyShortDescription,
    BotShortDescription,
    "getMyShortDescription"
);

// ─── setMyDefaultAdministratorRights ─────────────────────────────────────────

#[derive(Serialize, Default)]
struct SetMyDefaultAdministratorRightsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}

/// Builder for the [`setMyDefaultAdministratorRights`](https://core.telegram.org/bots/api#setmydefaultadministratorrights) method.
///
/// Sets the default administrator rights suggested to users when the bot is
/// added as an administrator. Pass `None` rights to clear the defaults.
pub struct SetMyDefaultAdministratorRights {
    client: BotClient,
    params: SetMyDefaultAdministratorRightsParams,
}

impl SetMyDefaultAdministratorRights {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Sets the new default administrator rights. Omit to clear the current defaults.
    pub fn rights(mut self, r: ChatAdministratorRights) -> Self {
        self.params.rights = Some(r);
        self
    }
    /// Pass `true` to change defaults for channels; otherwise changes group/supergroup defaults.
    pub fn for_channels(mut self, v: bool) -> Self {
        self.params.for_channels = Some(v);
        self
    }
}

impl_into_future!(
    SetMyDefaultAdministratorRights,
    bool,
    "setMyDefaultAdministratorRights"
);

// ─── getMyDefaultAdministratorRights ─────────────────────────────────────────

#[derive(Serialize, Default)]
struct GetMyDefaultAdministratorRightsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}

/// Builder for the [`getMyDefaultAdministratorRights`](https://core.telegram.org/bots/api#getmydefaultadministratorrights) method.
pub struct GetMyDefaultAdministratorRights {
    client: BotClient,
    params: GetMyDefaultAdministratorRightsParams,
}

impl GetMyDefaultAdministratorRights {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Pass `true` to get default administrator rights for channels;
    /// otherwise returns defaults for groups and supergroups.
    pub fn for_channels(mut self, v: bool) -> Self {
        self.params.for_channels = Some(v);
        self
    }
}

impl_into_future!(
    GetMyDefaultAdministratorRights,
    ChatAdministratorRights,
    "getMyDefaultAdministratorRights"
);

// ─── getChatMenuButton ────────────────────────────────────────────────────────

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

impl_into_future!(GetChatMenuButton, MenuButton, "getChatMenuButton");

// ─── setChatMenuButton ────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct SetChatMenuButtonParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_button: Option<MenuButton>,
}

/// Builder for the [`setChatMenuButton`](https://core.telegram.org/bots/api#setchatmenubutton) method.
///
/// Changes the bot's menu button in a private chat, or the default menu button.
/// Omit `chat_id` to change the default; omit `menu_button` to reset to `MenuButtonDefault`.
pub struct SetChatMenuButton {
    client: BotClient,
    params: SetChatMenuButtonParams,
}

impl SetChatMenuButton {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: Default::default(),
        }
    }
    /// Targets a specific private chat. Omit to change the default menu button.
    pub fn chat_id(mut self, id: i64) -> Self {
        self.params.chat_id = Some(id);
        self
    }
    /// Sets the new menu button. Omit to reset to `MenuButtonDefault`.
    pub fn menu_button(mut self, btn: MenuButton) -> Self {
        self.params.menu_button = Some(btn);
        self
    }
}

impl_into_future!(SetChatMenuButton, bool, "setChatMenuButton");

// ─── logOut ───────────────────────────────────────────────────────────────────

/// Builder for the [`logOut`](https://core.telegram.org/bots/api#logout) method.
///
/// Logs out from the cloud Bot API server. Must be called before running the
/// bot locally. After a successful call the bot cannot log back in to the cloud
/// server for 10 minutes.
pub struct LogOut {
    client: BotClient,
}

impl LogOut {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}

impl IntoFuture for LogOut {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("logOut", &serde_json::json!({}))
                .await
        })
    }
}

// ─── close ────────────────────────────────────────────────────────────────────

/// Builder for the [`close`](https://core.telegram.org/bots/api#close) method.
///
/// Closes the bot instance before moving it to another local server. Delete
/// the webhook before calling this to prevent the bot from restarting.
/// Returns error 429 in the first 10 minutes after launch.
pub struct Close {
    client: BotClient,
}

impl Close {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}

impl IntoFuture for Close {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("close", &serde_json::json!({})).await })
    }
}

// ─── setMyProfilePhoto ────────────────────────────────────────────────────────

/// Builder for the [`setMyProfilePhoto`](https://core.telegram.org/bots/api#setmyprofilephoto) method (Bot API 9.4).
///
/// Changes the profile photo of the bot. The photo must be uploaded via
/// multipart/form-data as an `InputProfilePhoto`.
pub struct SetMyProfilePhoto {
    client: BotClient,
    /// The serialised `InputProfilePhoto` JSON sent as the `photo` field.
    photo_json: String,
}

impl SetMyProfilePhoto {
    /// Creates a new builder from a pre-serialised `InputProfilePhoto` value.
    ///
    /// Pass the result of `serde_json::to_string(&input_profile_photo)`.
    pub(crate) fn new(client: BotClient, photo_json: String) -> Self {
        Self { client, photo_json }
    }
}

impl IntoFuture for SetMyProfilePhoto {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let part = Part::text(self.photo_json)
                .mime_str("application/json")
                .map_err(|e| crate::error::Error::Decode(e.to_string()))?;
            let form = Form::new().part("photo", part);
            self.client.post_multipart("setMyProfilePhoto", form).await
        })
    }
}

// ─── removeMyProfilePhoto ─────────────────────────────────────────────────────

/// Builder for the [`removeMyProfilePhoto`](https://core.telegram.org/bots/api#removemyprofilephoto) method (Bot API 9.4).
///
/// Removes the current profile photo of the bot. Requires no parameters.
pub struct RemoveMyProfilePhoto {
    client: BotClient,
}

impl RemoveMyProfilePhoto {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}

impl IntoFuture for RemoveMyProfilePhoto {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("removeMyProfilePhoto", &serde_json::json!({}))
                .await
        })
    }
}

// ─── replaceManagedBotToken ───────────────────────────────────────────────────

#[derive(Serialize)]
struct ReplaceManagedBotTokenParams {
    user_id: i64,
}

/// Builder for the [`replaceManagedBotToken`](https://core.telegram.org/bots/api#replacemanagedbottoken) method (Bot API 9.6).
///
/// Revokes the current token of a managed bot and generates a new one.
/// Returns the new token as a `String`.
pub struct ReplaceManagedBotToken {
    client: BotClient,
    params: ReplaceManagedBotTokenParams,
}

impl ReplaceManagedBotToken {
    pub(crate) fn new(client: BotClient, user_id: i64) -> Self {
        Self {
            client,
            params: ReplaceManagedBotTokenParams { user_id },
        }
    }
}

impl_into_future!(ReplaceManagedBotToken, String, "replaceManagedBotToken");

// ─── getManagedBotToken ───────────────────────────────────────────────────────

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

impl_into_future!(GetManagedBotToken, String, "getManagedBotToken");
