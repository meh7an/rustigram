//! Parsing the workspace's own Rust declarations.
//!
//! Two conformance properties are about the shape of the code rather than its
//! runtime behaviour — which fields a type declares, and whether each one can be
//! absent from the wire. Neither can be answered by a running program, so both
//! read the tree, and they share this parser rather than each growing their own.

#![allow(dead_code)] // each test binary uses a different subset

use super::library_sources;
use std::collections::BTreeMap;

/// A field as the wire sees it, after serde attributes are applied.
#[derive(Debug, Default, Clone)]
pub struct Field {
    /// Whether the field carries `#[serde(default)]`, which lets it be absent
    /// from the wire even when the Rust type is not an `Option`.
    pub has_default: bool,
    pub flattened: bool,
    /// The declared Rust type, needed to follow `flatten` and `opts` to the
    /// struct that actually holds the parameters.
    pub ty: String,
}

/// A parsed Rust struct or enum.
#[derive(Debug, Default)]
pub struct Item {
    pub fields: BTreeMap<String, Field>,
    pub variants: Vec<String>,
    pub is_enum: bool,
    /// Whether the declaration is `pub`. Spec types must be public; builder
    /// `Params` structs are private by design, so both are parsed but only the
    /// public ones count as satisfying the spec.
    pub public: bool,
    /// The `#[serde(tag = "...")]` name, if any. A tagged enum supplies this
    /// field to every variant, so variant structs correctly omit it.
    pub tag: Option<String>,
}

/// Parses every `pub struct` and `pub enum` in the workspace.
///
/// Deliberately simple: serde's own semantics are only honoured where they
/// change what appears on the wire — `rename`, `flatten`, and the container
/// `tag`. Anything subtler than that would be reimplementing serde, and a
/// conformance test that has its own bugs is worse than none.
pub fn parse_items() -> BTreeMap<String, Item> {
    let mut items: BTreeMap<String, Item> = BTreeMap::new();

    for (_path, text) in library_sources() {
        let mut current: Option<String> = None;
        let mut depth = 0_i32;
        let mut attrs: Vec<String> = Vec::new();
        let mut container_tag: Option<String> = None;

        for line in text.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("#[serde(") {
                if current.is_none() {
                    if let Some(tag) = extract_quoted(trimmed, "tag = \"") {
                        container_tag = Some(tag);
                    }
                }
                attrs.push(trimmed.to_owned());
                continue;
            }
            if trimmed.starts_with("#[") || trimmed.starts_with("//") {
                continue;
            }

            if current.is_none() {
                // `pub` is optional: rustigram-api declares its *Params structs
                // private, and a parser that assumes `pub` sees almost no
                // builders at all.
                let public = trimmed.starts_with("pub ");
                let declaration = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
                for keyword in ["struct ", "enum "] {
                    if let Some(rest) = declaration.strip_prefix(keyword) {
                        let name: String = rest
                            .chars()
                            .take_while(|c| c.is_alphanumeric() || *c == '_')
                            .collect();
                        if name.is_empty() || !rest.contains('{') {
                            continue;
                        }
                        let entry = items.entry(name.clone()).or_default();
                        entry.is_enum = keyword.starts_with("enum");
                        entry.public = public;
                        entry.tag = container_tag.take();
                        // Count this line's own braces. An empty struct is
                        // written `pub struct Foo {}` — assuming a depth of one
                        // here leaves the parser permanently inside it, and it
                        // then swallows every later declaration in the file.
                        depth = line.matches('{').count() as i32
                            - line.matches('}').count() as i32;
                        current = (depth > 0).then_some(name);
                        attrs.clear();
                    }
                }
                if current.is_none() {
                    attrs.clear();
                    container_tag = None;
                }
                continue;
            }

            let name = current.clone().unwrap_or_default();
            depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            if depth <= 0 {
                current = None;
                attrs.clear();
                continue;
            }

            let entry = items.entry(name).or_default();
            if entry.is_enum {
                let is_variant = trimmed
                    .chars()
                    .next()
                    .is_some_and(char::is_uppercase);
                if is_variant {
                    if let Some(variant) = trimmed.split('(').next() {
                        entry
                            .variants
                            .push(variant.trim_end_matches([',', ' ', '{']).to_owned());
                    }
                } else if let Some((field, ty)) = trimmed.split_once(':') {
                    // A struct-variant's own fields. An enum flattened into a
                    // params struct contributes exactly these — `EditTarget`
                    // is how the edit* methods carry chat_id, message_id, and
                    // inline_message_id.
                    let field = field.trim();
                    if !field.is_empty() && field.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        entry.fields.insert(
                            field.to_owned(),
                            Field {
                                flattened: false,
                                has_default: attrs.join(" ").contains("default"),
                                ty: ty.trim().trim_end_matches(',').to_owned(),
                            },
                        );
                    }
                }
            } else {
                // Struct fields carry `pub` in rustigram-types and omit it in
                // the api crate's Params structs. Requiring it here would make
                // every builder look like it declared no parameters at all.
                let rest = trimmed.strip_prefix("pub ").unwrap_or(trimmed);
                if let Some((raw_name, _)) = rest.split_once(':') {
                    let joined = attrs.join(" ");
                    let wire = extract_quoted(&joined, "rename = \"")
                        .unwrap_or_else(|| raw_name.trim().to_owned());
                    let ty = rest
                        .split_once(':')
                        .map(|(_, ty)| ty.trim().trim_end_matches(',').to_owned())
                        .unwrap_or_default();
                    entry.fields.insert(
                        wire,
                        Field {
                            flattened: joined.contains("flatten"),
                            has_default: joined.contains("default"),
                            ty,
                        },
                    );
                }
            }
            attrs.clear();
        }
    }
    items
}

pub fn extract_quoted(haystack: &str, prefix: &str) -> Option<String> {
    let start = haystack.find(prefix)? + prefix.len();
    let rest = &haystack[start..];
    Some(rest[..rest.find('"')?].to_owned())
}

