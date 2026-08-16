//! Building spec-shaped JSON payloads, and decoding them back by type name.
//!
//! Shared by every test that has to construct a value for an arbitrary Bot API
//! type. Rust cannot enumerate its own types at runtime, so the dispatch table
//! below is written out — and checked against the snapshot in both directions
//! by `conformance_roundtrip`, which is what stops it rotting.

#![allow(dead_code)] // each test binary uses a different subset

use super::Spec;
use rustigram_types::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Map, Value, json};

/// Which fields a generated payload carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fill {
    /// Required fields only, so every optional one is absent.
    Required,
    /// Every field the spec lists, so each optional one is present and typed.
    All,
}

/// How deep the generator will follow nested object types.
///
/// `Message` refers to `Message`, `Chat` to `Chat`, and several union members
/// reach back to their own parents. Only required fields are generated, which
/// terminates in practice — this is the backstop that turns a future cycle into
/// a bounded array rather than a stack overflow.
const MAX_DEPTH: usize = 8;

/// Types that need an otherwise-optional field present to be a valid value.
///
/// `Update` marks every kind optional because only one arrives at a time, but
/// Telegram never sends a bare `{"update_id": n}` and rustigram models the kinds
/// as a flattened enum that must match one of them. The spec cannot express
/// "exactly one of these", so the requirement is stated here.
const REQUIRES_ONE_OF: [(&str, &str); 1] = [("Update", "message")];

/// Values for fields the docs define by reference rather than in place.
///
/// `parse_mode` reads "See [formatting options] for more details" in every one
/// of its 60-odd appearances — the values live in a narrative section, not in
/// the field's own description, so the snapshot's `enum_values` cannot see them.
/// Matched on a field-name suffix: the same parameter appears as `parse_mode`,
/// `text_parse_mode`, and `quote_parse_mode` depending on what it applies to,
/// and it means the same thing in all three.
const FIELD_VALUES: [(&str, &str); 1] = [("parse_mode", "HTML")];

/// Unions the spec also admits as a bare scalar, with the value to use.
///
/// `RichText` is documented as "either a String for plain text, an Array of
/// RichText, or any of the following types", and every one of those 25 types
/// requires a nested `RichText` of its own. A generator that always reaches for
/// an object member therefore never terminates — the string form is the base
/// case, and the spec's own first-listed one.
const SCALAR_UNIONS: [(&str, &str); 1] = [("RichText", "plain text")];

/// A minimal valid payload for `name`, built from its spec field table.
pub fn payload(spec: &Spec, name: &str, fill: Fill, depth: usize) -> Result<Value, String> {
    if depth > MAX_DEPTH {
        return Err(format!("`{name}` nests deeper than {MAX_DEPTH} levels"));
    }
    // A union base name has no field table of its own; any member is a valid
    // payload for it, and the first keeps generation deterministic.
    if let Some(members) = spec.unions.get(name) {
        if !spec.types.get(name).is_some_and(|f| !f.is_empty()) {
            let first = members
                .first()
                .ok_or_else(|| format!("union `{name}` has no members"))?;
            return payload(spec, first, fill, depth + 1);
        }
    }
    let fields = spec
        .types
        .get(name)
        .ok_or_else(|| format!("`{name}` is not a type in the snapshot"))?;

    let mut object = Map::new();

    // The discriminant first: a tagged type that arrives without its tag is not
    // a valid payload, and serde would reject it before any field is examined.
    if let Some(literal) = spec.discriminants.get(name) {
        let tag = spec
            .discriminant_fields
            .get(name)
            .ok_or_else(|| format!("`{name}` has discriminant `{literal}` but no tag field"))?;
        object.insert(tag.clone(), json!(literal));
    }

    let required_extra = REQUIRES_ONE_OF
        .iter()
        .find(|(type_name, _)| *type_name == name)
        .map(|(_, field)| *field);

    for (field, spec_field) in fields {
        // Past the depth limit optional fields are dropped rather than
        // generated: a maximal payload follows every reference, and several
        // types reach back to themselves through an optional field.
        let needed = !spec_field.optional()
            || required_extra == Some(field.as_str())
            || (fill == Fill::All && depth < MAX_DEPTH);
        if !needed || object.contains_key(field) {
            continue;
        }
        // A field the docs give a fixed set of values for must receive one of
        // them. Any of the values is valid; the first is deterministic.
        let documented = spec
            .enum_values
            .get(&format!("{name}.{field}"))
            .and_then(|values| values.first().cloned())
            .or_else(|| {
                FIELD_VALUES
                    .iter()
                    .find(|(candidate, _)| field.ends_with(candidate))
                    .map(|(_, value)| (*value).to_owned())
            });

        let value = match documented {
            Some(literal) if spec_field.kind() == "String" => json!(literal),
            _ => match value_for(spec, spec_field.kind(), fill, depth + 1) {
                Ok(value) => value,
                // An optional field that cannot be generated is simply left
                // out — that is what optional means, and it is the base case
                // for the several types that reach back to themselves through
                // an optional reference. A required one is still fatal.
                Err(_) if spec_field.optional() => continue,
                Err(why) => return Err(format!("{name}.{field}: {why}")),
            },
        };
        object.insert(field.clone(), value);
    }

    Ok(Value::Object(object))
}

/// A value matching the spec's own type text, e.g. `Integer` or `Array of User`.
fn value_for(spec: &Spec, kind: &str, fill: Fill, depth: usize) -> Result<Value, String> {
    let kind = kind.trim();

    if let Some(element) = kind.strip_prefix("Array of ") {
        // Past the depth limit an empty array is still a valid value, so the
        // outer type can finish generating rather than failing wholesale.
        if depth > MAX_DEPTH {
            return Ok(json!([]));
        }
        return Ok(json!([value_for(spec, element, fill, depth + 1)?]));
    }

    // `Integer or String`, `InputFile or String` — any alternative is valid, and
    // taking the first keeps the generated payload deterministic.
    if let Some((first, _)) = kind.split_once(" or ") {
        return value_for(spec, first, fill, depth);
    }

    match kind {
        "Integer" => Ok(json!(1)),
        "String" => Ok(json!("x")),
        "Boolean" | "True" => Ok(json!(true)),
        "Float" | "Float number" => Ok(json!(1.0)),
        // In a JSON body an uploaded file is always a file_id or an
        // `attach://` reference. The bytes travel in the multipart form, which
        // is the transport tests' subject, not this one's.
        "InputFile" => Ok(json!("attach://file")),
        _ => {
            if let Some((_, scalar)) = SCALAR_UNIONS.iter().find(|(name, _)| *name == kind) {
                return Ok(json!(scalar));
            }
            if let Some(members) = spec.unions.get(kind) {
                let first = members
                    .first()
                    .ok_or_else(|| format!("union `{kind}` has no members"))?;
                return payload(spec, first, fill, depth + 1);
            }
            if spec.types.contains_key(kind) {
                return payload(spec, kind, fill, depth + 1);
            }
            Err(format!("no way to generate a `{kind}`"))
        }
    }
}

// ─── Round tripping ──────────────────────────────────────────────────────────

/// Deserialises `json` as `T` and serialises it back.
pub fn round_trip<T: DeserializeOwned + Serialize>(json: &Value) -> Result<Value, String> {
    let value: T = serde_json::from_value(json.clone())
        .map_err(|e| format!("did not decode: {e}\n      payload: {json}"))?;
    serde_json::to_value(&value).map_err(|e| format!("did not re-encode: {e}"))
}

/// Maps every spec type name to the Rust type that must carry it.
///
/// Rust cannot enumerate its own types at runtime, so this table is written out.
/// It cannot rot: [`the_dispatch_table_covers_every_spec_type`] compares it
/// against the snapshot, and a name in neither the table nor the exclusion list
/// fails the suite.
macro_rules! dispatch {
    (direct: [$($direct:ident),* $(,)?], via: [$($variant:ident => $parent:ident),* $(,)?] $(,)?) => {
        pub fn round_trip_by_name(name: &str, json: &Value) -> Result<Value, String> {
            match name {
                $(stringify!($direct) => round_trip::<$direct>(json),)*
                $(stringify!($variant) => round_trip::<$parent>(json),)*
                _ => Err(format!("`{name}` has no entry in the dispatch table")),
            }
        }

        /// Every name the table can handle.
        pub const DISPATCHED: &[&str] = &[
            $(stringify!($direct),)*
            $(stringify!($variant),)*
        ];
    };
}

dispatch! {
    // Types with a Rust type of the same name.
    direct: [
        AcceptedGiftTypes, AffiliateInfo, Animation, Audio, BackgroundFill, BackgroundType,
        Birthdate, BotAccessSettings, BotCommand, BotCommandScope, BotDescription, BotName,
        BotShortDescription, BotSubscriptionUpdated, BusinessBotRights, BusinessConnection,
        BusinessIntro, BusinessLocation, BusinessMessagesDeleted, BusinessOpeningHours,
        BusinessOpeningHoursInterval, CallbackGame, CallbackQuery, Chat,
        ChatAdministratorRights, ChatBackground, ChatBoost, ChatBoostAdded,
        ChatBoostRemoved, ChatBoostSource, ChatBoostUpdated, ChatFullInfo, ChatInviteLink,
        ChatJoinRequest, ChatLocation, ChatMember, ChatMemberUpdated, ChatOwnerChanged,
        ChatOwnerLeft, ChatPermissions, ChatPhoto, ChatShared, Checklist, ChecklistTask,
        ChecklistTasksAdded, ChecklistTasksDone, ChosenInlineResult, Community,
        CommunityChatAdded, CommunityChatRemoved, Contact, CopyTextButton, Dice,
        DirectMessagePriceChanged, DirectMessagesTopic, Document, EncryptedCredentials,
        EncryptedPassportElement, ExternalReplyInfo, File, ForceReply, ForumTopic,
        ForumTopicClosed, ForumTopicCreated, ForumTopicEdited, ForumTopicReopened, Game,
        GameHighScore, GeneralForumTopicHidden, GeneralForumTopicUnhidden, Gift,
        GiftBackground, GiftInfo, Gifts, Giveaway, GiveawayCompleted, GiveawayCreated,
        GiveawayWinners, InaccessibleMessage, InlineKeyboardButton, InlineKeyboardMarkup,
        InlineQuery, InlineQueryResult, InlineQueryResultsButton, InputChecklist,
        InputChecklistTask, InputContactMessageContent, InputInvoiceMessageContent,
        InputLocationMessageContent, InputMedia, InputMessageContent,
        InputPaidMedia, InputPollMedia, InputPollOption, InputPollOptionMedia,
        InputProfilePhoto, InputRichBlock, InputRichBlockListItem, InputRichMessage,
        InputRichMessageContent, InputRichMessageMedia, InputSticker, InputStoryContent,
        InputTextMessageContent, InputVenueMessageContent, Invoice, KeyboardButton,
        KeyboardButtonPollType, KeyboardButtonRequestChat, KeyboardButtonRequestManagedBot,
        KeyboardButtonRequestUsers, LabeledPrice, Link, LinkPreviewOptions, LivePhoto,
        Location, LocationAddress, LoginUrl, ManagedBotCreated, ManagedBotUpdated,
        MaskPosition, MaybeInaccessibleMessage, MenuButton, Message,
        MessageAutoDeleteTimerChanged, MessageEntity, MessageId, MessageOrigin,
        MessageReactionCountUpdated, MessageReactionUpdated, OrderInfo, OwnedGift,
        OwnedGifts, PaidMedia, PaidMediaInfo, PaidMediaPurchased, PaidMessagePriceChanged,
        PassportData, PassportElementError, PassportFile, PhotoSize, Poll, PollAnswer,
        PollMedia, PollOption, PollOptionAdded, PollOptionDeleted, PreCheckoutQuery,
        PreparedInlineMessage, PreparedKeyboardButton, ProximityAlertTriggered,
        ReactionCount, ReactionType, RefundedPayment, ReplyKeyboardMarkup,
        ReplyKeyboardRemove, ReplyParameters, RevenueWithdrawalState, RichBlock,
        RichBlockCaption, RichBlockListItem, RichBlockTableCell, RichMessage, RichText,
        SentGuestMessage, SentWebAppMessage, SharedUser, ShippingAddress, ShippingOption,
        ShippingQuery, StarAmount, StarTransaction, StarTransactions, Sticker, StickerSet,
        Story, StoryArea, StoryAreaPosition, StoryAreaType, SuccessfulPayment,
        SuggestedPostApprovalFailed, SuggestedPostApproved, SuggestedPostDeclined,
        SuggestedPostInfo, SuggestedPostPaid, SuggestedPostParameters, SuggestedPostPrice,
        SuggestedPostRefunded, SwitchInlineQueryChosenChat, TextQuote, TransactionPartner,
        UniqueGift, UniqueGiftBackdrop, UniqueGiftBackdropColors, UniqueGiftColors,
        UniqueGiftInfo, UniqueGiftModel, UniqueGiftSymbol, Update, User, UserChatBoosts,
        UserProfileAudios, UserProfilePhotos, UserRating, UsersShared, Venue, Video,
        VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted,
        VideoNote, VideoQuality, Voice, WebAppData, WebAppInfo, WebhookInfo,
        WriteAccessAllowed,
    ],
    // Spec types that Rust models as inline variants of their parent enum.
    // Round-tripping through the parent is the stronger check: it proves the
    // discriminant actually selects this variant, which is the property that
    // failed when every `RichText` value decoded as `Bold`.
    via: [
        // The docs express this union inline, as the type of
        // `InputRichMessageMedia.media` ("InputMediaAnimation or ... or
        // InputMediaVoiceNote") rather than as a named union with a member
        // list, so the snapshot's union table cannot supply the parent.
        InputMediaVoiceNote => InputMedia,
        BackgroundFillFreeformGradient => BackgroundFill,
        BackgroundFillGradient => BackgroundFill,
        BackgroundFillSolid => BackgroundFill,
        BackgroundTypeChatTheme => BackgroundType,
        BackgroundTypeFill => BackgroundType,
        BackgroundTypePattern => BackgroundType,
        BackgroundTypeWallpaper => BackgroundType,
        BotCommandScopeAllChatAdministrators => BotCommandScope,
        BotCommandScopeAllGroupChats => BotCommandScope,
        BotCommandScopeAllPrivateChats => BotCommandScope,
        BotCommandScopeChat => BotCommandScope,
        BotCommandScopeChatAdministrators => BotCommandScope,
        BotCommandScopeChatMember => BotCommandScope,
        BotCommandScopeDefault => BotCommandScope,
        ChatBoostSourceGiftCode => ChatBoostSource,
        ChatBoostSourceGiveaway => ChatBoostSource,
        ChatBoostSourcePremium => ChatBoostSource,
        ChatMemberAdministrator => ChatMember,
        ChatMemberBanned => ChatMember,
        ChatMemberLeft => ChatMember,
        ChatMemberMember => ChatMember,
        ChatMemberOwner => ChatMember,
        ChatMemberRestricted => ChatMember,
        InlineQueryResultArticle => InlineQueryResult,
        InlineQueryResultAudio => InlineQueryResult,
        InlineQueryResultCachedAudio => InlineQueryResult,
        InlineQueryResultCachedDocument => InlineQueryResult,
        InlineQueryResultCachedGif => InlineQueryResult,
        InlineQueryResultCachedMpeg4Gif => InlineQueryResult,
        InlineQueryResultCachedPhoto => InlineQueryResult,
        InlineQueryResultCachedSticker => InlineQueryResult,
        InlineQueryResultCachedVideo => InlineQueryResult,
        InlineQueryResultCachedVoice => InlineQueryResult,
        InlineQueryResultContact => InlineQueryResult,
        InlineQueryResultDocument => InlineQueryResult,
        InlineQueryResultGame => InlineQueryResult,
        InlineQueryResultGif => InlineQueryResult,
        InlineQueryResultLocation => InlineQueryResult,
        InlineQueryResultMpeg4Gif => InlineQueryResult,
        InlineQueryResultPhoto => InlineQueryResult,
        InlineQueryResultVenue => InlineQueryResult,
        InlineQueryResultVideo => InlineQueryResult,
        InlineQueryResultVoice => InlineQueryResult,
        InputMediaAnimation => InputPollOptionMedia,
        InputMediaAudio => InputPollMedia,
        InputMediaDocument => InputPollMedia,
        InputMediaLink => InputPollOptionMedia,
        InputMediaLivePhoto => InputPollOptionMedia,
        InputMediaLocation => InputPollOptionMedia,
        InputMediaPhoto => InputPollOptionMedia,
        InputMediaSticker => InputPollOptionMedia,
        InputMediaVenue => InputPollOptionMedia,
        InputMediaVideo => InputPollOptionMedia,
        InputPaidMediaLivePhoto => InputPaidMedia,
        InputPaidMediaPhoto => InputPaidMedia,
        InputPaidMediaVideo => InputPaidMedia,
        InputProfilePhotoAnimated => InputProfilePhoto,
        InputProfilePhotoStatic => InputProfilePhoto,
        InputRichBlockAnchor => InputRichBlock,
        InputRichBlockAnimation => InputRichBlock,
        InputRichBlockAudio => InputRichBlock,
        InputRichBlockBlockQuotation => InputRichBlock,
        InputRichBlockCollage => InputRichBlock,
        InputRichBlockDetails => InputRichBlock,
        InputRichBlockDivider => InputRichBlock,
        InputRichBlockFooter => InputRichBlock,
        InputRichBlockList => InputRichBlock,
        InputRichBlockMap => InputRichBlock,
        InputRichBlockMathematicalExpression => InputRichBlock,
        InputRichBlockParagraph => InputRichBlock,
        InputRichBlockPhoto => InputRichBlock,
        InputRichBlockPreformatted => InputRichBlock,
        InputRichBlockPullQuotation => InputRichBlock,
        InputRichBlockSectionHeading => InputRichBlock,
        InputRichBlockSlideshow => InputRichBlock,
        InputRichBlockTable => InputRichBlock,
        InputRichBlockThinking => InputRichBlock,
        InputRichBlockVideo => InputRichBlock,
        InputRichBlockVoiceNote => InputRichBlock,
        InputStoryContentPhoto => InputStoryContent,
        InputStoryContentVideo => InputStoryContent,
        MenuButtonCommands => MenuButton,
        MenuButtonDefault => MenuButton,
        MenuButtonWebApp => MenuButton,
        MessageOriginChannel => MessageOrigin,
        MessageOriginChat => MessageOrigin,
        MessageOriginHiddenUser => MessageOrigin,
        MessageOriginUser => MessageOrigin,
        OwnedGiftRegular => OwnedGift,
        OwnedGiftUnique => OwnedGift,
        PaidMediaLivePhoto => PaidMedia,
        PaidMediaPhoto => PaidMedia,
        PaidMediaPreview => PaidMedia,
        PaidMediaVideo => PaidMedia,
        PassportElementErrorDataField => PassportElementError,
        PassportElementErrorFile => PassportElementError,
        PassportElementErrorFiles => PassportElementError,
        PassportElementErrorFrontSide => PassportElementError,
        PassportElementErrorReverseSide => PassportElementError,
        PassportElementErrorSelfie => PassportElementError,
        PassportElementErrorTranslationFile => PassportElementError,
        PassportElementErrorTranslationFiles => PassportElementError,
        PassportElementErrorUnspecified => PassportElementError,
        ReactionTypeCustomEmoji => ReactionType,
        ReactionTypeEmoji => ReactionType,
        ReactionTypePaid => ReactionType,
        RevenueWithdrawalStateFailed => RevenueWithdrawalState,
        RevenueWithdrawalStatePending => RevenueWithdrawalState,
        RevenueWithdrawalStateSucceeded => RevenueWithdrawalState,
        RichBlockAnchor => RichBlock,
        RichBlockAnimation => RichBlock,
        RichBlockAudio => RichBlock,
        RichBlockBlockQuotation => RichBlock,
        RichBlockCollage => RichBlock,
        RichBlockDetails => RichBlock,
        RichBlockDivider => RichBlock,
        RichBlockFooter => RichBlock,
        RichBlockList => RichBlock,
        RichBlockMap => RichBlock,
        RichBlockMathematicalExpression => RichBlock,
        RichBlockParagraph => RichBlock,
        RichBlockPhoto => RichBlock,
        RichBlockPreformatted => RichBlock,
        RichBlockPullQuotation => RichBlock,
        RichBlockSectionHeading => RichBlock,
        RichBlockSlideshow => RichBlock,
        RichBlockTable => RichBlock,
        RichBlockThinking => RichBlock,
        RichBlockVideo => RichBlock,
        RichBlockVoiceNote => RichBlock,
        RichTextAnchor => RichText,
        RichTextAnchorLink => RichText,
        RichTextBankCardNumber => RichText,
        RichTextBold => RichText,
        RichTextBotCommand => RichText,
        RichTextCashtag => RichText,
        RichTextCode => RichText,
        RichTextCustomEmoji => RichText,
        RichTextDateTime => RichText,
        RichTextEmailAddress => RichText,
        RichTextHashtag => RichText,
        RichTextItalic => RichText,
        RichTextMarked => RichText,
        RichTextMathematicalExpression => RichText,
        RichTextMention => RichText,
        RichTextPhoneNumber => RichText,
        RichTextReference => RichText,
        RichTextReferenceLink => RichText,
        RichTextSpoiler => RichText,
        RichTextStrikethrough => RichText,
        RichTextSubscript => RichText,
        RichTextSuperscript => RichText,
        RichTextTextMention => RichText,
        RichTextUnderline => RichText,
        RichTextUrl => RichText,
        StoryAreaTypeLink => StoryAreaType,
        StoryAreaTypeLocation => StoryAreaType,
        StoryAreaTypeSuggestedReaction => StoryAreaType,
        StoryAreaTypeUniqueGift => StoryAreaType,
        StoryAreaTypeWeather => StoryAreaType,
        TransactionPartnerAffiliateProgram => TransactionPartner,
        TransactionPartnerChat => TransactionPartner,
        TransactionPartnerFragment => TransactionPartner,
        TransactionPartnerOther => TransactionPartner,
        TransactionPartnerTelegramAds => TransactionPartner,
        TransactionPartnerTelegramApi => TransactionPartner,
        TransactionPartnerUser => TransactionPartner,
    ],
}

/// Spec types that cannot round trip through JSON, each with the reason.
///
/// Two entries, and both should stay uncomfortable to add to: an exclusion is
/// permission for a type to go unchecked forever.
pub const EXCLUDED: &[(&str, &str)] = &[
    (
        "InputFile",
        "Not a JSON object. It carries raw bytes for a multipart upload, so it \
         implements neither Serialize nor Deserialize by design. Its behaviour \
         is the transport tests' subject.",
    ),
    (
        "ResponseParameters",
        "Private to rustigram-api, where it is decoded as part of the error \
         envelope and surfaced through the typed error rather than returned to \
         callers. The coverage suite documents the same exception.",
    ),
];

