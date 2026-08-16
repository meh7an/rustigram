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

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The workspace root, reached from this crate's manifest directory.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("workspace root resolves")
}

/// Every library source file in the workspace.
///
/// Tests and examples are excluded on purpose: a type referenced only by its own
/// test is still dead weight in the library, and counting those references would
/// hide precisely the bug this checks for.
fn library_sources() -> Vec<(PathBuf, String)> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }

    let crates = workspace_root().join("crates");
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&crates)
        .expect("crates/ exists")
        .flatten()
    {
        walk(&entry.path().join("src"), &mut files);
    }
    assert!(
        files.len() > 20,
        "expected to find the workspace sources, found {} files — the layout may \
         have changed and this test would silently check nothing",
        files.len()
    );
    files
        .into_iter()
        .map(|p| {
            let text = std::fs::read_to_string(&p).expect("source file is readable");
            (p, text)
        })
        .collect()
}

/// Counts word-boundary occurrences of `needle` in `haystack`.
fn count_occurrences(haystack: &str, needle: &str) -> usize {
    let bytes = haystack.as_bytes();
    let mut count = 0;
    let mut from = 0;
    while let Some(found) = haystack[from..].find(needle) {
        let start = from + found;
        let end = start + needle.len();
        let before_ok = start == 0 || !is_ident_byte(bytes[start - 1]);
        let after_ok = end == bytes.len() || !is_ident_byte(bytes[end]);
        if before_ok && after_ok {
            count += 1;
        }
        from = end;
    }
    count
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

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
