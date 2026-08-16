//! Every tagged enum accepts the discriminant Telegram actually sends.
//!
//! This is the check that found the `RichText`-collapses-to-`Bold` bug, made
//! permanent. It is invisible to coverage: field names, field types, and
//! optionality can all be correct while the wire discriminant is wrong, and the
//! result is a silently mis-typed value rather than an error.
//!
//! # How it tests, and why not by reading the source
//!
//! The first version of this check parsed the Rust source for `#[serde(rename)]`
//! attributes — and got it wrong, reporting five mismatches on correct code
//! because it ignored renames that were already there. Acting on that output
//! would have turned working code into a bug.
//!
//! So this tests behaviour instead. For each union member it feeds serde an
//! object carrying only that member's discriminant and asserts the error is not
//! `unknown variant`. A recognised discriminant fails later, on a missing field;
//! an unrecognised one fails immediately at the tag. That distinction is the
//! entire property, and it needs no fixture data at all.

mod common;

use rustigram_types::background::{BackgroundFill, BackgroundType};
use rustigram_types::chat_member::ChatMember;
use rustigram_types::file::{InputMedia, InputPaidMedia, InputProfilePhoto};
use rustigram_types::inline::InlineQueryResult;
use rustigram_types::keyboard::MenuButton;
use rustigram_types::message::{MessageOrigin, ReactionType};
use rustigram_types::passport::PassportElementError;
use rustigram_types::payments::{OwnedGift, PaidMedia, RevenueWithdrawalState, TransactionPartner};
use rustigram_types::poll::{InputPollMedia, InputPollOptionMedia};
use rustigram_types::rich_message::{InputRichBlock, RichBlock};
use rustigram_types::story::InputStoryContent;
use rustigram_types::user::BotCommandScope;

/// Attempts to deserialize `json` as `T` and reports only whether serde
/// recognised the discriminant.
type TryTag = fn(&str) -> Result<(), String>;

fn probe<T: serde::de::DeserializeOwned>(json: &str) -> Result<(), String> {
    match serde_json::from_str::<T>(json) {
        Ok(_) => Ok(()),
        Err(e) => {
            let message = e.to_string();
            // `unknown variant` means the tag itself was rejected. Anything
            // else — a missing field, a type error — means the tag resolved and
            // deserialization got as far as the payload, which is all this test
            // claims to check.
            if message.contains("unknown variant") {
                Err(message)
            } else {
                Ok(())
            }
        }
    }
}

/// Every tagged union, paired with the tag field it uses.
///
/// Hand-written, because Rust cannot dispatch `from_str::<T>` on a runtime type
/// name — but [`table_covers_every_tagged_union_in_the_spec`] asserts the list
/// is complete, so a union added to the Bot API cannot quietly go untested.
const TAGGED: &[(&str, &str, TryTag)] = &[
    ("BackgroundFill", "type", probe::<BackgroundFill>),
    ("BackgroundType", "type", probe::<BackgroundType>),
    ("BotCommandScope", "type", probe::<BotCommandScope>),
    (
        "ChatBoostSource",
        "source",
        probe::<rustigram_types::update::ChatBoostSource>,
    ),
    ("ChatMember", "status", probe::<ChatMember>),
    ("InlineQueryResult", "type", probe::<InlineQueryResult>),
    ("InputMedia", "type", probe::<InputMedia>),
    ("InputPaidMedia", "type", probe::<InputPaidMedia>),
    ("InputPollMedia", "type", probe::<InputPollMedia>),
    (
        "InputPollOptionMedia",
        "type",
        probe::<InputPollOptionMedia>,
    ),
    ("InputProfilePhoto", "type", probe::<InputProfilePhoto>),
    ("InputRichBlock", "type", probe::<InputRichBlock>),
    ("InputStoryContent", "type", probe::<InputStoryContent>),
    ("MenuButton", "type", probe::<MenuButton>),
    ("MessageOrigin", "type", probe::<MessageOrigin>),
    ("OwnedGift", "type", probe::<OwnedGift>),
    ("PaidMedia", "type", probe::<PaidMedia>),
    (
        "PassportElementError",
        "source",
        probe::<PassportElementError>,
    ),
    ("ReactionType", "type", probe::<ReactionType>),
    (
        "RevenueWithdrawalState",
        "type",
        probe::<RevenueWithdrawalState>,
    ),
    ("RichBlock", "type", probe::<RichBlock>),
    (
        "StoryAreaType",
        "type",
        probe::<rustigram_types::story::StoryAreaType>,
    ),
    ("TransactionPartner", "type", probe::<TransactionPartner>),
];

/// The three unions modelled as untagged enums. They carry no discriminant to
/// check; the variant-ordering tests cover them instead.
const UNTAGGED: &[&str] = &[
    "InputMessageContent",
    "MaybeInaccessibleMessage",
    "RichText",
];

/// Every discriminant in the spec is accepted by the enum that should accept it.
#[test]
fn every_discriminant_is_recognised() {
    let spec = common::load();
    let mut checked = 0;
    let mut failures = Vec::new();

    for (union_name, tag, try_tag) in TAGGED {
        let members = spec
            .unions
            .get(*union_name)
            .unwrap_or_else(|| panic!("`{union_name}` is in the test table but not in the spec"));

        for member in members {
            let discriminant = spec.discriminants.get(member).unwrap_or_else(|| {
                panic!("spec has no discriminant for `{member}` (member of `{union_name}`)")
            });
            let json = format!(r#"{{"{tag}":"{discriminant}"}}"#);
            checked += 1;
            if let Err(e) = try_tag(&json) {
                failures.push(format!(
                    "  {union_name}: Telegram sends {tag}=\"{discriminant}\" for {member}, \
                     but the enum rejects it\n      serde said: {e}"
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {checked} discriminants are not accepted:\n{}",
        failures.len(),
        failures.join("\n")
    );
    assert!(
        checked > 100,
        "expected 100+ discriminants, checked {checked}"
    );
}

/// The hand-written table must list every union the spec defines.
///
/// Without this the table silently rots: a Bot API version adds a union, nobody
/// adds it here, and the suite keeps reporting green over a shrinking fraction
/// of the surface.
#[test]
fn table_covers_every_tagged_union_in_the_spec() {
    let spec = common::load();
    let listed: Vec<&str> = TAGGED
        .iter()
        .map(|(n, _, _)| *n)
        .chain(UNTAGGED.iter().copied())
        .collect();

    let missing: Vec<&String> = spec
        .unions
        .keys()
        .filter(|u| !listed.contains(&u.as_str()))
        .collect();

    assert!(
        missing.is_empty(),
        "these spec unions are in neither TAGGED nor UNTAGGED, so nothing tests \
         their discriminants: {missing:?}"
    );
    assert_eq!(
        listed.len(),
        spec.unions.len(),
        "the table lists {} unions but the spec has {}",
        listed.len(),
        spec.unions.len()
    );
}

/// The probe must actually reject a wrong discriminant.
///
/// A probe that returned `Ok` for everything would make the test above pass
/// unconditionally — the exact failure mode this suite exists to prevent.
#[test]
fn the_probe_rejects_an_unknown_discriminant() {
    let bogus = probe::<RichBlock>(r#"{"type":"definitely_not_a_block"}"#);
    assert!(
        bogus.is_err(),
        "the probe accepted a discriminant that does not exist"
    );
    assert!(bogus.unwrap_err().contains("unknown variant"));

    // And a real one is accepted, so the probe is not simply always failing.
    assert!(probe::<RichBlock>(r#"{"type":"heading"}"#).is_ok());
}

/// The abbreviated `RichBlock` variants are the ones most likely to be
/// "corrected" by someone who has not checked the spec.
#[test]
fn abbreviated_rich_block_variants_match_the_spec() {
    let spec = common::load();
    for (member, expected) in [
        ("RichBlockSectionHeading", "heading"),
        ("RichBlockPreformatted", "pre"),
        ("RichBlockBlockQuotation", "blockquote"),
        ("RichBlockPullQuotation", "pullquote"),
    ] {
        assert_eq!(
            spec.discriminants.get(member).map(String::as_str),
            Some(expected),
            "spec discriminant for {member}"
        );
        assert!(
            probe::<RichBlock>(&format!(r#"{{"type":"{expected}"}}"#)).is_ok(),
            "RichBlock rejects {expected}, which is what Telegram sends for {member}"
        );
    }
}

/// `InlineQueryResult` is the one union where a discriminant does not identify a
/// variant on its own.
///
/// Telegram gives a cached result the same `type` as its non-cached counterpart
/// and distinguishes them by `*_file_id` versus `*_url`. rustigram used to emit
/// `cached_photo`, `cached_sticker` and so on, which Telegram rejects — so every
/// cached inline result failed to send. These tests pin both directions.
mod inline_query_result {
    use rustigram_types::inline::InlineQueryResult;
    use serde_json::json;

    /// Each cached variant must serialise with the spec's discriminant, not a
    /// `cached_` prefix Telegram has never heard of.
    #[test]
    fn cached_variants_serialise_with_the_spec_discriminant() {
        let spec = super::common::load();
        let cases: &[(&str, serde_json::Value)] = &[
            (
                "InlineQueryResultCachedPhoto",
                json!({"type":"photo","id":"1","photo_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedGif",
                json!({"type":"gif","id":"1","gif_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedMpeg4Gif",
                json!({"type":"mpeg4_gif","id":"1","mpeg4_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedSticker",
                json!({"type":"sticker","id":"1","sticker_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedDocument",
                json!({"type":"document","id":"1","title":"t","document_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedVideo",
                json!({"type":"video","id":"1","title":"t","video_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedVoice",
                json!({"type":"voice","id":"1","title":"t","voice_file_id":"f"}),
            ),
            (
                "InlineQueryResultCachedAudio",
                json!({"type":"audio","id":"1","audio_file_id":"f"}),
            ),
        ];

        for (type_name, payload) in cases {
            let expected = &spec.discriminants[*type_name];
            let parsed: InlineQueryResult = serde_json::from_value(payload.clone())
                .unwrap_or_else(|e| panic!("{type_name}: {e}\n  {payload}"));
            let emitted = serde_json::to_value(&parsed).unwrap();
            assert_eq!(
                emitted["type"], **expected,
                "{type_name} must send type=\"{expected}\"; Telegram rejects anything else"
            );
        }
    }

    /// The `*_file_id` field, not the tag, decides cached versus non-cached.
    #[test]
    fn a_shared_discriminant_resolves_by_the_file_id_field() {
        let cached: InlineQueryResult =
            serde_json::from_value(json!({"type":"photo","id":"1","photo_file_id":"f"})).unwrap();
        assert!(
            matches!(cached, InlineQueryResult::CachedPhoto(_)),
            "photo + photo_file_id is a cached result, got {cached:?}"
        );

        let linked: InlineQueryResult = serde_json::from_value(
            json!({"type":"photo","id":"1","photo_url":"https://e.x/p.jpg","thumbnail_url":"https://e.x/t.jpg"}),
        )
        .unwrap();
        assert!(
            matches!(linked, InlineQueryResult::Photo(_)),
            "photo + photo_url is a URL result, got {linked:?}"
        );
    }

    /// Every variant must survive a round trip as itself — the property a
    /// derived tagged enum would silently break for all seven cached pairs.
    #[test]
    fn every_variant_round_trips_to_itself() {
        let cases: &[(&str, serde_json::Value)] = &[
            (
                "Article",
                json!({"type":"article","id":"1","title":"t","input_message_content":{"message_text":"m"}}),
            ),
            (
                "Photo",
                json!({"type":"photo","id":"1","photo_url":"u","thumbnail_url":"t"}),
            ),
            (
                "CachedPhoto",
                json!({"type":"photo","id":"1","photo_file_id":"f"}),
            ),
            (
                "Gif",
                json!({"type":"gif","id":"1","gif_url":"u","thumbnail_url":"t"}),
            ),
            (
                "CachedGif",
                json!({"type":"gif","id":"1","gif_file_id":"f"}),
            ),
            (
                "Voice",
                json!({"type":"voice","id":"1","voice_url":"u","title":"t"}),
            ),
            (
                "CachedVoice",
                json!({"type":"voice","id":"1","voice_file_id":"f","title":"t"}),
            ),
            (
                "CachedSticker",
                json!({"type":"sticker","id":"1","sticker_file_id":"f"}),
            ),
        ];
        for (variant, payload) in cases {
            let first: InlineQueryResult = serde_json::from_value(payload.clone())
                .unwrap_or_else(|e| panic!("{variant}: {e}"));
            let name = format!("{first:?}");
            let name = name.split(['(', ' ']).next().unwrap_or_default();
            assert_eq!(&name, variant, "{payload} resolved to the wrong variant");

            let again: InlineQueryResult =
                serde_json::from_value(serde_json::to_value(&first).unwrap()).unwrap();
            let again_name = format!("{again:?}");
            assert_eq!(
                &again_name.split(['(', ' ']).next().unwrap_or_default(),
                variant,
                "{variant} changed variant on the second trip"
            );
        }
    }

    /// An unrecognised tag must still be an error, phrased so the conformance
    /// probe above keeps recognising it.
    #[test]
    fn an_unknown_type_is_rejected() {
        let err = serde_json::from_value::<InlineQueryResult>(json!({"type":"hologram","id":"1"}))
            .expect_err("an unknown type must not deserialize");
        assert!(err.to_string().contains("unknown variant"), "got: {err}");
    }
}
