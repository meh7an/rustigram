use serde::{Deserialize, Serialize};

use crate::message::WebAppInfo;

/// An inline keyboard attached to a message.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of
    /// [`InlineKeyboardButton`] objects.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Creates an empty inline keyboard.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a row of buttons.
    #[must_use]
    pub fn row(mut self, row: Vec<InlineKeyboardButton>) -> Self {
        self.inline_keyboard.push(row);
        self
    }
}

/// One button in an inline keyboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,
    /// Custom emoji identifier shown before the button text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// Visual style of the button (`"danger"`, `"success"`, or `"primary"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    /// URL to open when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Data to be sent in a callback query (1–64 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Web App to launch when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// Defines an authentication button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    /// Pressing the button prompts the user to select a chat and opens an inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// Pressing the button opens an inline query in the current chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// Prompts the user to select a specific type of chat for an inline query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    /// Describes a button that copies specified text to the clipboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,
    /// Description of the game that will be launched when the user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<serde_json::Value>,
    /// Specify `true` to send a Pay button (invoices only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

impl InlineKeyboardButton {
    /// Creates a callback button.
    #[must_use]
    pub fn callback(text: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            callback_data: Some(data.into()),
            icon_custom_emoji_id: None,
            style: None,
            url: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            callback_game: None,
            pay: None,
        }
    }

    /// Creates a URL button.
    #[must_use]
    pub fn url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            icon_custom_emoji_id: None,
            style: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            callback_game: None,
            pay: None,
        }
    }

    /// Creates a Web App button.
    #[must_use]
    pub fn web_app(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            web_app: Some(WebAppInfo { url: url.into() }),
            icon_custom_emoji_id: None,
            style: None,
            url: None,
            callback_data: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            callback_game: None,
            pay: None,
        }
    }
}

/// The visual style applied to an inline or reply keyboard button.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    /// Red destructive button style.
    Danger,
    /// Green positive button style.
    Success,
    /// Default blue button style.
    Primary,
}

/// Parameters for a Login URL button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUrl {
    /// HTTPS URL to forward the user to.
    pub url: String,
    /// New text of the button in forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    /// Username of the bot to use for user authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    /// `true` to request permission for the bot to send messages to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

/// Parameters for inline query routing to a specific type of chat.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// Default inline query to insert in the input field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// `true` if private chats with users can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// `true` if private chats with bots can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// `true` if group and supergroup chats can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// `true` if channel chats can be chosen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

/// Represents a button that copies text to clipboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyTextButton {
    /// Text to copy (1–256 characters).
    pub text: String,
}

/// Custom keyboard shown to the message recipient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows.
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Whether the keyboard is persistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    /// Requests clients to resize the keyboard vertically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard after a button is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// Placeholder text shown in the input field when the keyboard is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Show keyboard to specific users only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// One button in a reply keyboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Label text on the button.
    pub text: String,
    /// Custom emoji identifier shown before the button text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
    /// Visual style of the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
    /// Request to select and share one or more users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,
    /// Request to select and share a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    /// Request a managed bot from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_managed_bot: Option<KeyboardButtonRequestManagedBot>,
    /// Requests the user's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// Requests the user's current location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    /// Requests the user to create a poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    /// Web App to launch when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

impl KeyboardButton {
    /// Creates a simple text button.
    #[must_use]
    pub fn text(label: impl Into<String>) -> Self {
        Self {
            text: label.into(),
            icon_custom_emoji_id: None,
            style: None,
            request_users: None,
            request_chat: None,
            request_managed_bot: None,
            request_contact: None,
            request_location: None,
            request_poll: None,
            web_app: None,
        }
    }

    /// Creates a button that requests the user's phone number.
    #[must_use]
    pub fn request_contact(label: impl Into<String>) -> Self {
        Self {
            request_contact: Some(true),
            ..Self::text(label)
        }
    }

    /// Creates a button that requests the user's location.
    #[must_use]
    pub fn request_location(label: impl Into<String>) -> Self {
        Self {
            request_location: Some(true),
            ..Self::text(label)
        }
    }
}

/// Defines criteria for selecting users via a keyboard button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request.
    pub request_id: i32,
    /// `true` to request only bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// `true` to request only premium users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    /// Maximum number of users to be selected (1–10, default 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<u8>,
    /// `true` to request the user's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    /// `true` to request the user's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// `true` to request the user's profile photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// Defines criteria for selecting a chat via a keyboard button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request.
    pub request_id: i32,
    /// `true` to request a channel chat; `false` for group or supergroup.
    pub chat_is_channel: bool,
    /// `true` to request a forum supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// `true` to request a chat with a username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// `true` to request a chat owned by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    /// Required administrator rights of the user in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<serde_json::Value>,
    /// Required administrator rights of the bot in the chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<serde_json::Value>,
    /// `true` to request a chat where the bot is a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    /// `true` to request the chat title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    /// `true` to request the chat username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// `true` to request the chat photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// Defines parameters for requesting the creation of a managed bot.
///
/// Bot API 9.6 — available for bots that have enabled managed bot creation in @BotFather.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonRequestManagedBot {
    /// Signed 32-bit identifier of the request; must be unique within the message.
    pub request_id: i32,
    /// Suggested name for the new bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_name: Option<String>,
    /// Suggested username for the new bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_username: Option<String>,
}

/// The type of poll requested via a keyboard button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    /// `"quiz"`, `"regular"`, or absent (any type).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// Instructs clients to remove the reply keyboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Must be `true`.
    pub remove_keyboard: bool,
    /// Show the remove keyboard to specific users only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// Forces a reply from the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceReply {
    /// Must be `true`.
    pub force_reply: bool,
    /// Placeholder text in the input field when the reply is active (1–64 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Show the force reply to specific users only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// All reply markup variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// An inline keyboard attached to the message.
    InlineKeyboard(InlineKeyboardMarkup),
    /// A custom reply keyboard shown to the user.
    ReplyKeyboard(ReplyKeyboardMarkup),
    /// Removes the reply keyboard.
    Remove(ReplyKeyboardRemove),
    /// Forces the user to reply to the message.
    ForceReply(ForceReply),
}

/// Menu button configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButton {
    /// Shows the list of bot commands.
    Commands,
    /// Launches a Web App.
    WebApp {
        /// Button label text.
        text: String,
        /// Web App to launch.
        web_app: WebAppInfo,
    },
    /// No action — uses the default behavior.
    Default,
}
