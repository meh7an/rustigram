//! Untagged enums resolve each payload to the variant it belongs to.
//!
//! This is the class of bug that made every `RichText` value deserialize as
//! `Bold`, and made every venue deserialize as a `Location` and lose its title
//! and address. It is invisible to every other check in this suite: field names,
//! field types, optionality, and discriminants are all correct. The value simply
//! arrives as the wrong variant.
//!
//! # The property, and the proxy for it
//!
//! `#[serde(untagged)]` takes the first variant that matches, and serde ignores
//! fields a variant does not declare. So a variant whose required fields are a
//! *subset* of a later one's will swallow it — `Location{latitude, longitude}`
//! declared before `Venue{latitude, longitude, title, address}` means no venue
//! can ever be produced.
//!
//! Declaration order is the usual way to describe that rule, but order is only a
//! proxy. The real property is that a payload belonging to a variant produces
//! that variant, so that is what these tests assert directly: one payload per
//! variant, checked against the variant it must yield.

mod common;

use rustigram_types::inline::InputMessageContent;
use rustigram_types::keyboard::ReplyMarkup;
use rustigram_types::message::MaybeInaccessibleMessage;
use rustigram_types::rich_message::RichText;
use rustigram_types::user::ChatId;

/// The variant name from a `Debug` rendering, e.g. `Venue(..)` -> `Venue`.
fn variant_of<T: std::fmt::Debug>(value: &T) -> String {
    format!("{value:?}")
        .split(['(', ' ', '{'])
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// Asserts `json` deserializes as `T` and lands on `expected`.
fn resolves_to<T>(json: &str, expected: &str)
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let parsed: T = serde_json::from_str(json)
        .unwrap_or_else(|e| panic!("{expected}: payload did not deserialize: {e}\n  {json}"));
    let got = variant_of(&parsed);
    assert_eq!(
        got, expected,
        "payload for `{expected}` resolved to `{got}` instead\n  {json}"
    );
}

/// `InputMessageContent` — the venue-swallowed-by-location case.
///
/// `Venue` requires everything `Location` requires plus `title` and `address`,
/// so it must be tried first. Every variant is listed, and
/// [`input_message_content_table_is_complete`] enforces that.
#[test]
fn input_message_content_resolves_every_variant() {
    for (expected, json) in INPUT_MESSAGE_CONTENT_CASES {
        resolves_to::<InputMessageContent>(json, expected);
    }
}

const INPUT_MESSAGE_CONTENT_CASES: &[(&str, &str)] = &[
    ("Text", r#"{"message_text":"hello"}"#),
    (
        "Rich",
        r#"{"rich_message":{"blocks":[{"type":"paragraph","text":"hi"}]}}"#,
    ),
    (
        "Venue",
        r#"{"latitude":41.0,"longitude":29.0,"title":"Blue Mosque","address":"Sultanahmet"}"#,
    ),
    ("Location", r#"{"latitude":41.0,"longitude":29.0}"#),
    ("Contact", r#"{"phone_number":"+100","first_name":"A"}"#),
    (
        "Invoice",
        r#"{"title":"T","description":"D","payload":"p","currency":"XTR",
            "prices":[{"label":"l","amount":1}]}"#,
    ),
];

/// The venue case in detail: the fields that used to be lost must survive.
///
/// A resolution to `Location` still deserializes successfully — it just drops
/// `title` and `address` silently, which is why the variant assertion above is
/// not enough on its own to describe the damage.
#[test]
fn a_venue_keeps_its_title_and_address() {
    let parsed: InputMessageContent = serde_json::from_str(
        r#"{"latitude":41.0,"longitude":29.0,"title":"Blue Mosque","address":"Sultanahmet"}"#,
    )
    .unwrap();
    match parsed {
        InputMessageContent::Venue(v) => {
            assert_eq!(v.title, "Blue Mosque");
            assert_eq!(v.address, "Sultanahmet");
        }
        other => panic!(
            "a venue became {}, losing its title and address",
            variant_of(&other)
        ),
    }
}

/// Every member the spec lists for `InputMessageContent` has a case above.
///
/// Without this the table rots: a Bot API version adds a content type, nobody
/// adds a case, and the suite reports green over a shrinking share of the enum.
#[test]
fn input_message_content_table_is_complete() {
    let spec = common::load();
    let members = &spec.unions["InputMessageContent"];
    assert_eq!(
        INPUT_MESSAGE_CONTENT_CASES.len(),
        members.len(),
        "the spec lists {} InputMessageContent members but the table has {} cases: {members:?}",
        members.len(),
        INPUT_MESSAGE_CONTENT_CASES.len()
    );
}

/// `RichText` distinguishes three shapes, not twenty-five kinds.
///
/// The kinds live in `RichTextNode`, which is tagged and covered by the
/// discriminant tests. What this enum must get right is telling a bare string
/// from an array from an object — the only job left to `untagged` after the
/// split, and the reason the split was made.
#[test]
fn rich_text_resolves_its_three_shapes() {
    resolves_to::<RichText>(r#""just text""#, "Plain");
    resolves_to::<RichText>(r#"["a","b"]"#, "Array");
    resolves_to::<RichText>(r#"{"type":"bold","text":"hi"}"#, "Node");
}

/// Nested rich text must resolve at every level, not just the outermost.
#[test]
fn rich_text_resolves_when_nested() {
    let parsed: RichText =
        serde_json::from_str(r#"{"type":"bold","text":{"type":"italic","text":"deep"}}"#).unwrap();
    let RichText::Node(outer) = parsed else {
        panic!("outer value is not a node")
    };
    let rustigram_types::rich_message::RichTextNode::Bold(bold) = *outer else {
        panic!("outer node is not bold")
    };
    assert!(
        matches!(bold.text, RichText::Node(_)),
        "the nested value should be a node, got {:?}",
        variant_of(&bold.text)
    );
}

/// `MaybeInaccessibleMessage` cannot be resolved by shape at all.
///
/// An inaccessible message is a strict subset of a message, so `untagged` would
/// always pick whichever came first. Telegram's actual rule is `date == 0`,
/// which the manual `Deserialize` implements — this pins that behaviour.
#[test]
fn maybe_inaccessible_message_resolves_on_the_date_field() {
    resolves_to::<MaybeInaccessibleMessage>(
        r#"{"chat":{"id":1,"type":"private"},"message_id":5,"date":0}"#,
        "Inaccessible",
    );
    resolves_to::<MaybeInaccessibleMessage>(
        r#"{"chat":{"id":1,"type":"private"},"message_id":5,"date":1700000000,"text":"hi"}"#,
        "Message",
    );
}

/// `ChatId` separates a number from a string — distinct JSON types, so untagged
/// is safe here, but the property is still worth pinning.
#[test]
fn chat_id_resolves_numbers_and_usernames() {
    resolves_to::<ChatId>("-1001234567890", "Id");
    resolves_to::<ChatId>(r#""@rustigram""#, "Username");
}

/// `ReplyMarkup`'s four variants each have a distinct required field, so none
/// can shadow another. Pinned because adding a variant with an all-optional
/// body would break that silently.
#[test]
fn reply_markup_resolves_every_variant() {
    resolves_to::<ReplyMarkup>(r#"{"inline_keyboard":[]}"#, "InlineKeyboard");
    resolves_to::<ReplyMarkup>(r#"{"keyboard":[]}"#, "ReplyKeyboard");
    resolves_to::<ReplyMarkup>(r#"{"remove_keyboard":true}"#, "Remove");
    resolves_to::<ReplyMarkup>(r#"{"force_reply":true}"#, "ForceReply");
}

/// Every untagged enum in the crate is covered by a test above.
///
/// Hand-listed, because Rust cannot enumerate its own types at runtime. The
/// spec-driven half is checked by [`input_message_content_table_is_complete`];
/// this is the reminder for the two enums that are rustigram's own conveniences
/// rather than Bot API unions.
#[test]
fn every_untagged_enum_in_the_crate_is_tested() {
    // Kept in sync by hand. If you add `#[serde(untagged)]` to an enum, add it
    // here and give it a resolution test above.
    const TESTED: &[&str] = &[
        "InputMessageContent",
        "RichText",
        "MaybeInaccessibleMessage",
        "ChatId",
        "ReplyMarkup",
    ];

    let spec = common::load();
    // The spec's own untagged unions must all appear.
    for union in [
        "InputMessageContent",
        "RichText",
        "MaybeInaccessibleMessage",
    ] {
        assert!(
            spec.unions.contains_key(union),
            "`{union}` is no longer a spec union; revisit this list"
        );
        assert!(TESTED.contains(&union), "`{union}` has no resolution test");
    }
    assert_eq!(
        TESTED.len(),
        5,
        "update this count when adding an untagged enum"
    );
}
