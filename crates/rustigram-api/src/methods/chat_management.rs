use crate::client::BotClient;
use crate::error::Result;
use rustigram_types::chat::{ChatInviteLink, ChatPermissions};
use rustigram_types::user::ChatId;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

// ─── banChatMember ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct BanChatMemberParams {
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_messages: Option<bool>,
}

/// Builder for the [`banChatMember`](https://core.telegram.org/bots/api#banchatmember) method.
pub struct BanChatMember {
    client: BotClient,
    params: BanChatMemberParams,
}

impl BanChatMember {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: BanChatMemberParams {
                chat_id: chat_id.into(),
                user_id,
                until_date: None,
                revoke_messages: None,
            },
        }
    }
    /// Bans the user until this Unix timestamp. Omit or set to 0 for a permanent ban.
    pub fn until_date(mut self, ts: i64) -> Self {
        self.params.until_date = Some(ts);
        self
    }
    /// Deletes all messages from this user in the chat on ban.
    pub fn revoke_messages(mut self, v: bool) -> Self {
        self.params.revoke_messages = Some(v);
        self
    }
}

impl IntoFuture for BanChatMember {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("banChatMember", &self.params).await })
    }
}

// ─── unbanChatMember ──────────────────────────────────────────────────────────

#[derive(Serialize)]
struct UnbanChatMemberParams {
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_if_banned: Option<bool>,
}

/// Builder for the [`unbanChatMember`](https://core.telegram.org/bots/api#unbanchatmember) method.
pub struct UnbanChatMember {
    client: BotClient,
    params: UnbanChatMemberParams,
}

impl UnbanChatMember {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: UnbanChatMemberParams {
                chat_id: chat_id.into(),
                user_id,
                only_if_banned: None,
            },
        }
    }
    /// Only unbans the user if they are currently banned (ignores non-banned users).
    pub fn only_if_banned(mut self, v: bool) -> Self {
        self.params.only_if_banned = Some(v);
        self
    }
}

impl IntoFuture for UnbanChatMember {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("unbanChatMember", &self.params).await })
    }
}

// ─── restrictChatMember ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct RestrictChatMemberParams {
    chat_id: ChatId,
    user_id: i64,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
}

/// Builder for the [`restrictChatMember`](https://core.telegram.org/bots/api#restrictchatmember) method.
pub struct RestrictChatMember {
    client: BotClient,
    params: RestrictChatMemberParams,
}

impl RestrictChatMember {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            client,
            params: RestrictChatMemberParams {
                chat_id: chat_id.into(),
                user_id,
                permissions,
                use_independent_chat_permissions: None,
                until_date: None,
            },
        }
    }
    /// Restricts the user until this Unix timestamp. Omit or set to 0 for a permanent restriction.
    pub fn until_date(mut self, ts: i64) -> Self {
        self.params.until_date = Some(ts);
        self
    }
    /// Sets whether to apply permissions independently (supergroups only).
    pub fn use_independent_chat_permissions(mut self, v: bool) -> Self {
        self.params.use_independent_chat_permissions = Some(v);
        self
    }
}

impl IntoFuture for RestrictChatMember {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("restrictChatMember", &self.params)
                .await
        })
    }
}

// ─── promoteChatMember ────────────────────────────────────────────────────────

#[derive(Serialize)]
/// Parameters for a `promoteChatMember` request.
pub struct PromoteChatMemberParams {
    chat_id: ChatId,
    user_id: i64,
    /// `true` to make the admin anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Allows the admin to manage the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Allows the admin to delete messages of other users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Allows the admin to manage video chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// Allows the admin to restrict, ban, or unban chat members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Allows the admin to add new administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Allows the admin to change the chat title, photo, and other settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Allows the admin to invite new users to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Allows the admin to post messages in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Allows the admin to edit messages in channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Allows the admin to pin messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Allows the admin to manage forum topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Allows the admin to post stories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    /// Allows the admin to edit stories posted by others.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    /// Allows the admin to delete stories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    /// Allows the admin to manage direct messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    /// Allows the admin to manage tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_tags: Option<bool>,
}

/// Builder for the [`promoteChatMember`](https://core.telegram.org/bots/api#promotechatmember) method.
pub struct PromoteChatMember {
    client: BotClient,
    params: PromoteChatMemberParams,
}

impl PromoteChatMember {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: PromoteChatMemberParams {
                chat_id: chat_id.into(),
                user_id,
                is_anonymous: None,
                can_manage_chat: None,
                can_delete_messages: None,
                can_manage_video_chats: None,
                can_restrict_members: None,
                can_promote_members: None,
                can_change_info: None,
                can_invite_users: None,
                can_post_messages: None,
                can_edit_messages: None,
                can_pin_messages: None,
                can_manage_topics: None,
                can_post_stories: None,
                can_edit_stories: None,
                can_delete_stories: None,
                can_manage_direct_messages: None,
                can_manage_tags: None,
            },
        }
    }
    /// Allows the admin to manage the chat.
    pub fn can_manage_chat(mut self, v: bool) -> Self {
        self.params.can_manage_chat = Some(v);
        self
    }
    /// Allows the admin to delete messages of other users.
    pub fn can_delete_messages(mut self, v: bool) -> Self {
        self.params.can_delete_messages = Some(v);
        self
    }
    /// Allows the admin to manage video chats.
    pub fn can_manage_video_chats(mut self, v: bool) -> Self {
        self.params.can_manage_video_chats = Some(v);
        self
    }
    /// Allows the admin to restrict, ban, or unban chat members.
    pub fn can_restrict_members(mut self, v: bool) -> Self {
        self.params.can_restrict_members = Some(v);
        self
    }
    /// Allows the admin to add new administrators with a subset of their own privileges or demote administrators that they have promoted, directly or indirectly (promoted by administrators that were appointed by the user).
    pub fn can_promote_members(mut self, v: bool) -> Self {
        self.params.can_promote_members = Some(v);
        self
    }
    /// Allows the admin to change the chat title, photo, and other settings.
    pub fn can_change_info(mut self, v: bool) -> Self {
        self.params.can_change_info = Some(v);
        self
    }
    /// Allows the admin to invite new users to the chat.
    pub fn can_invite_users(mut self, v: bool) -> Self {
        self.params.can_invite_users = Some(v);
        self
    }
    /// Allows the admin to pin messages.
    pub fn can_pin_messages(mut self, v: bool) -> Self {
        self.params.can_pin_messages = Some(v);
        self
    }
    /// Allows the admin to manage forum topics.
    pub fn can_manage_topics(mut self, v: bool) -> Self {
        self.params.can_manage_topics = Some(v);
        self
    }
    /// Allows the admin to post stories.
    pub fn can_post_stories(mut self, v: bool) -> Self {
        self.params.can_post_stories = Some(v);
        self
    }
    /// Allows the admin to manage direct messages.
    pub fn can_manage_direct_messages(mut self, v: bool) -> Self {
        self.params.can_manage_direct_messages = Some(v);
        self
    }
    /// Allows the admin to manage tags.
    pub fn can_manage_tags(mut self, v: bool) -> Self {
        self.params.can_manage_tags = Some(v);
        self
    }
}

impl IntoFuture for PromoteChatMember {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("promoteChatMember", &self.params)
                .await
        })
    }
}

// ─── createChatInviteLink ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct CreateChatInviteLinkParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

/// Builder for the [`createChatInviteLink`](https://core.telegram.org/bots/api#createchatinvitelink) method.
pub struct CreateChatInviteLink {
    client: BotClient,
    params: CreateChatInviteLinkParams,
}

impl CreateChatInviteLink {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: CreateChatInviteLinkParams {
                chat_id: chat_id.into(),
                name: None,
                expire_date: None,
                member_limit: None,
                creates_join_request: None,
            },
        }
    }
    /// Sets the name of the invite link (up to 32 characters).
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.params.name = Some(n.into());
        self
    }
    /// Sets the Unix timestamp when the invite link expires.
    pub fn expire_date(mut self, ts: i64) -> Self {
        self.params.expire_date = Some(ts);
        self
    }
    /// Limits how many users can join via this link.
    pub fn member_limit(mut self, n: u32) -> Self {
        self.params.member_limit = Some(n);
        self
    }
    /// Makes the link require admin approval for each join request.
    pub fn creates_join_request(mut self, v: bool) -> Self {
        self.params.creates_join_request = Some(v);
        self
    }
}

impl IntoFuture for CreateChatInviteLink {
    type Output = Result<ChatInviteLink>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("createChatInviteLink", &self.params)
                .await
        })
    }
}

// ─── pinChatMessage ───────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PinChatMessageParams {
    chat_id: ChatId,
    message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

/// Builder for the [`pinChatMessage`](https://core.telegram.org/bots/api#pinchatmessage) method.
pub struct PinChatMessage {
    client: BotClient,
    params: PinChatMessageParams,
}

impl PinChatMessage {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, message_id: i64) -> Self {
        Self {
            client,
            params: PinChatMessageParams {
                chat_id: chat_id.into(),
                message_id,
                disable_notification: None,
            },
        }
    }
    /// Pins the message without notifying members.
    pub fn disable_notification(mut self, v: bool) -> Self {
        self.params.disable_notification = Some(v);
        self
    }
}

impl IntoFuture for PinChatMessage {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("pinChatMessage", &self.params).await })
    }
}

// ─── unpinChatMessage ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct UnpinChatMessageParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<i64>,
}

/// Builder for the [`unpinChatMessage`](https://core.telegram.org/bots/api#unpinchatmessage) method.
pub struct UnpinChatMessage {
    client: BotClient,
    params: UnpinChatMessageParams,
}

impl UnpinChatMessage {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: UnpinChatMessageParams {
                chat_id: chat_id.into(),
                message_id: None,
            },
        }
    }
    /// Unpins a specific message. Omit to unpin the most recent pinned message.
    pub fn message_id(mut self, id: i64) -> Self {
        self.params.message_id = Some(id);
        self
    }
}

impl IntoFuture for UnpinChatMessage {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("unpinChatMessage", &self.params)
                .await
        })
    }
}
