//! Every spec type survives a JSON round trip with its required fields intact.
//!
//! Coverage tests ask whether a field is *declared*. This one asks whether it
//! still exists after the value has been through serde in both directions —
//! which is a different question, and the one the field-level bugs in this
//! codebase were hiding behind:
//!
//! - `RichBlockMap` declared `latitude` and `longitude` where the spec has a
//!   nested `location` object. Every field was present and correctly typed; the
//!   type simply never decoded.
//! - `RichTextReference` sent its name under the wrong wire key. Declared,
//!   typed, covered, and invisible to Telegram.
//!
//! Both are round-trip failures and nothing else. A payload built from the spec
//! goes in, and the re-serialised value is checked for every field the spec
//! marks required — so a renamed key, a flattened object, or a variant that
//! decodes as its neighbour all surface as a named missing field.
//!
//! # Where the input values come from
//!
//! Constructing 386 values by hand would be its own source of bugs, and one
//! that ages badly. Instead each payload is generated from the snapshot's own
//! field table: required fields only, with a value chosen by the spec's declared
//! type and the discriminant filled in where the spec pins one. The generator is
//! deliberately minimal — it proves the shape is right, not that the crate
//! handles every value Telegram might send.

mod common;

use common::Spec;
use rustigram_types::*;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Map, Value, json};

// ─── Generating a payload from the spec ──────────────────────────────────────

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

/// Unions the spec also admits as a bare scalar, with the value to use.
///
/// `RichText` is documented as "either a String for plain text, an Array of
/// RichText, or any of the following types", and every one of those 25 types
/// requires a nested `RichText` of its own. A generator that always reaches for
/// an object member therefore never terminates — the string form is the base
/// case, and the spec's own first-listed one.
const SCALAR_UNIONS: [(&str, &str); 1] = [("RichText", "plain text")];

/// A minimal valid payload for `name`, built from its spec field table.
fn minimal(spec: &Spec, name: &str, depth: usize) -> Result<Value, String> {
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
            return minimal(spec, first, depth + 1);
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
        let needed = !spec_field.optional() || required_extra == Some(field.as_str());
        if !needed || object.contains_key(field) {
            continue;
        }
        // A field the docs give a fixed set of values for must receive one of
        // them. Any of the values is valid; the first is deterministic.
        let value = match spec.enum_values.get(&format!("{name}.{field}")) {
            Some(values) if spec_field.kind() == "String" => {
                let first = values
                    .first()
                    .ok_or_else(|| format!("{name}.{field} has an empty value list"))?;
                json!(first)
            }
            _ => value_for(spec, spec_field.kind(), depth + 1)
                .map_err(|why| format!("{name}.{field}: {why}"))?,
        };
        object.insert(field.clone(), value);
    }

    Ok(Value::Object(object))
}

/// A value matching the spec's own type text, e.g. `Integer` or `Array of User`.
fn value_for(spec: &Spec, kind: &str, depth: usize) -> Result<Value, String> {
    let kind = kind.trim();

    if let Some(element) = kind.strip_prefix("Array of ") {
        // Past the depth limit an empty array is still a valid value, so the
        // outer type can finish generating rather than failing wholesale.
        if depth > MAX_DEPTH {
            return Ok(json!([]));
        }
        return Ok(json!([value_for(spec, element, depth + 1)?]));
    }

    // `Integer or String`, `InputFile or String` — any alternative is valid, and
    // taking the first keeps the generated payload deterministic.
    if let Some((first, _)) = kind.split_once(" or ") {
        return value_for(spec, first, depth);
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
                return minimal(spec, first, depth + 1);
            }
            if spec.types.contains_key(kind) {
                return minimal(spec, kind, depth + 1);
            }
            Err(format!("no way to generate a `{kind}`"))
        }
    }
}

// ─── Round tripping ──────────────────────────────────────────────────────────

/// Deserialises `json` as `T` and serialises it back.
fn round_trip<T: DeserializeOwned + Serialize>(json: &Value) -> Result<Value, String> {
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
        fn round_trip_by_name(name: &str, json: &Value) -> Result<Value, String> {
            match name {
                $(stringify!($direct) => round_trip::<$direct>(json),)*
                $(stringify!($variant) => round_trip::<$parent>(json),)*
                _ => Err(format!("`{name}` has no entry in the dispatch table")),
            }
        }

        /// Every name the table can handle.
        const DISPATCHED: &[&str] = &[
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
const EXCLUDED: &[(&str, &str)] = &[
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

// ─── The tests ───────────────────────────────────────────────────────────────

/// Every spec type decodes a spec-shaped payload and re-encodes it with all of
/// its required fields still present under the names the spec gives them.
#[test]
fn every_spec_type_round_trips() {
    let spec = common::load();
    let mut failures = Vec::new();

    for (name, fields) in &spec.types {
        if EXCLUDED.iter().any(|(excluded, _)| excluded == name) {
            continue;
        }

        let payload = match minimal(&spec, name, 0) {
            Ok(payload) => payload,
            // A payload that cannot be built is reported, never skipped — a
            // generator that quietly gives up would shrink this suite's real
            // coverage while the test count stayed the same.
            Err(why) => {
                failures.push(format!("  {name}: could not build a payload — {why}"));
                continue;
            }
        };

        let encoded = match round_trip_by_name(name, &payload) {
            Ok(encoded) => encoded,
            Err(why) => {
                failures.push(format!("  {name}: {why}"));
                continue;
            }
        };

        let Some(object) = encoded.as_object() else {
            failures.push(format!(
                "  {name}: re-encoded as {encoded}, which is not a JSON object"
            ));
            continue;
        };

        let lost: Vec<&str> = fields
            .iter()
            .filter(|(field, spec_field)| {
                !spec_field.optional() && !object.contains_key(*field)
            })
            .map(|(field, _)| field.as_str())
            .collect();

        if !lost.is_empty() {
            failures.push(format!(
                "  {name}: required field(s) {lost:?} did not survive the round trip.\n\
                 \x20     sent:      {payload}\n\
                 \x20     came back: {encoded}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} spec types failed to round trip:\n{}",
        failures.len(),
        spec.types.len(),
        failures.join("\n")
    );
}

/// The dispatch table names every spec type exactly once, minus the exclusions.
///
/// This is what stops the table from rotting. A new Bot API type is covered the
/// moment someone adds it here, and until they do the suite is red rather than
/// quietly reporting green over a shrinking share of the spec.
#[test]
fn the_dispatch_table_covers_every_spec_type() {
    let spec = common::load();

    let missing: Vec<&str> = spec
        .types
        .keys()
        .map(String::as_str)
        .filter(|name| {
            !DISPATCHED.contains(name) && !EXCLUDED.iter().any(|(e, _)| e == name)
        })
        .collect();
    assert!(
        missing.is_empty(),
        "{} spec type(s) are in neither the dispatch table nor the exclusion \
         list, so nothing round-trips them:\n  {}",
        missing.len(),
        missing.join("\n  ")
    );

    let unknown: Vec<&&str> = DISPATCHED
        .iter()
        .filter(|name| !spec.types.contains_key(**name))
        .collect();
    assert!(
        unknown.is_empty(),
        "{} dispatch entr(ies) name a type the spec no longer has — remove \
         them:\n  {unknown:?}",
        unknown.len()
    );

    let mut sorted = DISPATCHED.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        sorted.len(),
        DISPATCHED.len(),
        "the dispatch table lists a type twice"
    );
}

/// Each exclusion still describes a type the spec has.
///
/// An exclusion for a type that no longer exists is dead permission, and the
/// next person to need one inherits it.
#[test]
fn every_exclusion_still_applies() {
    let spec = common::load();
    let stale: Vec<&str> = EXCLUDED
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| !spec.types.contains_key(*name))
        .collect();
    assert!(
        stale.is_empty(),
        "{} exclusion(s) name a type the spec no longer has — remove them: {stale:?}",
        stale.len()
    );
}

/// `RichBlockMap` carries a nested `location` object, not flat coordinates.
///
/// Pinned separately from the generated sweep because this is the exact shape
/// the type had when it could not decode at all, and a regression here should
/// name the field rather than appear in a list of 386.
#[test]
fn rich_block_map_keeps_its_nested_location() {
    let encoded = serde_json::to_value(rich_message::RichBlock::Map(
        serde_json::from_value(json!({
            "location": { "latitude": 41.0, "longitude": 29.0 },
            "zoom": 12, "width": 640, "height": 480
        }))
        .expect("a map block decodes from a nested location"),
    ))
    .expect("a map block re-encodes");

    assert!(
        encoded.get("location").is_some(),
        "the map block lost its nested `location` object; flattening it back to \
         `latitude`/`longitude` is what stopped this type decoding: {encoded}"
    );
    assert!(
        encoded.get("latitude").is_none(),
        "coordinates leaked back to the top level: {encoded}"
    );
}

/// `RichTextReference` sends its name under the wire key `name`.
///
/// The Rust field is called something else, so this depends entirely on a
/// `#[serde(rename)]` that is one deletion away from silently breaking.
#[test]
fn rich_text_reference_keeps_its_name_field() {
    let node: rich_message::RichTextNode =
        serde_json::from_value(json!({ "type": "reference", "text": "see also", "name": "rfc-1" }))
            .expect("a reference node decodes from the spec's wire shape");
    let encoded = serde_json::to_value(&node).expect("a reference node re-encodes");

    assert_eq!(
        encoded.get("name").and_then(Value::as_str),
        Some("rfc-1"),
        "the reference lost its `name` key — Telegram would not see it: {encoded}"
    );
}
