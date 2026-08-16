//! The public surface stays wired up and typed.
//!
//! Two properties, both of which have hidden a real bug in this codebase:
//!
//! 1. **No orphan types.** `OwnedGift::Regular` held `Box<OwnedGiftUnique>` — a
//!    copy-paste — so every regular owned gift failed to decode. The symptom
//!    from the outside was that `OwnedGiftRegular` was declared, re-exported,
//!    and referenced by nothing. A coverage audit scores such a type as covered
//!    the moment it exists, which is exactly why it survived.
//!
//! 2. **No untyped escape hatches.** A field or parameter typed
//!    `serde_json::Value` has the right name and passes every conformance check
//!    while giving the caller nothing. Eighteen of these were found and typed;
//!    the two that remain are deliberate and listed below with reasons.
//!
//! Unlike the other conformance tests, these read the source tree — the
//! properties are about the code's shape, not its runtime behaviour, and there
//! is no way to ask a running program whether a type is referenced.

mod common;

use common::{count_occurrences, library_sources};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Every public type is referenced by something other than its own declaration.
///
/// One reference is the floor rather than zero because each type is named once
/// more in its crate's `pub use` block. A type at exactly one is declared,
/// exported, and used nowhere — which is what a mis-wired enum variant looks
/// like from the outside.
#[test]
fn no_public_type_is_orphaned() {
    let sources = library_sources();

    let mut declared: BTreeMap<String, PathBuf> = BTreeMap::new();
    for (path, text) in &sources {
        for line in text.lines() {
            let line = line.trim_start();
            for prefix in ["pub struct ", "pub enum "] {
                if let Some(rest) = line.strip_prefix(prefix) {
                    let name: String = rest
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !name.is_empty() {
                        declared.insert(name, path.clone());
                    }
                }
            }
        }
    }
    assert!(
        declared.len() > 300,
        "expected 300+ public types, found {} — the scan is not seeing the sources",
        declared.len()
    );

    let mut orphans = Vec::new();
    for (name, home) in &declared {
        let mut references = 0;
        for (path, text) in &sources {
            references += count_occurrences(text, name);
            if path == home {
                // Discount the declaration itself. The trailing boundary check
                // matters: `starts_with("pub struct Chat")` also matches
                // `pub struct ChatFullInfo`, which would over-subtract and make
                // a well-used type look orphaned.
                let declarations = text
                    .lines()
                    .filter(|line| {
                        let line = line.trim_start();
                        ["pub struct ", "pub enum "].iter().any(|prefix| {
                            line.strip_prefix(prefix)
                                .and_then(|rest| rest.strip_prefix(name.as_str()))
                                .is_some_and(|after| {
                                    !after.starts_with(|c: char| c.is_alphanumeric() || c == '_')
                                })
                        })
                    })
                    .count();
                references = references.saturating_sub(declarations);
            }
        }
        if references <= 1 {
            let file = home.file_name().unwrap_or_default().to_string_lossy();
            orphans.push(format!(
                "  {name} ({file}) — declared and exported, used nowhere"
            ));
        }
    }

    assert!(
        orphans.is_empty(),
        "{} public type(s) are wired to nothing. Either something that should \
         reference them points at the wrong type, or they are dead and should be \
         removed:\n{}",
        orphans.len(),
        orphans.join("\n")
    );
}

/// The only places a `serde_json::Value` may appear in the crates.
///
/// Each entry states why. Adding one should be a decision someone argues for,
/// not a reflex when a type is inconvenient to model — eighteen such fields were
/// removed from this codebase, and every one of them looked fully covered while
/// giving callers nothing.
const UNTYPED_ALLOWLIST: &[(&str, &str, &str)] = &[
    (
        "methods/games.rs",
        "impl_into_future!(SetGameScore, serde_json::Value, \"setGameScore\");",
        "setGameScore returns `Message` for a chat message and `True` for an \
         inline one. The crate has no union type for that, and inventing one is \
         a design decision rather than a wiring task.",
    ),
    (
        "methods/sending.rs",
        "extra: serde_json::Value,",
        "media_json_body is a private helper that merges caller-supplied extra \
         fields into a JSON body. A Value is the correct type here.",
    ),
    (
        "methods/sending.rs",
        ") -> serde_json::Value {",
        "The return of that same private helper.",
    ),
];

/// No `serde_json::Value` appears outside the allowlist.
#[test]
fn no_untyped_escape_hatches_outside_the_allowlist() {
    let mut found = Vec::new();

    for (path, text) in library_sources() {
        let display = path
            .components()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("/");

        for (number, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.contains("serde_json::Value") {
                continue;
            }
            // Doc comments and ordinary comments describe, they do not expose.
            if trimmed.starts_with("//") {
                continue;
            }
            // Constructing or inspecting a Value is not the same as exposing one.
            if ["serde_json::Value::", "to_value", "from_value", "json!"]
                .iter()
                .any(|marker| trimmed.contains(marker))
            {
                continue;
            }
            let allowed = UNTYPED_ALLOWLIST
                .iter()
                .any(|(file, snippet, _)| display.ends_with(file) && trimmed == *snippet);
            if !allowed {
                found.push(format!("  {display}:{}: {trimmed}", number + 1));
            }
        }
    }

    assert!(
        found.is_empty(),
        "{} untyped `serde_json::Value` site(s) not in the allowlist. Model the \
         real type, or add an entry with a reason:\n{}",
        found.len(),
        found.join("\n")
    );
}

/// The allowlist itself must stay honest.
///
/// An entry describing a line that no longer exists means the allowlist is
/// carrying dead permission, and the next person to add a `Value` inherits it.
#[test]
fn every_allowlist_entry_still_applies() {
    let sources = library_sources();
    let mut stale = Vec::new();

    for (file, snippet, _reason) in UNTYPED_ALLOWLIST {
        let matched = sources.iter().any(|(path, text)| {
            path.to_string_lossy().ends_with(file) && text.lines().any(|l| l.trim() == *snippet)
        });
        if !matched {
            stale.push(format!("  {file}: {snippet}"));
        }
    }

    assert!(
        stale.is_empty(),
        "{} allowlist entr(ies) no longer match anything — remove them:\n{}",
        stale.len(),
        stale.join("\n")
    );
}
