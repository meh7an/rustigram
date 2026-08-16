//! A media option reaches Telegram whichever send path the file takes.
//!
//! Every media builder has two paths. Given a `file_id` or a URL the parameters
//! go out as a JSON body, which serde builds from the params struct, so an
//! option that has a setter necessarily arrives. Given raw bytes they go out as
//! `multipart/form-data`, which is assembled field by field in hand-written
//! code — and that is where five settable options were silently dropped:
//! `reply_parameters`, `reply_markup`, `message_effect_id`,
//! `allow_paid_broadcast`, and `suggested_post_parameters`. A bot could call
//! `.reply_to(id)` on a byte upload and Telegram would never see it.
//!
//! Nothing in the type system can catch that: both paths compile, both send a
//! valid request, and both return a `Message`. The only way to see it is to set
//! an option, send it both ways, and compare what actually left the process.
//!
//! # The property
//!
//! The two paths must carry the *same field names*. Not the same encoding —
//! JSON nests `reply_parameters` as an object where multipart sends it as one
//! serialised part — but the same set of things sent. That is exactly what
//! drifted, and it is checkable without reimplementing either encoder.

mod mock;

use mock::fixtures;
use rustigram_api::BotClient;
use serde_json::Value;
use std::collections::BTreeSet;
use wiremock::Request;

/// The field names a request carries, whichever encoding it used.
fn field_names(request: &Request) -> BTreeSet<String> {
    match serde_json::from_slice::<Value>(&request.body) {
        Ok(Value::Object(body)) => body.keys().cloned().collect(),
        _ => mock::multipart_field_names(request).into_iter().collect(),
    }
}

/// Sends one call twice — as a byte upload and by `file_id` — and returns the
/// field names each path produced.
///
/// The two calls go to separate servers so neither sees the other's request.
macro_rules! both_paths {
    (|$client:ident, $file:ident| $call:expr) => {{
        async fn run(upload: bool) -> BTreeSet<String> {
            let (server, $client) = mock::spawn().await;
            mock::mount_catch_all(&server).await;
            let $file = if upload {
                fixtures::uploaded_file()
            } else {
                fixtures::input_file()
            };
            let _ = $call.await;
            field_names(&mock::only_request(&server).await)
        }
        (run(true).await, run(false).await)
    }};
}

/// Records how the two paths differ for one method, if at all.
fn difference(method: &str, multipart: &BTreeSet<String>, json: &BTreeSet<String>) -> Option<String> {
    let dropped: Vec<&String> = json.difference(multipart).collect();
    let extra: Vec<&String> = multipart.difference(json).collect();
    (!dropped.is_empty() || !extra.is_empty()).then(|| {
        format!(
            "  {method}:\n                 never reaches Telegram on a byte upload: {dropped:?}\n                 never reaches Telegram by file_id:      {extra:?}"
        )
    })
}

/// Every media builder carries the same options on both paths.
///
/// One case per builder that accepts an `InputFile`, each setting the options
/// that builder actually has. `upload_sticker_file` is absent on purpose: it
/// takes no options at all, so there is nothing to drop.
#[tokio::test]
async fn media_builders_send_the_same_options_on_both_paths() {
    let mut differences = Vec::new();
    let (multipart, json) = both_paths!(|c, f| c
        .send_photo(1_i64, f)
        .caption("cap")
        .protect_content(true)
        .message_effect_id("effect")
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7)));
    differences.extend(difference("sendPhoto", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_audio(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .caption("cap")
        .parse_mode(rustigram_types::message::ParseMode::HTML)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendAudio", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_document(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .caption("cap")
        .parse_mode(rustigram_types::message::ParseMode::HTML)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendDocument", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_video(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .caption("cap")
        .parse_mode(rustigram_types::message::ParseMode::HTML)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendVideo", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_animation(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .caption("cap")
        .parse_mode(rustigram_types::message::ParseMode::HTML)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendAnimation", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_voice(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .caption("cap")
        .parse_mode(rustigram_types::message::ParseMode::HTML)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendVoice", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_video_note(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendVideoNote", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c
        .send_sticker(1_i64, f)
        .business_connection_id("bc")
        .message_thread_id(3)
        .direct_messages_topic_id(4)
        .disable_notification(true)
        .message_effect_id("effect")
        .protect_content(true)
        .allow_paid_broadcast(true)
        .reply_parameters(fixtures::reply_to(7))
        .receiver_user_id(9)
        .callback_query_id("cq"));
    differences.extend(difference("sendSticker", &multipart, &json));

    let (multipart, json) = both_paths!(|c, f| c.set_chat_photo(1_i64, f));
    differences.extend(difference("setChatPhoto", &multipart, &json));

    assert!(
        differences.is_empty(),
        "{} media builder(s) send different options depending on how the file \
         travels. An option accepted by the builder and absent from one path is \
         silently lost — the call still succeeds:\n{}",
        differences.len(),
        differences.join("\n")
    );
}

/// The exact bug: a reply set on a byte upload reaches the form.
///
/// Pinned on its own because it is the one a user reported behaviour for — the
/// photo arrived, and it was not a reply. Everything about the call looked
/// correct from the outside.
#[tokio::test]
async fn a_reply_survives_a_byte_upload() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .send_photo(42_i64, fixtures::uploaded_file())
        .reply_parameters(fixtures::reply_to(7))
        .await;

    let request = mock::only_request(&server).await;
    let fields = mock::multipart_field_names(&request);
    assert!(
        fields.iter().any(|f| f == "reply_parameters"),
        "the reply was dropped from the multipart form; Telegram would send the \
         photo as a new message instead of a reply. Fields sent: {fields:?}"
    );
}

/// Every option the shared multipart helper is supposed to carry is carried.
///
/// The five that were dropped, asserted by name. The parity test above would
/// catch a regression in any of them too, but only as a set difference — this
/// states which options the helper exists to handle, so a reader of a failure
/// knows what was lost rather than inferring it from a diff.
#[tokio::test]
async fn the_shared_multipart_options_all_reach_the_form() {
    let (server, client) = mock::spawn().await;
    mock::mount_catch_all(&server).await;

    let _ = client
        .send_photo(42_i64, fixtures::uploaded_file())
        .reply_parameters(fixtures::reply_to(7))
        .message_effect_id("effect")
        .allow_paid_broadcast(true)
        .caption("cap")
        .protect_content(true)
        .await;

    let fields = mock::multipart_field_names(&mock::only_request(&server).await);
    for option in [
        "reply_parameters",
        "message_effect_id",
        "allow_paid_broadcast",
        "caption",
        "protect_content",
    ] {
        assert!(
            fields.iter().any(|f| f == option),
            "`{option}` was set on the builder and never reached the form. \
             Fields sent: {fields:?}"
        );
    }
}

/// A byte upload really does take the multipart path, and a `file_id` does not.
///
/// The premise every test above rests on. If both paths quietly became JSON the
/// parity assertions would pass while checking one encoder twice.
#[tokio::test]
async fn the_two_paths_use_the_encodings_they_are_named_for() {
    async fn content_type(file: rustigram_types::file::InputFile) -> String {
        let (server, client) = mock::spawn().await;
        mock::mount_catch_all(&server).await;
        let _ = client.send_photo(1_i64, file).await;
        mock::only_request(&server)
            .await
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned()
    }

    let upload = content_type(fixtures::uploaded_file()).await;
    assert!(
        upload.starts_with("multipart/form-data"),
        "a byte upload must go out as multipart, got `{upload}`"
    );

    let by_id = content_type(fixtures::input_file()).await;
    assert!(
        by_id.starts_with("application/json"),
        "a file_id send must go out as JSON, got `{by_id}`"
    );
}

/// The client's `BotClient` type is what the sweep exercises.
///
/// A compile-time assertion that the fixtures build the argument types the
/// builders expect; if a signature changes, this file fails to compile rather
/// than silently testing a different call.
#[allow(dead_code)]
fn fixtures_match_the_builder_signatures(client: &BotClient) {
    let _ = client.send_photo(1_i64, fixtures::uploaded_file());
    let _ = client.send_photo(1_i64, fixtures::input_file());
}

/// Every option the media builders share is settable, or listed as not being.
///
/// `MediaSendOptions` is written to both send paths in full, so a field there
/// with no setter is dead weight the caller cannot reach — and a setter for a
/// parameter the spec does not define for that method is surface the caller can
/// reach and Telegram will not honour. The coverage suite sees neither: it
/// resolves a builder's parameters through the shared options struct, so a field
/// present there counts as covered whether or not any setter exposes it.
///
/// The macro that generates seven of these builders is uniform while the methods
/// are not, which is where both mismatches come from. The exceptions below
/// record the current state precisely so it cannot widen unnoticed; narrowing it
/// is a decision about the builder surface rather than a fix.
#[test]
fn the_shared_media_options_are_reachable_or_listed() {
    let source = include_str!("../src/methods/sending.rs");

    let fields: Vec<&str> = source
        .split("pub struct MediaSendOptions {")
        .nth(1)
        .and_then(|s| s.split("\n}").next())
        .expect("MediaSendOptions struct")
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub "))
        .filter_map(|l| l.split(':').next())
        .collect();

    let macro_body = source
        .split("macro_rules! media_sender {")
        .nth(1)
        .and_then(|s| s.split("\nmedia_sender!(").next())
        .expect("the media_sender macro body");

    /// Shared options the generated builders deliberately do not expose, and why.
    const UNEXPOSED: &[(&str, &str)] = &[
        (
            "caption_entities",
            "Valid for five of the seven generated methods but not sendVideoNote \
             or sendSticker, which take no caption at all. Exposing it uniformly \
             would add surface Telegram ignores.",
        ),
        (
            "show_caption_above_media",
            "Only sendVideo and sendAnimation accept it.",
        ),
        ("has_spoiler", "Only sendVideo and sendAnimation accept it."),
    ];

    let mut unreachable = Vec::new();
    for field in &fields {
        let settable = macro_body.contains(&format!("self.opts.{field} = Some("));
        let listed = UNEXPOSED.iter().any(|(name, _)| name == field);
        if !settable && !listed {
            unreachable.push(format!(
                "  {field}: written to both send paths, but no setter reaches it"
            ));
        }
    }
    assert!(
        unreachable.is_empty(),
        "{} shared media option(s) can never be set by a caller:\n{}",
        unreachable.len(),
        unreachable.join("\n")
    );

    let stale: Vec<&str> = UNEXPOSED
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| {
            !fields.contains(name) || macro_body.contains(&format!("self.opts.{name} = Some("))
        })
        .collect();
    assert!(
        stale.is_empty(),
        "{} exception(s) no longer describe anything — the option is gone or is \
         now exposed, so remove them: {stale:?}",
        stale.len()
    );
}
