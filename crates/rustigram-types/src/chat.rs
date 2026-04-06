use serde::{Deserialize, Serialize};

use crate::message::Message;
use crate::user::User;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Type of a Telegram chat.
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    /// One-on-one conversation.
    Private,
    /// Group chat.
    Group,
    /// Supergroup.
    Supergroup,
    /// Broadcast channel.
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Minimal chat descriptor — used in messages and updates.
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,

    /// Type of chat.
    #[serde(rename = "type")]
    pub kind: ChatType,

    /// Title, for supergroups, channels and group chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Username, for private chats, supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// First name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// `true` if the supergroup chat is a forum (has topics enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
}

impl Chat {
    /// Returns the chat's display name.
    #[must_use]
    pub fn display_name(&self) -> String {
        self.title
            .clone()
            .or_else(|| {
                self.first_name.as_ref().map(|f| {
                    self.last_name
                        .as_ref()
                        .map_or(f.clone(), |l| format!("{f} {l}"))
                })
            })
            .or_else(|| self.username.clone())
            .unwrap_or_else(|| self.id.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Full chat information — returned by `getChat`.
pub struct ChatFullInfo {
    /// Unique identifier for this chat.
    pub id: i64,

    /// Type of chat.
    #[serde(rename = "type")]
    pub kind: ChatType,

    /// Title of the chat (groups, supergroups, and channels).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Username of the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// First name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// `true` if the supergroup chat is a forum (has topics enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,

    /// Identifier of the accent color for the chat name and backgrounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color_id: Option<u32>,

    /// Maximum number of reactions that can be set on a message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reaction_count: Option<u32>,

    /// Chat photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,

    /// Active usernames of a channel, supergroup, or user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,

    /// Bio of the other party in a private chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// `true` if privacy settings prevent viewing the other party's bio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,

    /// `true` if the privacy settings prevent sending voice and video note messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,

    /// `true` if users need to join in order to send messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,

    /// `true` if all users directly joining the supergroup need to be approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,

    /// Description, for groups, supergroups and channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Primary invite link for the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,

    /// Latest pinned message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,

    /// Default chat member permissions, for groups and supergroups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,

    /// `true` if paid media messages can be sent or forwarded to the channel chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,

    /// Delay in seconds between consecutive messages from a non-administrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<u32>,

    /// Delay in seconds after which all messages sent by the user will be automatically
    /// deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<u32>,

    /// Message auto-delete timer setting for new messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<u32>,

    /// `true` if aggressive anti-spam checks are enabled in the supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,

    /// `true` if non-administrators can only get the list of bots and administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,

    /// `true` if messages from the chat can't be forwarded to other chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    /// `true` if new chat members will have access to old messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,

    /// Name of the group sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,

    /// `true` if the bot can change the group sticker set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,

    /// Custom emoji identifier of the emoji chosen by the chat for the reply header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,

    /// Unique identifier for the linked chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,

    /// The location to which the supergroup is connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Chat photo information.
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo.
    pub small_file_id: String,
    /// Unique file identifier of small chat photo.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo.
    pub big_file_id: String,
    /// Unique file identifier of big chat photo.
    pub big_file_unique_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Defines chat permissions for regular members.
pub struct ChatPermissions {
    /// Allows sending text messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// Allows sending audio files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    /// Allows sending documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    /// Allows sending photos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    /// Allows sending videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    /// Allows sending video notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    /// Allows sending voice notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    /// Allows sending polls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// Allows sending other types of messages (stickers, GIFs, games, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// Allows adding web page previews to messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// Allows changing the chat title, photo, and other settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Allows inviting new users to the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Allows pinning messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Allows managing forum topics (supergroups only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    /// Allows editing the chat tag (supergroups only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_tag: Option<bool>,
}

/// Location to which the supergroup is connected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected.
    pub location: Location,
    /// Location address; 1-64 characters.
    pub address: String,
}

/// Geographic point on the map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Latitude as defined by the sender.
    pub latitude: f64,
    /// Longitude as defined by the sender.
    pub longitude: f64,
    /// Radius of uncertainty for the location, in metres (0–1500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Time relative to the message sending date, during which the location can
    /// be updated; in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// Direction of movement in degrees (1–360).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// Maximum distance in metres for proximity alerts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
}

/// Represents a venue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    /// Venue location.
    pub location: Location,
    /// Name of the venue.
    pub title: String,
    /// Address of the venue.
    pub address: String,
    /// Foursquare identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Invite link for a chat.
pub struct ChatInviteLink {
    /// The invite link.
    pub invite_link: String,
    /// Creator of the link.
    pub creator: User,
    /// `true` if users joining the chat via the link need to be approved by chat admins.
    pub creates_join_request: bool,
    /// `true` if the link is primary.
    pub is_primary: bool,
    /// `true` if the link is revoked.
    pub is_revoked: bool,
    /// Invite link name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix) when the link will expire or has been expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Maximum number of users that can be members of the chat simultaneously.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// Number of pending join requests created using this link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<u32>,
    /// Number of seconds the subscription created by this link will be active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<u32>,
    /// Number of Telegram Stars a user must pay for a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a join request sent to a chat.
pub struct ChatJoinRequest {
    /// The chat the join request was sent to.
    pub chat: Chat,
    /// The user that sent the join request.
    pub from: User,
    /// Identifier of a private chat with the user.
    pub user_chat_id: i64,
    /// Date the request was sent as Unix time.
    pub date: i64,
    /// Bio of the user, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// The invite link used to send the request, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}
