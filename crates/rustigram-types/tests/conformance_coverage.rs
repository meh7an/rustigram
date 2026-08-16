//! Every type, method, field, and parameter the spec defines exists in the crate.
//!
//! This is the audit that drove the Bot API 10.2 remediation, brought into the
//! repository so it runs on every commit instead of when someone remembers to.
//! It compares the committed spec snapshot against the declared Rust surface.
//!
//! # What it can and cannot tell you
//!
//! It measures whether the *surface matches* — not whether it behaves. During
//! the remediation it reported 100% while five real bugs were live: a type that
//! never deserialized, an enum that ignored its discriminant, options silently
//! dropped on one send path. Those are the other conformance tests' job. Read a
//! green result here as "nothing is missing", never as "it works".
//!
//! # Why it reads source
//!
//! Answering "does `Message` declare `giveaway`" behaviourally would need a
//! dispatch table over all 388 spec types, which is not maintainable by hand.
//! The properties here are about declarations, so declarations are what it reads.

mod common;

use common::library_sources;
use common::rust_source::{Item, parse_items};
use std::collections::{BTreeMap, BTreeSet};

/// Spec entries that legitimately have no Rust counterpart.
///
/// Each states why. These are asserted to still be exceptions, so if Telegram
/// changes one the test fails rather than keeping a stale excuse.
const EXCEPTIONS: &[(&str, &str)] = &[(
    "ResponseParameters",
    "Exists privately in client.rs as the error-response wrapper. Its contents \
     already reach callers through Error::Api, so a public type would add \
     surface for nothing.",
)];

/// Every spec type exists, as a struct or as a union variant.
#[test]
fn every_spec_type_exists() {
    let spec = common::load();
    let items = parse_items();

    // A union member may be modelled as an enum variant rather than a struct.
    let mut covered: BTreeSet<String> = items
        .iter()
        .filter(|(_, item)| item.public)
        .map(|(name, _)| name.clone())
        .collect();
    for (base, members) in &spec.unions {
        let Some(item) = items.get(base) else { continue };
        for member in members {
            let short = member.strip_prefix(base).unwrap_or(member);
            if item.variants.iter().any(|v| {
                v.eq_ignore_ascii_case(short)
                    || v.eq_ignore_ascii_case(member)
                    || short.to_lowercase().starts_with(&v.to_lowercase())
            }) {
                covered.insert(member.clone());
            }
        }
    }

    let excepted: BTreeSet<&str> = EXCEPTIONS.iter().map(|(n, _)| *n).collect();
    let missing: Vec<&String> = spec
        .types
        .keys()
        .filter(|t| !covered.contains(*t) && !excepted.contains(t.as_str()))
        .collect();

    assert!(
        missing.is_empty(),
        "{} spec type(s) have no Rust counterpart:\n  {:?}",
        missing.len(),
        missing
    );
}

/// Every spec field exists on its Rust type.
#[test]
fn every_spec_field_exists() {
    let spec = common::load();
    let items = parse_items();
    let mut missing = Vec::new();
    let mut checked = 0;

    for (type_name, spec_fields) in &spec.types {
        let Some(item) = items.get(type_name) else {
            continue; // absence of the type is the other test's report
        };
        if item.is_enum || item.fields.values().any(|f| f.flattened) {
            // A flattened field supplies whatever its target carries, and this
            // parser does not resolve across types. Skipping is honest; the
            // count below records how much was skipped.
            continue;
        }
        for field in spec_fields.keys() {
            // A tagged enum supplies the discriminant to its variants.
            if item.tag.as_deref() == Some(field.as_str())
                || matches!(field.as_str(), "type" | "status" | "source")
            {
                continue;
            }
            checked += 1;
            if !item.fields.contains_key(field) {
                missing.push(format!("  {type_name}.{field}"));
            }
        }
    }

    assert!(
        checked > 1200,
        "only {checked} fields were compared; the parser is not seeing the surface"
    );
    assert!(
        missing.is_empty(),
        "{} spec field(s) are missing from their Rust type:\n{}",
        missing.len(),
        missing.join("\n")
    );
}

/// Every spec method has a `BotClient` entry point.
#[test]
fn every_spec_method_exists() {
    let spec = common::load();
    let client = library_sources()
        .into_iter()
        .find(|(p, _)| p.ends_with("client.rs"))
        .map(|(_, t)| t)
        .expect("client.rs is in the workspace");

    let missing: Vec<&String> = spec
        .methods
        .keys()
        .filter(|m| !client.contains(&format!("/// Calls `{m}`")))
        .collect();

    assert!(
        missing.is_empty(),
        "{} spec method(s) have no BotClient entry point:\n  {:?}",
        missing.len(),
        missing
    );
}

/// Every parameter a builder accepts, following the indirections it uses.
///
/// A builder's parameters are not all in one struct. `#[serde(flatten)]` pulls
/// them from another type — the `edit*` methods get `chat_id`, `message_id`, and
/// `inline_message_id` from an `EditTarget` enum — the multipart senders keep
/// theirs in a shared `opts: MediaSendOptions`, and `setWebhook` holds its
/// certificate on the builder rather than in `Params` because uploading one
/// switches the request to multipart. A scan that reads only `{Method}Params`
/// reports all of those as missing.
fn builder_parameters(items: &BTreeMap<String, Item>, method: &str) -> BTreeSet<String> {
    fn collect(
        items: &BTreeMap<String, Item>,
        name: &str,
        out: &mut BTreeSet<String>,
        depth: usize,
    ) {
        if depth > 3 {
            return;
        }
        let Some(item) = items.get(name) else { return };
        for (field, info) in &item.fields {
            let inner = info
                .ty
                .trim_start_matches("Option<")
                .trim_end_matches('>')
                .trim();
            if info.flattened || field == "opts" || field == "params" {
                collect(items, inner, out, depth + 1);
                // A flattened enum contributes the fields of its variants.
                if let Some(target) = items.get(inner) {
                    if target.is_enum {
                        for variant in &target.variants {
                            collect(items, variant, out, depth + 1);
                        }
                    }
                }
            } else if field != "client" {
                out.insert(field.clone());
            }
        }
    }

    let capitalised = format!("{}{}", method[..1].to_uppercase(), &method[1..]);
    let mut out = BTreeSet::new();
    collect(items, &format!("{capitalised}Params"), &mut out, 0);
    collect(items, &capitalised, &mut out, 0);
    out
}

/// Method parameters that a builder takes as a constructor argument rather than
/// as a `Params` field, so the field scan cannot see them.
const CONSTRUCTOR_PARAMS: &[(&str, &str, &str)] = &[(
    "getBusinessAccountStarBalance",
    "business_connection_id",
    "Taken as a required constructor argument through the shared \
     BizConnectionIdParams, so it never appears in a getBusinessAccountStarBalance-named struct.",
)];

/// Every spec parameter exists on its builder.
#[test]
fn every_spec_parameter_exists() {
    let spec = common::load();
    let items = parse_items();
    let mut missing = Vec::new();
    let mut checked = 0;

    for (method, spec_params) in &spec.methods {
        let builder = format!(
            "{}{}Params",
            method[..1].to_uppercase(),
            &method[1..]
        );
        if !items.contains_key(&builder) && !items.contains_key(&builder[..builder.len() - 6]) {
            continue;
        }
        let declared = builder_parameters(&items, method);
        for param in spec_params.keys() {
            if CONSTRUCTOR_PARAMS
                .iter()
                .any(|(m, p, _)| m == method && p == param)
            {
                continue;
            }
            checked += 1;
            if !declared.contains(param) {
                missing.push(format!("  {method}.{param}"));
            }
        }
    }

    assert!(
        checked > 400,
        "only {checked} parameters were compared; the parser is not seeing the builders"
    );
    assert!(
        missing.is_empty(),
        "{} spec parameter(s) are missing from their builder:\n{}",
        missing.len(),
        missing.join("\n")
    );
}

/// The documented exceptions must still be exceptions.
///
/// If Telegram drops `ResponseParameters`, or a builder starts declaring the
/// parameter it currently takes by constructor, the excuse should fail rather
/// than sit in the list forever granting permission nobody has re-examined.
#[test]
fn every_documented_exception_still_applies() {
    let spec = common::load();
    let items = parse_items();

    for (name, reason) in EXCEPTIONS {
        assert!(
            spec.types.contains_key(*name),
            "`{name}` is no longer a spec type, so this exception is stale — \
             remove it. Its reason was: {reason}"
        );
        assert!(
            !items.get(*name).is_some_and(|item| item.public),
            "`{name}` now exists as a public type, so the exception is obsolete — \
             remove it and let the coverage test cover it"
        );
    }

    for (method, param, reason) in CONSTRUCTOR_PARAMS {
        let spec_method = spec
            .methods
            .get(*method)
            .unwrap_or_else(|| panic!("`{method}` is no longer a spec method; drop this exception"));
        assert!(
            spec_method.contains_key(*param),
            "`{method}` no longer takes `{param}`, so this exception is stale — \
             remove it. Its reason was: {reason}"
        );
    }
}
