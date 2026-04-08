use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::payments::StarAmount;
use rustigram_types::update::BusinessConnection;
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── getBusinessConnection ────────────────────────────────────────────────────

#[derive(Serialize)]
struct GetBusinessConnectionParams {
    business_connection_id: String,
}

/// Builder for the [`getBusinessConnection`](https://core.telegram.org/bots/api#getbusinessconnection) method.
pub struct GetBusinessConnection {
    client: BotClient,
    params: GetBusinessConnectionParams,
}

impl GetBusinessConnection {
    pub(crate) fn new(client: BotClient, id: impl Into<String>) -> Self {
        Self {
            client,
            params: GetBusinessConnectionParams {
                business_connection_id: id.into(),
            },
        }
    }
}

impl IntoFuture for GetBusinessConnection {
    type Output = Result<BusinessConnection>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getBusinessConnection", &self.params)
                .await
        })
    }
}

// ─── readBusinessMessage ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct ReadBusinessMessageParams {
    business_connection_id: String,
    chat_id: ChatId,
    message_id: i64,
}

/// Builder for the [`readBusinessMessage`](https://core.telegram.org/bots/api#readbusinessmessage) method.
pub struct ReadBusinessMessage {
    client: BotClient,
    params: ReadBusinessMessageParams,
}

impl ReadBusinessMessage {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        chat_id: impl Into<ChatId>,
        message_id: i64,
    ) -> Self {
        Self {
            client,
            params: ReadBusinessMessageParams {
                business_connection_id: business_connection_id.into(),
                chat_id: chat_id.into(),
                message_id,
            },
        }
    }
}

impl IntoFuture for ReadBusinessMessage {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("readBusinessMessage", &self.params)
                .await
        })
    }
}

// ─── deleteBusinessMessages ───────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteBusinessMessagesParams {
    business_connection_id: String,
    message_ids: Vec<i64>,
}

/// Builder for the [`deleteBusinessMessages`](https://core.telegram.org/bots/api#deletebusinessmessages) method.
pub struct DeleteBusinessMessages {
    client: BotClient,
    params: DeleteBusinessMessagesParams,
}

impl DeleteBusinessMessages {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        message_ids: Vec<i64>,
    ) -> Self {
        Self {
            client,
            params: DeleteBusinessMessagesParams {
                business_connection_id: business_connection_id.into(),
                message_ids,
            },
        }
    }
}

impl IntoFuture for DeleteBusinessMessages {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("deleteBusinessMessages", &self.params)
                .await
        })
    }
}

// ─── setBusinessAccountName ───────────────────────────────────────────────────

#[derive(Serialize)]
struct SetBusinessAccountNameParams {
    business_connection_id: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
}

/// Builder for the [`setBusinessAccountName`](https://core.telegram.org/bots/api#setbusinessaccountname) method.
pub struct SetBusinessAccountName {
    client: BotClient,
    params: SetBusinessAccountNameParams,
}

impl SetBusinessAccountName {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        first_name: String,
        last_name: Option<String>,
    ) -> Self {
        Self {
            client,
            params: SetBusinessAccountNameParams {
                business_connection_id: business_connection_id.into(),
                first_name,
                last_name,
            },
        }
    }
}

impl IntoFuture for SetBusinessAccountName {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setBusinessAccountName", &self.params)
                .await
        })
    }
}

// ─── setBusinessAccountUsername ───────────────────────────────────────────────

#[derive(Serialize)]
struct SetBusinessAccountUsernameParams {
    business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
}

/// Builder for the [`setBusinessAccountUsername`](https://core.telegram.org/bots/api#setbusinessaccountusername) method.
pub struct SetBusinessAccountUsername {
    client: BotClient,
    params: SetBusinessAccountUsernameParams,
}

impl SetBusinessAccountUsername {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        username: Option<String>,
    ) -> Self {
        Self {
            client,
            params: SetBusinessAccountUsernameParams {
                business_connection_id: business_connection_id.into(),
                username,
            },
        }
    }
}

impl IntoFuture for SetBusinessAccountUsername {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setBusinessAccountUsername", &self.params)
                .await
        })
    }
}

// ─── setBusinessAccountBio ────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetBusinessAccountBioParams {
    business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<String>,
}

/// Builder for the [`setBusinessAccountBio`](https://core.telegram.org/bots/api#setbusinessaccountbio) method.
pub struct SetBusinessAccountBio {
    client: BotClient,
    params: SetBusinessAccountBioParams,
}

impl SetBusinessAccountBio {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        bio: Option<String>,
    ) -> Self {
        Self {
            client,
            params: SetBusinessAccountBioParams {
                business_connection_id: business_connection_id.into(),
                bio,
            },
        }
    }
}

impl IntoFuture for SetBusinessAccountBio {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setBusinessAccountBio", &self.params)
                .await
        })
    }
}

// ─── getBusinessAccountStarBalance ───────────────────────────────────────────

#[derive(Serialize)]
struct BizConnectionIdParams {
    business_connection_id: String,
}

/// Builder for the [`getBusinessAccountStarBalance`](https://core.telegram.org/bots/api#getbusinessaccountstarbalance) method.
pub struct GetBusinessAccountStarBalance {
    client: BotClient,
    params: BizConnectionIdParams,
}

impl GetBusinessAccountStarBalance {
    pub(crate) fn new(client: BotClient, id: impl Into<String>) -> Self {
        Self {
            client,
            params: BizConnectionIdParams {
                business_connection_id: id.into(),
            },
        }
    }
}

impl IntoFuture for GetBusinessAccountStarBalance {
    type Output = Result<StarAmount>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getBusinessAccountStarBalance", &self.params)
                .await
        })
    }
}

// ─── transferBusinessAccountStars ─────────────────────────────────────────────

#[derive(Serialize)]
struct TransferBusinessAccountStarsParams {
    business_connection_id: String,
    star_count: u64,
}

/// Builder for the [`transferBusinessAccountStars`](https://core.telegram.org/bots/api#transferbusinessaccountstars) method.
pub struct TransferBusinessAccountStars {
    client: BotClient,
    params: TransferBusinessAccountStarsParams,
}

impl TransferBusinessAccountStars {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        star_count: u64,
    ) -> Self {
        Self {
            client,
            params: TransferBusinessAccountStarsParams {
                business_connection_id: business_connection_id.into(),
                star_count,
            },
        }
    }
}

impl IntoFuture for TransferBusinessAccountStars {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("transferBusinessAccountStars", &self.params)
                .await
        })
    }
}

// ─── setBusinessAccountProfilePhoto ──────────────────────────────────────────

#[derive(Serialize)]
struct SetBusinessAccountProfilePhotoParams {
    business_connection_id: String,
    /// The new profile photo.
    ///
    /// Uses `serde_json::Value` — serialise from
    /// `rustigram_types::file::InputProfilePhoto` with `serde_json::to_value`.
    photo: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,
}

/// Builder for the [`setBusinessAccountProfilePhoto`](https://core.telegram.org/bots/api#setbusinessaccountprofilephoto) method.
///
/// Changes the profile photo of a managed business account.
/// Requires the `can_edit_profile_photo` business bot right.
///
/// Pass the `photo` as `serde_json::to_value(&input_profile_photo)`.
pub struct SetBusinessAccountProfilePhoto {
    client: BotClient,
    params: SetBusinessAccountProfilePhotoParams,
}

impl SetBusinessAccountProfilePhoto {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        photo: serde_json::Value,
    ) -> Self {
        Self {
            client,
            params: SetBusinessAccountProfilePhotoParams {
                business_connection_id: business_connection_id.into(),
                photo,
                is_public: None,
            },
        }
    }
    /// Pass `true` to set the public photo, visible even if the main photo is
    /// hidden by the account's privacy settings. An account can have only one public photo.
    pub fn is_public(mut self, v: bool) -> Self {
        self.params.is_public = Some(v);
        self
    }
}

impl IntoFuture for SetBusinessAccountProfilePhoto {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setBusinessAccountProfilePhoto", &self.params)
                .await
        })
    }
}

// ─── removeBusinessAccountProfilePhoto ───────────────────────────────────────

#[derive(Serialize)]
struct RemoveBusinessAccountProfilePhotoParams {
    business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,
}

/// Builder for the [`removeBusinessAccountProfilePhoto`](https://core.telegram.org/bots/api#removebusinessaccountprofilephoto) method.
///
/// Removes the current profile photo of a managed business account.
/// Requires the `can_edit_profile_photo` business bot right.
pub struct RemoveBusinessAccountProfilePhoto {
    client: BotClient,
    params: RemoveBusinessAccountProfilePhotoParams,
}

impl RemoveBusinessAccountProfilePhoto {
    pub(crate) fn new(client: BotClient, business_connection_id: impl Into<String>) -> Self {
        Self {
            client,
            params: RemoveBusinessAccountProfilePhotoParams {
                business_connection_id: business_connection_id.into(),
                is_public: None,
            },
        }
    }
    /// Pass `true` to remove the public photo instead of the main photo.
    pub fn is_public(mut self, v: bool) -> Self {
        self.params.is_public = Some(v);
        self
    }
}

impl IntoFuture for RemoveBusinessAccountProfilePhoto {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("removeBusinessAccountProfilePhoto", &self.params)
                .await
        })
    }
}

// ─── setBusinessAccountGiftSettings ──────────────────────────────────────────

#[derive(Serialize)]
struct SetBusinessAccountGiftSettingsParams {
    business_connection_id: String,
    show_gift_button: bool,
    accepted_gift_types: rustigram_types::payments::AcceptedGiftTypes,
}

/// Builder for the [`setBusinessAccountGiftSettings`](https://core.telegram.org/bots/api#setbusinessaccountgiftsettings) method.
///
/// Changes the gift privacy settings of a managed business account.
/// Requires the `can_change_gift_settings` business bot right.
pub struct SetBusinessAccountGiftSettings {
    client: BotClient,
    params: SetBusinessAccountGiftSettingsParams,
}

impl SetBusinessAccountGiftSettings {
    pub(crate) fn new(
        client: BotClient,
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: rustigram_types::payments::AcceptedGiftTypes,
    ) -> Self {
        Self {
            client,
            params: SetBusinessAccountGiftSettingsParams {
                business_connection_id: business_connection_id.into(),
                show_gift_button,
                accepted_gift_types,
            },
        }
    }
}

impl IntoFuture for SetBusinessAccountGiftSettings {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("setBusinessAccountGiftSettings", &self.params)
                .await
        })
    }
}
