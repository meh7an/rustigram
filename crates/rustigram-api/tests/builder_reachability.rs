//! Every parameter the Bot API defines can actually be set by a caller.
//!
//! The coverage suite asks whether a parameter *exists* in a builder's `*Params`
//! struct. That is a weaker question than it looks: a field can exist, be
//! serialised correctly, and have no way for anyone outside the crate to give it
//! a value. It scores as covered and is unreachable.
//!
//! Forty-three parameters were in exactly that state across thirteen methods —
//! `sendInvoice` alone accounted for sixteen, including `protect_content` and
//! `reply_parameters`. Nothing failed, no test went red, and the coverage
//! numbers read 100%.
//!
//! # What counts as reachable
//!
//! A params field is reachable if some code path lets a caller determine its
//! value:
//!
//! - a setter assigns it (`self.params.field = …`), or
//! - a constructor initialises it from an argument, either as `field: expr` or
//!   the field-init shorthand `field,`.
//!
//! Initialising it to `None` or `Default::default()` in the constructor does
//! **not** count — that is the builder defaulting the field, which is precisely
//! the state that looks reachable and is not. Getting this distinction wrong is
//! what made an earlier hand-rolled version of this scan report zero.

use std::collections::BTreeMap;

const SNAPSHOT: &str = include_str!("../../rustigram-types/tests/spec/bot-api-10.2.json");

#[derive(serde::Deserialize)]
struct Spec {
    methods: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
}

/// The body of the brace-delimited block opening at or after `from`.
fn block_at(text: &str, from: usize) -> &str {
    let Some(open) = text[from..].find('{').map(|o| from + o) else {
        return "";
    };
    let bytes = text.as_bytes();
    let mut depth = 0_i32;
    for i in open..text.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return &text[open + 1..i];
                }
            }
            _ => {}
        }
    }
    &text[open + 1..]
}

/// Every occurrence of `needle` at a token boundary.
fn occurrences<'a>(haystack: &'a str, needle: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(at) = haystack[from..].find(needle) {
        let start = from + at;
        let before_ok = start == 0 || {
            let c = haystack[..start].chars().next_back().unwrap_or(' ');
            !c.is_alphanumeric() && c != '_'
        };
        if before_ok {
            out.push(&haystack[start + needle.len()..]);
        }
        from = start + needle.len();
    }
    out
}

/// Whether a caller can determine `field`'s value on this builder.
fn reachable(impl_body: &str, field: &str) -> bool {
    // A setter writes it.
    if occurrences(impl_body, &format!("params.{field}"))
        .iter()
        .any(|rest| rest.trim_start().starts_with('='))
    {
        return true;
    }
    for rest in occurrences(impl_body, field) {
        let rest = rest.trim_start();
        // Field-init shorthand: the value came in as a constructor argument.
        if rest.starts_with(',') || rest.starts_with('}') {
            return true;
        }
        if let Some(value) = rest.strip_prefix(':') {
            let value = value
                .trim_start()
                .split([',', '\n', '}'])
                .next()
                .unwrap_or("")
                .trim();
            // `None` and `Default::default()` are the builder defaulting the
            // field, not a caller setting it.
            if !matches!(value, "None" | "Default::default()" | "") {
                return true;
            }
        }
    }
    false
}

/// Builders, their params struct, and the impl block that fills it.
struct Builder {
    api_method: String,
    params_struct: String,
    impl_body: String,
}

fn builders(source: &str) -> Vec<Builder> {
    // builder -> params struct
    let mut params_of: BTreeMap<String, String> = BTreeMap::new();
    let mut from = 0;
    while let Some(at) = source[from..].find("\npub struct ") {
        let start = from + at;
        let name: String = source[start + 12..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let body = block_at(source, start);
        if let Some(rest) = body.split("params:").nth(1) {
            let ps: String = rest
                .trim_start()
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if ps.ends_with("Params") {
                params_of.insert(name, ps);
            }
        }
        from = start + 12;
    }

    // builder -> concatenated `impl X { … }` bodies
    let mut impl_of: BTreeMap<String, String> = BTreeMap::new();
    let mut from = 0;
    while let Some(at) = source[from..].find("\nimpl ") {
        let start = from + at;
        let name: String = source[start + 6..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        // Only inherent impls; `impl IntoFuture for X` names the trait first.
        if source[start + 6 + name.len()..]
            .trim_start()
            .starts_with('{')
        {
            impl_of
                .entry(name)
                .or_default()
                .push_str(block_at(source, start));
        }
        from = start + 6;
    }

    // builder -> API method. Two forms, and missing either loses most of the
    // surface: `impl_into_future!(SendDice, Message, "sendDice")` names the
    // method as its third argument, while a hand-written `impl IntoFuture for X`
    // names it in the `post_json`/`post_multipart` call inside.
    let mut method_of: BTreeMap<String, String> = BTreeMap::new();

    let mut from = 0;
    while let Some(at) = source[from..].find("impl_into_future!(") {
        let start = from + at + "impl_into_future!(".len();
        let name: String = source[start..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if let Some(method) = source[start..]
            .split(");")
            .next()
            .and_then(|args| args.split('"').nth(1))
        {
            method_of.entry(name).or_insert_with(|| method.to_owned());
        }
        from = start;
    }

    let mut from = 0;
    while let Some(at) = source[from..].find("\nimpl IntoFuture for ") {
        let start = from + at + "\nimpl IntoFuture for ".len();
        let name: String = source[start..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let body = block_at(source, start);
        for call in ["post_json(\"", "post_multipart(\""] {
            if let Some(m) = body.split(call).nth(1).and_then(|r| r.split('"').next()) {
                method_of
                    .entry(name.clone())
                    .or_insert_with(|| m.to_owned());
                break;
            }
        }
        from = start;
    }

    params_of
        .into_iter()
        .filter_map(|(builder, params_struct)| {
            Some(Builder {
                api_method: method_of.get(&builder)?.clone(),
                params_struct,
                impl_body: impl_of.get(&builder)?.clone(),
            })
        })
        .collect()
}

/// The wire names of a params struct's fields, honouring `#[serde(rename)]`.
fn params_fields(source: &str, params_struct: &str) -> Vec<(String, String)> {
    let Some(at) = source.find(&format!("struct {params_struct} {{")) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut renamed: Option<String> = None;
    for line in block_at(source, at).lines() {
        let trimmed = line.trim();
        if let Some(r) = trimmed
            .split("rename = \"")
            .nth(1)
            .and_then(|r| r.split('"').next())
        {
            renamed = Some(r.to_owned());
            continue;
        }
        if trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }
        if let Some((name, _)) = trimmed.trim_start_matches("pub ").split_once(':') {
            let name = name.trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                out.push((
                    name.to_owned(),
                    renamed.take().unwrap_or_else(|| name.to_owned()),
                ));
                continue;
            }
        }
        renamed = None;
    }
    out
}

/// Every spec parameter a builder declares can be given a value by a caller.
#[test]
fn every_declared_parameter_is_reachable() {
    let source = include_str!("../src/methods/sending.rs");
    let mut sources = String::from(source);
    for extra in [
        include_str!("../src/methods/payments.rs"),
        include_str!("../src/methods/editing.rs"),
        include_str!("../src/methods/chat_management.rs"),
        include_str!("../src/methods/inline.rs"),
        include_str!("../src/methods/stickers.rs"),
        include_str!("../src/methods/getters.rs"),
        include_str!("../src/methods/bot_settings.rs"),
        include_str!("../src/methods/games.rs"),
        include_str!("../src/methods/stories.rs"),
        include_str!("../src/methods/forum.rs"),
    ] {
        sources.push('\n');
        sources.push_str(extra);
    }

    let spec: Spec = serde_json::from_str(SNAPSHOT).expect("the snapshot parses");
    let found = builders(&sources);
    assert!(
        found.len() > 100,
        "parsed only {} builders — the layout changed and this test would check \
         almost nothing",
        found.len()
    );

    let mut unreachable = Vec::new();
    for builder in &found {
        let Some(params) = spec.methods.get(&builder.api_method) else {
            continue;
        };
        for (rust, wire) in params_fields(&sources, &builder.params_struct) {
            if params.contains_key(&wire) && !reachable(&builder.impl_body, &rust) {
                unreachable.push(format!("  {}.{wire}", builder.api_method));
            }
        }
    }

    unreachable.sort();
    unreachable.dedup();
    assert!(
        unreachable.is_empty(),
        "{} spec parameter(s) are declared, serialised, and impossible to set. \
         The coverage suite scores these as covered because the field exists — \
         add a setter:\n{}",
        unreachable.len(),
        unreachable.join("\n")
    );
}
