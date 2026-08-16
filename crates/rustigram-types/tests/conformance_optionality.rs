//! A field the spec marks optional can be absent, and one it defines can be present.
//!
//! Telegram adds fields to existing objects constantly, and it serves a mixed
//! fleet — an older server omits a field a newer one always sends. So the two
//! halves of this property are both load-bearing, and they fail in opposite
//! directions:
//!
//! - **Absence must decode.** A spec-optional field modelled as a bare required
//!   Rust type turns a routine Telegram deployment into a decode error for every
//!   update carrying that object. `ChatFullInfo::accent_color_id` is the shape
//!   of it: a plain `u32` that any server predating the field would break.
//! - **Presence must decode.** A required-fields-only payload never exercises an
//!   optional field's *type*, so a wrong one — `String` where the spec says
//!   `Integer`, a flat pair where the spec nests — sits undetected until a user
//!   hits the one update that includes it.
//!
//! The round-trip suite proves the first half in bulk: every payload it builds
//! carries required fields only, so all 386 types already decode with every
//! optional field missing. What is left is the second half, which this sweep
//! covers, and the structural rule behind the first, which no payload can prove
//! because a field that is merely *never sent* looks identical to one that
//! cannot be absent.

mod common;

use common::payload::{payload, round_trip_by_name, Fill, DISPATCHED, EXCLUDED};
use common::rust_source::parse_items;
use serde_json::json;
use std::collections::BTreeSet;

/// Every spec type decodes a payload carrying all of its optional fields.
///
/// The complement of the round-trip sweep. A type passes there by decoding its
/// required fields and passes here by decoding the rest, and only both together
/// say the declared shape matches the documented one.
#[test]
fn every_spec_type_decodes_with_every_optional_field_present() {
    let spec = common::load();
    let mut failures = Vec::new();

    for name in spec.types.keys() {
        if EXCLUDED.iter().any(|(excluded, _)| excluded == name) {
            continue;
        }
        let full = match payload(&spec, name, Fill::All, 0) {
            Ok(full) => full,
            Err(why) => {
                failures.push(format!("  {name}: could not build a payload — {why}"));
                continue;
            }
        };
        if let Err(why) = round_trip_by_name(name, &full) {
            failures.push(format!("  {name}: {why}"));
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} spec types rejected a payload containing every field the spec \
         lists for them:\n{}",
        failures.len(),
        spec.types.len(),
        failures.join("\n")
    );
}

/// The maximal payloads really do carry the optional fields.
///
/// An optional field whose value cannot be generated is dropped rather than
/// failing the payload — correct, since that is what optional means, but it is
/// also the way this sweep could quietly decay into the required-only one and
/// go on reporting success over half the surface. This pins the gap between
/// them, so a generator change that starts dropping fields wholesale is a
/// failure rather than a silent loss of coverage.
#[test]
fn the_maximal_payloads_carry_far_more_than_the_required_ones() {
    let spec = common::load();
    let (mut required, mut all) = (0_usize, 0_usize);

    for name in spec.types.keys() {
        for (fill, total) in [(Fill::Required, &mut required), (Fill::All, &mut all)] {
            if let Ok(value) = payload(&spec, name, fill, 0) {
                *total += value.as_object().map_or(0, serde_json::Map::len);
            }
        }
    }

    assert!(
        all > required * 2,
        "maximal payloads carry {all} fields against the required-only {required}; \
         they are supposed to add every optional field on top, so this sweep is \
         no longer testing much more than the round-trip one"
    );
}

/// No spec-optional field is a bare required Rust type.
///
/// A field is safely absent if it is an `Option`, or if it carries
/// `#[serde(default)]` — both let serde build the value when the key is missing.
/// Anything else is a decode error waiting for the first server that omits it.
///
/// This is a structural check because no payload can make the distinction: a
/// field that is simply never sent and a field that *cannot* be absent produce
/// exactly the same passing test.
#[test]
fn no_spec_optional_field_is_required_in_rust() {
    let spec = common::load();
    let items = parse_items();
    let mut brittle = Vec::new();

    for (name, fields) in &spec.types {
        let Some(item) = items.get(name) else {
            continue; // covered by the coverage suite, not this one
        };
        // A tagged enum's variants are separate items; the parent carries no
        // fields of its own, so there is nothing here to check.
        if item.is_enum && item.fields.is_empty() {
            continue;
        }
        for (field_name, spec_field) in fields {
            if !spec_field.optional() {
                continue;
            }
            let Some(field) = item.fields.get(field_name) else {
                continue; // a missing field is the coverage suite's finding
            };
            let optional_in_rust = field.ty.starts_with("Option<")
                || field.ty.contains("Option <")
                || field.has_default
                || field.flattened;
            if !optional_in_rust {
                brittle.push(format!(
                    "  {name}.{field_name}: spec says optional, Rust has `{}` \
                     with no #[serde(default)]",
                    field.ty
                ));
            }
        }
    }

    assert!(
        brittle.is_empty(),
        "{} spec-optional field(s) cannot be absent from the wire. Any server \
         that omits one fails to decode the whole object — make it an `Option` \
         or give it `#[serde(default)]`:\n{}",
        brittle.len(),
        brittle.join("\n")
    );
}

/// The scan above must actually be looking at the types it claims to.
///
/// Without this the test passes just as happily when `parse_items` matches
/// nothing — the most dangerous way for a structural check to fail, because it
/// reports success while checking an empty set.
#[test]
fn the_optionality_scan_reaches_the_spec_types() {
    let spec = common::load();
    let items = parse_items();

    let seen: BTreeSet<&String> = spec
        .types
        .keys()
        .filter(|name| items.contains_key(*name))
        .collect();
    assert!(
        seen.len() > 300,
        "the scan found only {} of the {} spec types in the source tree; it \
         would report success while checking almost nothing",
        seen.len(),
        spec.types.len()
    );

    let checked: usize = spec
        .types
        .iter()
        .filter_map(|(name, fields)| items.get(name).map(|item| (fields, item)))
        .map(|(fields, item)| {
            fields
                .iter()
                .filter(|(f, sf)| sf.optional() && item.fields.contains_key(*f))
                .count()
        })
        .sum();
    assert!(
        checked > 900,
        "only {checked} optional fields were reachable; expected the great \
         majority of the spec's optional fields"
    );
}

/// A `ChatMemberAdministrator` with none of its optional rights decodes.
///
/// Pinned because this exact object was once reported as a decode failure
/// during an audit, on the strength of its fields not being `Option`. They carry
/// `#[serde(default)]` instead, which is equally safe — a distinction a reader
/// of the type signature alone will get wrong, and a test will not.
#[test]
fn a_chat_member_administrator_decodes_without_any_optional_rights() {
    let admin: rustigram_types::chat_member::ChatMember = serde_json::from_value(json!({
        "status": "administrator",
        "user": { "id": 1, "is_bot": false, "first_name": "A" },
        "can_be_edited": false,
        "is_anonymous": false,
        "can_manage_chat": true,
        "can_delete_messages": false,
        "can_manage_video_chats": false,
        "can_restrict_members": false,
        "can_promote_members": false,
        "can_change_info": false,
        "can_invite_users": false
    }))
    .expect("an administrator with no optional rights decodes");

    let rustigram_types::chat_member::ChatMember::Administrator(admin) = admin else {
        panic!("the status discriminant selected the wrong variant");
    };
    assert!(
        !admin.can_post_messages,
        "an absent right must default to not granted, never to granted"
    );
}

/// A `ChatFullInfo` from a server predating `accent_color_id` decodes.
///
/// Note that the spec marks `accent_color_id` **required**, so
/// [`no_spec_optional_field_is_required_in_rust`] does not cover it and cannot:
/// the `#[serde(default)]` on it is deliberately defensive, guarding against
/// servers that predate the field rather than satisfying the documented shape.
/// That makes it exactly the kind of attribute a later reader tidies away as
/// redundant, and this test is the thing that objects.
#[test]
fn a_chat_full_info_decodes_without_accent_color_id() {
    let chat: rustigram_types::chat::ChatFullInfo = serde_json::from_value(json!({
        "id": -100,
        "type": "supergroup",
        "max_reaction_count": 11
    }))
    .expect("a ChatFullInfo omitting accent_color_id decodes");

    assert_eq!(
        chat.accent_color_id, 0,
        "an absent accent colour must fall back to the default, not to garbage"
    );
}

/// Every type the optionality sweep covers is one the dispatch table knows.
///
/// Keeps this file honest about its own scope: it inherits the round-trip
/// suite's table, so a type missing there is unchecked here too, and that should
/// be visible rather than implied.
#[test]
fn the_sweep_covers_the_whole_dispatch_table() {
    let spec = common::load();
    let uncovered: Vec<&String> = spec
        .types
        .keys()
        .filter(|name| {
            !DISPATCHED.contains(&name.as_str())
                && !EXCLUDED.iter().any(|(e, _)| *e == name.as_str())
        })
        .collect();
    assert!(
        uncovered.is_empty(),
        "{} spec type(s) are outside the dispatch table, so this sweep silently \
         skips them: {uncovered:?}",
        uncovered.len()
    );
}
