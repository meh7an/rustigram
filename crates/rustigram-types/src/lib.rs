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
//! | [`checklist`] | [`checklist::Checklist`], [`checklist::ChecklistTask`], [`checklist::InputChecklist`], service messages |
//! | [`direct_messages`] | [`direct_messages::DirectMessagesTopic`], price-changed service messages |
//! | [`mod@file`] | [`file::File`], [`file::PhotoSize`], [`file::InputFile`], [`file::VideoQuality`], media types |
//! | [`managed_bot`] | [`managed_bot::ManagedBotCreated`] |
//! | [`message`] | [`message::Message`], [`message::MessageEntity`], [`message::ParseMode`], reply types |
//! | [`update`] | [`update::Update`], [`update::UpdateKind`], [`update::CallbackQuery`], [`update::BusinessBotRights`] |
//! | [`user`] | [`user::User`], [`user::ChatId`], [`user::BotCommand`] |
//! | [`keyboard`] | Inline and reply keyboards, [`keyboard::ReplyMarkup`] |
//! | [`payments`] | [`payments::LabeledPrice`], invoices, Star transactions, [`payments::TransactionPartner`], [`payments::OwnedGift`] |
//! | [`poll`] | [`poll::Poll`], [`poll::PollAnswer`], [`poll::InputPollOption`] |
//! | [`sticker`] | [`sticker::Sticker`], [`sticker::StickerSet`], [`sticker::InputSticker`] |
//! | [`suggested_post`] | [`suggested_post::SuggestedPostInfo`], [`suggested_post::SuggestedPostParameters`], service messages |
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
/// Checklist content type and related service messages.
pub mod checklist;
/// Direct messages topic and channel pricing service messages.
pub mod direct_messages;
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
/// Managed bot service message types.
pub mod managed_bot;
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
/// Suggested post types and service messages.
pub mod suggested_post;
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
pub use checklist::{
    Checklist, ChecklistTask, ChecklistTasksAdded, ChecklistTasksDone, InputChecklist,
    InputChecklistTask,
};
pub use direct_messages::{
    DirectMessagePriceChanged, DirectMessagesTopic, PaidMessagePriceChanged,
};
pub use file::{
    File, InputMedia, InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto,
    InputMediaVideo, InputPaidMedia, InputPaidMediaPhoto, InputPaidMediaVideo, InputProfilePhoto,
    InputProfilePhotoAnimated, InputProfilePhotoStatic, PhotoSize, VideoQuality,
};
pub use inline::{ChosenInlineResult, InlineQuery, InlineQueryResult};
pub use keyboard::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};
pub use managed_bot::ManagedBotCreated;
pub use message::{
    ChatOwnerChanged, ChatOwnerLeft, Message, MessageEntity, MessageEntityKind, MessageOrigin,
    ReactionType, ReplyParameters,
};
pub use payments::{
    AcceptedGiftTypes, AffiliateInfo, LabeledPrice, OrderInfo, OwnedGift, OwnedGiftRegular,
    OwnedGiftUnique, OwnedGifts, PreCheckoutQuery, ShippingAddress, ShippingOption, ShippingQuery,
    StarTransaction, StarTransactions, TransactionPartner,
};
pub use poll::{InputPollOption, Poll, PollAnswer, PollOption, PollOptionAdded, PollOptionDeleted};
pub use sticker::{MaskPosition, Sticker, StickerSet, StickerType};
pub use story::{
    InputStoryContent, InputStoryContentPhoto, InputStoryContentVideo, LocationAddress, Story,
    StoryArea, StoryAreaPosition, StoryAreaType,
};
pub use suggested_post::{
    SuggestedPostApprovalFailed, SuggestedPostApproved, SuggestedPostDeclined, SuggestedPostInfo,
    SuggestedPostPaid, SuggestedPostParameters, SuggestedPostPrice, SuggestedPostRefunded,
};
pub use update::{BusinessBotRights, CallbackQuery, Update, UpdateKind, UserChatBoosts};
pub use user::{
    BotCommand, BotDescription, BotName, BotShortDescription, ChatAdministratorRights, ChatId,
    User, UserProfileAudios, UserProfilePhotos,
};
pub use webhook::WebhookInfo;
