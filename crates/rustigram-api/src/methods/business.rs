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
