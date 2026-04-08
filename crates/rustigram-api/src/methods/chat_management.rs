use crate::client::BotClient;
use crate::error::Result;
use reqwest::multipart::{Form, Part};
use rustigram_types::chat::{ChatInviteLink, ChatPermissions};
use rustigram_types::file::InputFile;
use rustigram_types::update::UserChatBoosts;
use rustigram_types::user::ChatId;
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

impl_into_future!(BanChatMember, bool, "banChatMember");

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

impl_into_future!(UnbanChatMember, bool, "unbanChatMember");

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

impl_into_future!(RestrictChatMember, bool, "restrictChatMember");

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

impl_into_future!(PromoteChatMember, bool, "promoteChatMember");

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

impl_into_future!(CreateChatInviteLink, ChatInviteLink, "createChatInviteLink");

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

impl_into_future!(PinChatMessage, bool, "pinChatMessage");

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

impl_into_future!(UnpinChatMessage, bool, "unpinChatMessage");

// ─── setChatAdministratorCustomTitle ─────────────────────────────────────────

#[derive(Serialize)]
struct SetChatAdministratorCustomTitleParams {
    chat_id: ChatId,
    user_id: i64,
    custom_title: String,
}

/// Builder for the [`setChatAdministratorCustomTitle`](https://core.telegram.org/bots/api#setchatadministratorcustomtitle) method.
pub struct SetChatAdministratorCustomTitle {
    client: BotClient,
    params: SetChatAdministratorCustomTitleParams,
}

impl SetChatAdministratorCustomTitle {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        user_id: i64,
        custom_title: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SetChatAdministratorCustomTitleParams {
                chat_id: chat_id.into(),
                user_id,
                custom_title: custom_title.into(),
            },
        }
    }
}

impl_into_future!(
    SetChatAdministratorCustomTitle,
    bool,
    "setChatAdministratorCustomTitle"
);

// ─── setChatMemberTag ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetChatMemberTagParams {
    chat_id: ChatId,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

/// Builder for the [`setChatMemberTag`](https://core.telegram.org/bots/api#setchatmembertag) method.
///
/// Bot API 9.5 — requires the `can_manage_tags` administrator right.
pub struct SetChatMemberTag {
    client: BotClient,
    params: SetChatMemberTagParams,
}

impl SetChatMemberTag {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: SetChatMemberTagParams {
                chat_id: chat_id.into(),
                user_id,
                tag: None,
            },
        }
    }
    /// Sets the tag for the member (0–16 characters, no emoji). Omit to remove the tag.
    pub fn tag(mut self, t: impl Into<String>) -> Self {
        self.params.tag = Some(t.into());
        self
    }
}

impl_into_future!(SetChatMemberTag, bool, "setChatMemberTag");

// ─── setChatPermissions ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetChatPermissionsParams {
    chat_id: ChatId,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
}

/// Builder for the [`setChatPermissions`](https://core.telegram.org/bots/api#setchatpermissions) method.
pub struct SetChatPermissions {
    client: BotClient,
    params: SetChatPermissionsParams,
}

impl SetChatPermissions {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            client,
            params: SetChatPermissionsParams {
                chat_id: chat_id.into(),
                permissions,
                use_independent_chat_permissions: None,
            },
        }
    }
    /// Sets whether permissions are applied independently (supergroups only).
    pub fn use_independent_chat_permissions(mut self, v: bool) -> Self {
        self.params.use_independent_chat_permissions = Some(v);
        self
    }
}

impl_into_future!(SetChatPermissions, bool, "setChatPermissions");

// ─── exportChatInviteLink ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct ExportChatInviteLinkParams {
    chat_id: ChatId,
}

/// Builder for the [`exportChatInviteLink`](https://core.telegram.org/bots/api#exportchatinvitelink) method.
///
/// Generates a new primary invite link, revoking any previously generated primary link.
pub struct ExportChatInviteLink {
    client: BotClient,
    params: ExportChatInviteLinkParams,
}

impl ExportChatInviteLink {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: ExportChatInviteLinkParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

impl_into_future!(ExportChatInviteLink, String, "exportChatInviteLink");

// ─── editChatInviteLink ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct EditChatInviteLinkParams {
    chat_id: ChatId,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

/// Builder for the [`editChatInviteLink`](https://core.telegram.org/bots/api#editchatinvitelink) method.
pub struct EditChatInviteLink {
    client: BotClient,
    params: EditChatInviteLinkParams,
}

impl EditChatInviteLink {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: EditChatInviteLinkParams {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
                name: None,
                expire_date: None,
                member_limit: None,
                creates_join_request: None,
            },
        }
    }
    /// Sets the name of the invite link (0–32 characters).
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.params.name = Some(n.into());
        self
    }
    /// Sets the Unix timestamp when the invite link expires.
    pub fn expire_date(mut self, ts: i64) -> Self {
        self.params.expire_date = Some(ts);
        self
    }
    /// Limits how many users can join via this link (1–99999).
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

impl_into_future!(EditChatInviteLink, ChatInviteLink, "editChatInviteLink");

// ─── revokeChatInviteLink ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct RevokeChatInviteLinkParams {
    chat_id: ChatId,
    invite_link: String,
}

/// Builder for the [`revokeChatInviteLink`](https://core.telegram.org/bots/api#revokechatinvitelink) method.
pub struct RevokeChatInviteLink {
    client: BotClient,
    params: RevokeChatInviteLinkParams,
}

impl RevokeChatInviteLink {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: RevokeChatInviteLinkParams {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
            },
        }
    }
}

impl_into_future!(RevokeChatInviteLink, ChatInviteLink, "revokeChatInviteLink");

// ─── createChatSubscriptionInviteLink ────────────────────────────────────────

#[derive(Serialize)]
struct CreateChatSubscriptionInviteLinkParams {
    chat_id: ChatId,
    subscription_period: u32,
    subscription_price: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// Builder for the [`createChatSubscriptionInviteLink`](https://core.telegram.org/bots/api#createchatsubscriptioninvitelink) method.
///
/// Creates a subscription invite link for a channel. The subscription period must currently
/// always be `2592000` (30 days) and the price must be between 1–10000 Telegram Stars.
pub struct CreateChatSubscriptionInviteLink {
    client: BotClient,
    params: CreateChatSubscriptionInviteLinkParams,
}

impl CreateChatSubscriptionInviteLink {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        subscription_period: u32,
        subscription_price: u32,
    ) -> Self {
        Self {
            client,
            params: CreateChatSubscriptionInviteLinkParams {
                chat_id: chat_id.into(),
                subscription_period,
                subscription_price,
                name: None,
            },
        }
    }
    /// Sets the name of the invite link (0–32 characters).
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.params.name = Some(n.into());
        self
    }
}

impl_into_future!(
    CreateChatSubscriptionInviteLink,
    ChatInviteLink,
    "createChatSubscriptionInviteLink"
);

// ─── editChatSubscriptionInviteLink ──────────────────────────────────────────

#[derive(Serialize)]
struct EditChatSubscriptionInviteLinkParams {
    chat_id: ChatId,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// Builder for the [`editChatSubscriptionInviteLink`](https://core.telegram.org/bots/api#editchatsubscriptioninvitelink) method.
pub struct EditChatSubscriptionInviteLink {
    client: BotClient,
    params: EditChatSubscriptionInviteLinkParams,
}

impl EditChatSubscriptionInviteLink {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: EditChatSubscriptionInviteLinkParams {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
                name: None,
            },
        }
    }
    /// Sets the name of the invite link (0–32 characters).
    pub fn name(mut self, n: impl Into<String>) -> Self {
        self.params.name = Some(n.into());
        self
    }
}

impl_into_future!(
    EditChatSubscriptionInviteLink,
    ChatInviteLink,
    "editChatSubscriptionInviteLink"
);

// ─── approveChatJoinRequest ───────────────────────────────────────────────────

#[derive(Serialize)]
struct ApproveChatJoinRequestParams {
    chat_id: ChatId,
    user_id: i64,
}

/// Builder for the [`approveChatJoinRequest`](https://core.telegram.org/bots/api#approvechatjoinrequest) method.
pub struct ApproveChatJoinRequest {
    client: BotClient,
    params: ApproveChatJoinRequestParams,
}

impl ApproveChatJoinRequest {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: ApproveChatJoinRequestParams {
                chat_id: chat_id.into(),
                user_id,
            },
        }
    }
}

impl_into_future!(ApproveChatJoinRequest, bool, "approveChatJoinRequest");

// ─── declineChatJoinRequest ───────────────────────────────────────────────────

#[derive(Serialize)]
struct DeclineChatJoinRequestParams {
    chat_id: ChatId,
    user_id: i64,
}

/// Builder for the [`declineChatJoinRequest`](https://core.telegram.org/bots/api#declinechatjoinrequest) method.
pub struct DeclineChatJoinRequest {
    client: BotClient,
    params: DeclineChatJoinRequestParams,
}

impl DeclineChatJoinRequest {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: DeclineChatJoinRequestParams {
                chat_id: chat_id.into(),
                user_id,
            },
        }
    }
}

impl_into_future!(DeclineChatJoinRequest, bool, "declineChatJoinRequest");

// ─── banChatSenderChat ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct BanChatSenderChatParams {
    chat_id: ChatId,
    sender_chat_id: i64,
}

/// Builder for the [`banChatSenderChat`](https://core.telegram.org/bots/api#banchatsenderchat) method.
///
/// Bans a channel chat in a supergroup or channel. The owner of the banned chat
/// will not be able to send messages on behalf of any of their channels until unbanned.
pub struct BanChatSenderChat {
    client: BotClient,
    params: BanChatSenderChatParams,
}

impl BanChatSenderChat {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, sender_chat_id: i64) -> Self {
        Self {
            client,
            params: BanChatSenderChatParams {
                chat_id: chat_id.into(),
                sender_chat_id,
            },
        }
    }
}

impl_into_future!(BanChatSenderChat, bool, "banChatSenderChat");

// ─── unbanChatSenderChat ──────────────────────────────────────────────────────

#[derive(Serialize)]
struct UnbanChatSenderChatParams {
    chat_id: ChatId,
    sender_chat_id: i64,
}

/// Builder for the [`unbanChatSenderChat`](https://core.telegram.org/bots/api#unbanchatsenderchat) method.
pub struct UnbanChatSenderChat {
    client: BotClient,
    params: UnbanChatSenderChatParams,
}

impl UnbanChatSenderChat {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, sender_chat_id: i64) -> Self {
        Self {
            client,
            params: UnbanChatSenderChatParams {
                chat_id: chat_id.into(),
                sender_chat_id,
            },
        }
    }
}

impl_into_future!(UnbanChatSenderChat, bool, "unbanChatSenderChat");

// ─── unpinAllChatMessages ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct UnpinAllChatMessagesParams {
    chat_id: ChatId,
}

/// Builder for the [`unpinAllChatMessages`](https://core.telegram.org/bots/api#unpinallchatmessages) method.
///
/// Clears the entire list of pinned messages in a chat.
pub struct UnpinAllChatMessages {
    client: BotClient,
    params: UnpinAllChatMessagesParams,
}

impl UnpinAllChatMessages {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: UnpinAllChatMessagesParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

impl_into_future!(UnpinAllChatMessages, bool, "unpinAllChatMessages");

// ─── setChatPhoto ─────────────────────────────────────────────────────────────

/// Builder for the [`setChatPhoto`](https://core.telegram.org/bots/api#setchatphoto) method.
///
/// Sets a new profile photo for the chat. Must be uploaded via multipart/form-data.
pub struct SetChatPhoto {
    client: BotClient,
    chat_id: ChatId,
    photo: InputFile,
}

impl SetChatPhoto {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, photo: InputFile) -> Self {
        Self {
            client,
            chat_id: chat_id.into(),
            photo,
        }
    }
}

impl IntoFuture for SetChatPhoto {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            match self.photo {
                InputFile::Bytes {
                    filename,
                    data,
                    mime_type,
                } => {
                    let part = Part::bytes(data)
                        .file_name(filename)
                        .mime_str(&mime_type)
                        .map_err(|e| crate::error::Error::Decode(e.to_string()))?;
                    let form = Form::new()
                        .text("chat_id", self.chat_id.to_string())
                        .part("photo", part);
                    self.client.post_multipart("setChatPhoto", form).await
                }
                other => {
                    let body = serde_json::json!({
                        "chat_id": self.chat_id,
                        "photo": other.as_str(),
                    });
                    self.client.post_json("setChatPhoto", &body).await
                }
            }
        })
    }
}

// ─── deleteChatPhoto ──────────────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteChatPhotoParams {
    chat_id: ChatId,
}

/// Builder for the [`deleteChatPhoto`](https://core.telegram.org/bots/api#deletechatphoto) method.
pub struct DeleteChatPhoto {
    client: BotClient,
    params: DeleteChatPhotoParams,
}

impl DeleteChatPhoto {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: DeleteChatPhotoParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

impl_into_future!(DeleteChatPhoto, bool, "deleteChatPhoto");

// ─── setChatTitle ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetChatTitleParams {
    chat_id: ChatId,
    title: String,
}

/// Builder for the [`setChatTitle`](https://core.telegram.org/bots/api#setchattitle) method.
pub struct SetChatTitle {
    client: BotClient,
    params: SetChatTitleParams,
}

impl SetChatTitle {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SetChatTitleParams {
                chat_id: chat_id.into(),
                title: title.into(),
            },
        }
    }
}

impl_into_future!(SetChatTitle, bool, "setChatTitle");

// ─── setChatDescription ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetChatDescriptionParams {
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// Builder for the [`setChatDescription`](https://core.telegram.org/bots/api#setchatdescription) method.
///
/// Pass an empty string or omit `description` to remove the current description.
pub struct SetChatDescription {
    client: BotClient,
    params: SetChatDescriptionParams,
}

impl SetChatDescription {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: SetChatDescriptionParams {
                chat_id: chat_id.into(),
                description: None,
            },
        }
    }
    /// Sets the new description (0–255 characters). Omit to clear the description.
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.params.description = Some(d.into());
        self
    }
}

impl_into_future!(SetChatDescription, bool, "setChatDescription");

// ─── setChatStickerSet ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SetChatStickerSetParams {
    chat_id: ChatId,
    sticker_set_name: String,
}

/// Builder for the [`setChatStickerSet`](https://core.telegram.org/bots/api#setchatstickerset) method.
pub struct SetChatStickerSet {
    client: BotClient,
    params: SetChatStickerSetParams,
}

impl SetChatStickerSet {
    pub(crate) fn new(
        client: BotClient,
        chat_id: impl Into<ChatId>,
        sticker_set_name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            params: SetChatStickerSetParams {
                chat_id: chat_id.into(),
                sticker_set_name: sticker_set_name.into(),
            },
        }
    }
}

impl_into_future!(SetChatStickerSet, bool, "setChatStickerSet");

// ─── deleteChatStickerSet ─────────────────────────────────────────────────────

#[derive(Serialize)]
struct DeleteChatStickerSetParams {
    chat_id: ChatId,
}

/// Builder for the [`deleteChatStickerSet`](https://core.telegram.org/bots/api#deletechatstickerset) method.
pub struct DeleteChatStickerSet {
    client: BotClient,
    params: DeleteChatStickerSetParams,
}

impl DeleteChatStickerSet {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: DeleteChatStickerSetParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

impl_into_future!(DeleteChatStickerSet, bool, "deleteChatStickerSet");

// ─── leaveChat ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct LeaveChatParams {
    chat_id: ChatId,
}

/// Builder for the [`leaveChat`](https://core.telegram.org/bots/api#leavechat) method.
pub struct LeaveChat {
    client: BotClient,
    params: LeaveChatParams,
}

impl LeaveChat {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>) -> Self {
        Self {
            client,
            params: LeaveChatParams {
                chat_id: chat_id.into(),
            },
        }
    }
}

impl_into_future!(LeaveChat, bool, "leaveChat");

// ─── getUserChatBoosts ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GetUserChatBoostsParams {
    chat_id: ChatId,
    user_id: i64,
}

/// Builder for the [`getUserChatBoosts`](https://core.telegram.org/bots/api#getuserchatboosts) method.
///
/// Returns the list of boosts added to a chat by a specific user.
/// Requires administrator rights in the chat.
pub struct GetUserChatBoosts {
    client: BotClient,
    params: GetUserChatBoostsParams,
}

impl GetUserChatBoosts {
    pub(crate) fn new(client: BotClient, chat_id: impl Into<ChatId>, user_id: i64) -> Self {
        Self {
            client,
            params: GetUserChatBoostsParams {
                chat_id: chat_id.into(),
                user_id,
            },
        }
    }
}

impl_into_future!(GetUserChatBoosts, UserChatBoosts, "getUserChatBoosts");
