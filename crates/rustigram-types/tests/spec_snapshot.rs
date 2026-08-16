//! The spec snapshot itself is sound and current.
//!
//! These tests guard the foundation every other conformance test stands on. If
//! the snapshot is stale or empty, the rest of the suite passes while checking
//! nothing, so these run first in spirit even though cargo orders them
//! alphabetically.

mod common;

/// The snapshot parses and contains what the Bot API 10.2 mirror reported.
///
/// The exact counts are asserted rather than a lower bound: a regenerated
/// snapshot that loses entries should fail here loudly, not shrink quietly.
#[test]
fn snapshot_loads_with_the_expected_shape() {
    let spec = common::load();

    assert_eq!(
        spec.snapshot_version, "1",
        "snapshot schema version changed"
    );
    assert_eq!(spec.bot_api_version, "10.2");
    assert_eq!(spec.types.len(), 388, "spec type count");
    assert_eq!(spec.methods.len(), 185, "spec method count");
    assert_eq!(spec.discriminants.len(), 168, "discriminant count");
    assert_eq!(spec.unions.len(), 26, "union count");
}

/// Spot-checks that the snapshot carries real detail, not just names.
///
/// A generator bug that emitted every type with an empty field map would still
/// satisfy the counts above while making every field-level test vacuous.
#[test]
fn snapshot_carries_field_level_detail() {
    let spec = common::load();

    let message = spec.types.get("Message").expect("Message in spec");
    assert!(
        message.len() > 100,
        "Message should have 100+ fields, found {}",
        message.len()
    );
    assert!(!message["message_id"].optional(), "message_id is required");
    assert!(message["text"].optional(), "text is optional");

    let send_message = spec
        .methods
        .get("sendMessage")
        .expect("sendMessage in spec");
    assert!(!send_message["chat_id"].optional());
    assert!(send_message["parse_mode"].optional());

    // Discriminants are what the tagged-enum conformance tests rely on.
    assert_eq!(spec.discriminants["RichBlockSectionHeading"], "heading");
    assert_eq!(spec.discriminants["PaidMediaPhoto"], "photo");

    // Unions drive the untagged-ordering tests.
    assert!(spec.unions["PaidMedia"].contains(&"PaidMediaVideo".to_owned()));

    // `Array of X` must be reachable as `X` for reference checks.
    let invited = &spec.types["VideoChatParticipantsInvited"]["users"];
    assert_eq!(invited.kind(), "Array of User");
    assert_eq!(invited.base_type(), "User");
}

/// The snapshot must describe the same Bot API version the crate advertises.
///
/// This is the staleness guard. Without it, a Bot API bump would leave the
/// conformance suite silently validating against the previous version — passing
/// for the wrong reason, which is the failure mode this suite exists to
/// prevent.
#[test]
fn snapshot_version_matches_what_the_crate_claims() {
    let spec = common::load();
    let readme = include_str!("../../../README.md");

    // The README writes it as "10.2 (July 2026)"; only the version compares.
    let claimed = readme
        .lines()
        .find_map(|l| l.strip_prefix("**Supported Bot API version:**"))
        .and_then(|rest| rest.split_whitespace().next())
        .expect("README states a supported Bot API version");

    assert_eq!(
        claimed, spec.bot_api_version,
        "the README claims Bot API {claimed} but the spec snapshot is {}. \
         Regenerate the snapshot after a version bump, or the conformance tests \
         validate against the wrong spec.",
        spec.bot_api_version
    );
}
