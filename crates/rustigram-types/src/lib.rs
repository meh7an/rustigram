//! Type definitions for the Telegram Bot API.
//!
//! This crate contains every struct, enum, and type alias described in the
//! [official Bot API documentation](https://core.telegram.org/bots/api).
//! All types implement [`serde::Serialize`] and [`serde::Deserialize`] and
//! map directly to the JSON objects Telegram sends and receives.
//!
//! You rarely need to depend on this crate directly — the `rustigram` facade
//! re-exports everything you need for day-to-day bot development.
//!
//! # Organisation
//!
//! | Module | Contents |
//! |---|---|
//! | [`chat`] | [`chat::Chat`], [`chat::ChatFullInfo`], [`chat::ChatPermissions`], locations, venues |
//! | [`chat_member`] | All six [`chat_member::ChatMember`] variants |
//! | [`mod@file`] | [`file::File`], [`file::PhotoSize`], [`file::InputFile`], media types |
//! | [`message`] | [`message::Message`], [`message::MessageEntity`], [`message::ParseMode`], reply types |
//! | [`update`] | [`update::Update`], [`update::UpdateKind`], [`update::CallbackQuery`] |
//! | [`user`] | [`user::User`], [`user::ChatId`], [`user::BotCommand`] |
//! | [`keyboard`] | Inline and reply keyboards, [`keyboard::ReplyMarkup`] |
//! | [`payments`] | [`payments::LabeledPrice`], invoices, Star transactions |
//! | [`poll`] | [`poll::Poll`], [`poll::PollAnswer`], [`poll::InputPollOption`] |
//! | [`sticker`] | [`sticker::Sticker`], [`sticker::StickerSet`], [`sticker::InputSticker`] |
//! | [`inline`] | [`inline::InlineQuery`], all `InlineQueryResult` variants |
//! | [`story`] | Story types and area definitions |
//! | [`forum`] | [`forum::ForumTopic`] |
//! | [`passport`] | Telegram Passport types and error variants |
//! | [`games`] | [`games::GameHighScore`] |
//! | [`gifts`] | [`gifts::Gift`], [`gifts::Gifts`] |
//! | [`reaction`] | Re-exports [`message::ReactionType`] |
//! | [`webhook`] | [`webhook::WebhookInfo`] |

#![warn(missing_docs)]
/// Chat and location types.
pub mod chat;
/// Chat member status types.
pub mod chat_member;
/// File and media types.
pub mod file;
/// Forum topic types.
pub mod forum;
/// Game types.
pub mod games;
/// Gift types.
pub mod gifts;
/// Inline query and result types.
pub mod inline;
/// Keyboard and markup types.
pub mod keyboard;
/// Message and entity types.
pub mod message;
/// Telegram Passport types.
pub mod passport;
/// Payment and invoice types.
pub mod payments;
/// Poll types.
pub mod poll;
/// Reaction types.
pub mod reaction;
/// Sticker types.
pub mod sticker;
/// Story types.
pub mod story;
/// Update and event types.
pub mod update;
/// User and bot command types.
pub mod user;
/// Webhook info types.
pub mod webhook;

pub use chat::{Chat, ChatFullInfo, ChatPermissions, ChatType};
pub use chat_member::{
    ChatMember, ChatMemberAdministrator, ChatMemberBanned, ChatMemberLeft, ChatMemberMember,
    ChatMemberOwner, ChatMemberRestricted,
};
pub use file::{File, PhotoSize};
pub use inline::{ChosenInlineResult, InlineQuery, InlineQueryResult};
pub use keyboard::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};
pub use message::{
    Message, MessageEntity, MessageEntityKind, MessageOrigin, ReactionType, ReplyParameters,
};
pub use payments::{
    LabeledPrice, OrderInfo, PreCheckoutQuery, ShippingAddress, ShippingOption, ShippingQuery,
};
pub use poll::{InputPollOption, Poll, PollAnswer, PollOption};
pub use sticker::{MaskPosition, Sticker, StickerSet, StickerType};
pub use update::{CallbackQuery, Update, UpdateKind};
pub use user::{User, UserProfilePhotos};
pub use webhook::WebhookInfo;
