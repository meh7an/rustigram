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

use common::payload::{EXCLUDED, DISPATCHED, Fill, payload, round_trip_by_name};
use rustigram_types::rich_message;
use serde_json::{Value, json};

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

        let payload = match payload(&spec, name, Fill::Required, 0) {
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
