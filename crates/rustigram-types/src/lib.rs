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
//! | [`rich_message`] | [`rich_message::RichMessage`], [`rich_message::RichBlock`], [`rich_message::RichText`], [`rich_message::InputRichMessage`] |
//! | [`sticker`] | [`sticker::Sticker`], [`sticker::StickerSet`], [`sticker::InputSticker`] |
//! | [`suggested_post`] | [`suggested_post::SuggestedPostInfo`], [`suggested_post::SuggestedPostParameters`], service messages |
//! | [`inline`] | [`inline::InlineQuery`], all `InlineQueryResult` variants |
//! | [`story`] | Story types and area definitions |
//! | [`forum`] | [`forum::ForumTopic`] and the six forum service messages |
//! | [`shared`] | [`shared::UsersShared`], [`shared::ChatShared`], [`shared::SharedUser`] |
//! | [`video_chat`] | The four video chat service messages |
//! | [`passport`] | Telegram Passport types and error variants |
//! | [`games`] | [`games::GameHighScore`] |
//! | [`gifts`] | [`gifts::Gift`], [`gifts::Gifts`] |
//! | [`giveaway`] | [`giveaway::Giveaway`] and the three giveaway service messages |
//! | [`reaction`] | Re-exports [`message::ReactionType`] |
//! | [`webhook`] | [`webhook::WebhookInfo`] |
#![warn(missing_docs)]

/// Chat and location types.
pub mod chat;
/// Chat member status types.
pub mod chat_member;
/// Checklist content type and related service messages.
pub mod checklist;
/// Community (linked chats) types.
pub mod community;
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
/// Giveaway types and service messages.
pub mod giveaway;
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
/// Rich message types (Bot API 10.1).
pub mod rich_message;
/// Users and chats shared with the bot via keyboard request buttons.
pub mod shared;
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
/// Video chat service message types.
pub mod video_chat;
/// Webhook info types.
pub mod webhook;

pub use chat::{
    Chat, ChatFullInfo, ChatInviteLink, ChatJoinRequest, ChatPermissions, ChatType, InputMediaLink,
    Link,
};
pub use chat_member::{
    ChatMember, ChatMemberAdministrator, ChatMemberBanned, ChatMemberLeft, ChatMemberMember,
    ChatMemberOwner, ChatMemberRestricted,
};
pub use checklist::{
    Checklist, ChecklistTask, ChecklistTasksAdded, ChecklistTasksDone, InputChecklist,
    InputChecklistTask,
};
pub use community::{Community, CommunityChatAdded, CommunityChatRemoved};
pub use direct_messages::{
    DirectMessagePriceChanged, DirectMessagesTopic, PaidMessagePriceChanged,
};
pub use file::{
    File, InputMedia, InputMediaAnimation, InputMediaAudio, InputMediaDocument,
    InputMediaLivePhoto, InputMediaLocation, InputMediaPhoto, InputMediaSticker, InputMediaVenue,
    InputMediaVideo, InputMediaVoiceNote, InputPaidMedia, InputPaidMediaLivePhoto,
    InputPaidMediaPhoto, InputPaidMediaVideo, InputProfilePhoto, InputProfilePhotoAnimated,
    InputProfilePhotoStatic, LivePhoto, PhotoSize, VideoQuality,
};
pub use inline::{ChosenInlineResult, InlineQuery, InlineQueryResult, SentGuestMessage};
pub use keyboard::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, ReplyKeyboardMarkup,
    ReplyKeyboardRemove,
};
pub use managed_bot::ManagedBotCreated;
pub use forum::{
    ForumTopic, ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened,
    GeneralForumTopicHidden, GeneralForumTopicUnhidden,
};
pub use message::{
    ChatOwnerChanged, ChatOwnerLeft, Message, MessageAutoDeleteTimerChanged, MessageEntity,
    MessageEntityKind, MessageOrigin, ProximityAlertTriggered, ReactionType, ReplyParameters,
    WriteAccessAllowed,
};
pub use shared::{ChatShared, SharedUser, UsersShared};
pub use video_chat::{
    VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted,
};
pub use games::{CallbackGame, Game, GameHighScore};
pub use gifts::{GiftInfo, UniqueGiftInfo};
pub use giveaway::{Giveaway, GiveawayCompleted, GiveawayCreated, GiveawayWinners};
pub use payments::{
    AcceptedGiftTypes, AffiliateInfo, BotSubscriptionUpdated, Invoice, LabeledPrice, OrderInfo,
    OwnedGift, OwnedGiftRegular, OwnedGiftUnique, OwnedGifts, PaidMedia, PaidMediaInfo,
    PaidMediaLivePhoto, PaidMediaPhoto, PaidMediaPreview, PaidMediaVideo, PreCheckoutQuery,
    RefundedPayment, ShippingAddress, ShippingOption, ShippingQuery, StarTransaction,
    StarTransactions, SuccessfulPayment, TransactionPartner,
};
pub use poll::{
    InputPollMedia, InputPollOption, InputPollOptionMedia, Poll, PollAnswer, PollMedia, PollOption,
    PollOptionAdded, PollOptionDeleted,
};
pub use rich_message::{
    InputRichBlock, InputRichBlockAnchor, InputRichBlockAnimation, InputRichBlockAudio,
    InputRichBlockBlockQuotation, InputRichBlockCollage, InputRichBlockDetails,
    InputRichBlockDivider, InputRichBlockFooter, InputRichBlockList, InputRichBlockListItem,
    InputRichBlockMap, InputRichBlockMathematicalExpression, InputRichBlockParagraph,
    InputRichBlockPhoto, InputRichBlockPreformatted, InputRichBlockPullQuotation,
    InputRichBlockSectionHeading, InputRichBlockSlideshow, InputRichBlockTable,
    InputRichBlockThinking, InputRichBlockVideo, InputRichBlockVoiceNote, InputRichMessage,
    InputRichMessageContent, InputRichMessageMedia, InputRichMessageMediaKind, RichBlock,
    RichBlockAnchor, RichBlockAnimation, RichBlockAudio, RichBlockBlockQuotation, RichBlockCaption,
    RichBlockCollage, RichBlockDetails, RichBlockDivider, RichBlockFooter, RichBlockList,
    RichBlockListItem, RichBlockMap, RichBlockMathematicalExpression, RichBlockPhoto,
    RichBlockPreformatted, RichBlockPullQuotation, RichBlockSectionHeading, RichBlockSlideshow,
    RichBlockTable, RichBlockTableCell, RichBlockThinking, RichBlockVideo, RichBlockVoiceNote,
    RichMessage, RichText, RichTextAnchor, RichTextAnchorLink, RichTextBankCardNumber,
    RichTextBold, RichTextBotCommand, RichTextCashtag, RichTextCode, RichTextCustomEmoji,
    RichTextDateTime, RichTextEmailAddress, RichTextFootnote, RichTextHashtag, RichTextItalic,
    RichTextMarked, RichTextMathematicalExpression, RichTextMention, RichTextPhoneNumber,
    RichTextReference, RichTextSpoiler, RichTextStrikethrough, RichTextSubscript,
    RichTextSuperscript, RichTextTextMention, RichTextUnderline, RichTextUrl,
};
pub use sticker::{MaskPosition, Sticker, StickerSet, StickerType};
pub use story::{
    InputStoryContent, InputStoryContentPhoto, InputStoryContentVideo, LocationAddress, Story,
    StoryArea, StoryAreaPosition, StoryAreaType,
};
pub use suggested_post::{
    SuggestedPostApprovalFailed, SuggestedPostApproved, SuggestedPostDeclined, SuggestedPostInfo,
    SuggestedPostPaid, SuggestedPostParameters, SuggestedPostPrice, SuggestedPostRefunded,
};
pub use update::{
    BusinessBotRights, CallbackQuery, ChatBoostAdded, Update, UpdateKind, UserChatBoosts,
};
pub use user::{
    BotAccessSettings, BotCommand, BotDescription, BotName, BotShortDescription,
    ChatAdministratorRights, ChatId, User, UserProfileAudios, UserProfilePhotos,
};
pub use webhook::WebhookInfo;
