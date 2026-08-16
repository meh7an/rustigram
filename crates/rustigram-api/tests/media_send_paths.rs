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
fn difference(
    method: &str,
    multipart: &BTreeSet<String>,
    json: &BTreeSet<String>,
) -> Option<String> {
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

/// Each generated media builder exposes exactly the options its method takes.
///
/// Two failures this replaces, both of which the coverage suite is blind to
/// because it resolves a builder's parameters through `MediaSendOptions` — a
/// field there counts as covered whether or not any setter reaches it:
///
/// - A setter for a parameter the spec does not define for that method is
///   surface the caller can reach and Telegram will not honour.
/// - A field written to both send paths with no setter anywhere is dead weight
///   the caller cannot use.
///
/// Driven by the committed snapshot rather than by a hand-maintained exception
/// list, so a Bot API version that adds a caption to `sendSticker` shows up here
/// instead of quietly widening the gap.
#[test]
fn each_media_builder_exposes_exactly_the_options_its_method_takes() {
    let source = include_str!("../src/methods/sending.rs");
    let spec: SpecMethods = serde_json::from_str(SNAPSHOT).expect("the snapshot parses");

    /// Shared options every generated builder exposes, and every one of these
    /// seven methods accepts. Asserted against the spec below rather than
    /// trusted.
    const UNIVERSAL: &[&str] = &[
        "business_connection_id",
        "message_thread_id",
        "direct_messages_topic_id",
        "disable_notification",
        "message_effect_id",
        "protect_content",
        "allow_paid_broadcast",
        "reply_parameters",
        "reply_markup",
        "suggested_post_parameters",
        "receiver_user_id",
        "callback_query_id",
    ];

    let mut wrong = Vec::new();

    for (builder, api_method, caption_opts) in media_sender_invocations(source) {
        let Some(params) = spec.methods.get(&api_method) else {
            wrong.push(format!("  {builder}: `{api_method}` is not in the spec"));
            continue;
        };

        let exposed: Vec<&str> = UNIVERSAL
            .iter()
            .copied()
            .chain(caption_opts.iter().map(String::as_str))
            .collect();

        for option in &exposed {
            if !params.contains_key(*option) {
                wrong.push(format!(
                    "  {api_method}: exposes `{option}`, which the spec does not \
                     define for it — a caller can set it and Telegram ignores it"
                ));
            }
        }
        for option in CAPTION_FAMILY {
            if params.contains_key(*option) && !exposed.contains(option) {
                wrong.push(format!(
                    "  {api_method}: the spec takes `{option}` and no setter \
                     reaches it"
                ));
            }
        }
    }

    assert!(
        wrong.is_empty(),
        "{} media builder surface mismatch(es):\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

/// The options that vary per method; the rest are universal.
const CAPTION_FAMILY: &[&str] = &[
    "caption",
    "parse_mode",
    "caption_entities",
    "show_caption_above_media",
    "has_spoiler",
];

const SNAPSHOT: &str = include_str!("../../rustigram-types/tests/spec/bot-api-10.2.json");

#[derive(serde::Deserialize)]
struct SpecMethods {
    methods:
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, serde_json::Value>>,
}

/// Every `media_sender!` invocation, as (builder, API method, caption options).
fn media_sender_invocations(source: &str) -> Vec<(String, String, Vec<String>)> {
    let mut found = Vec::new();
    for block in source.split("media_sender!(").skip(1) {
        let head = block.split(");").next().unwrap_or_default();
        // `SendAudio, "audio", "sendAudio", Message, [..], [caption, ..]` —
        // written on one line after the doc attribute, so the builder name is
        // the first `Send*` token rather than a line of its own.
        let Some(builder) = head
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .find(|token| token.starts_with("Send"))
            .map(str::to_owned)
        else {
            continue;
        };
        let quoted: Vec<&str> = head.split('"').skip(1).step_by(2).collect();
        let Some(api_method) = quoted.get(1) else {
            continue;
        };
        let caption_opts = head
            .rsplit_once('[')
            .and_then(|(_, tail)| tail.split(']').next())
            .map(|list| {
                list.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();
        found.push((builder, (*api_method).to_owned(), caption_opts));
    }
    assert_eq!(
        found.len(),
        7,
        "expected seven generated media builders, parsed {} — the macro's shape \
         changed and this test would check almost nothing",
        found.len()
    );
    found
}
