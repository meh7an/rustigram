//! The mock harness itself works.
//!
//! These are the worked examples the transport tests build on. They prove the
//! harness can observe a real request end to end, and that its assertions fail
//! when the request is wrong — a harness whose assertions always pass would make
//! every transport test worthless while looking like coverage.

mod mock;

use rustigram_types::message::ParseMode;
use serde_json::json;

/// A minimal `Message` result, enough for `sendMessage` to decode.
fn message_result() -> serde_json::Value {
    json!({
        "message_id": 1,
        "date": 1_700_000_000,
        "chat": { "id": 42, "type": "private" }
    })
}

/// The worked example: a builder with several parameters set produces exactly
/// the body Telegram expects.
#[tokio::test]
async fn send_message_puts_the_expected_body_on_the_wire() {
    let (server, client) = mock::spawn().await;
    mock::mount_ok(&server, "sendMessage", message_result()).await;

    let sent = client
        .send_message(42_i64, "hello")
        .parse_mode(ParseMode::HTML)
        .disable_notification(true)
        .reply_to(7)
        .await
        .expect("the mocked call succeeds");

    assert_eq!(sent.message_id, 1, "the result decodes back into a Message");

    let request = mock::only_request(&server).await;
    let body = mock::json_body(&request);

    mock::assert_field(&body, "chat_id", json!(42));
    mock::assert_field(&body, "text", json!("hello"));
    mock::assert_field(&body, "parse_mode", json!("HTML"));
    mock::assert_field(&body, "disable_notification", json!(true));
    assert_eq!(
        body["reply_parameters"]["message_id"], 7,
        "reply_to sets reply_parameters.message_id"
    );
}

/// Unset optional parameters must not appear at all.
#[tokio::test]
async fn unset_parameters_are_absent_from_the_body() {
    let (server, client) = mock::spawn().await;
    mock::mount_ok(&server, "sendMessage", message_result()).await;

    client
        .send_message(42_i64, "hello")
        .await
        .expect("the mocked call succeeds");

    let body = mock::json_body(&mock::only_request(&server).await);
    for key in [
        "parse_mode",
        "disable_notification",
        "reply_parameters",
        "business_connection_id",
        "message_effect_id",
    ] {
        mock::assert_absent(&body, key);
    }
}

/// The harness must observe multipart bodies too — this is the path the
/// silently-dropped options lived on.
#[tokio::test]
async fn multipart_uploads_are_observable() {
    use rustigram_types::file::InputFile;

    let (server, client) = mock::spawn().await;
    mock::mount_ok(&server, "sendPhoto", message_result()).await;

    client
        .send_photo(
            42_i64,
            InputFile::Bytes {
                filename: "p.jpg".to_owned(),
                data: b"\xff\xd8\xff".to_vec(),
                mime_type: "image/jpeg".to_owned(),
            },
        )
        .caption("a caption")
        .protect_content(true)
        .await
        .expect("the mocked call succeeds");

    let request = mock::only_request(&server).await;
    let content_type = request
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(
        content_type.starts_with("multipart/form-data"),
        "byte uploads must use multipart, got {content_type}"
    );

    mock::assert_multipart_fields(
        &request,
        &["caption", "chat_id", "photo", "protect_content"],
    );
}

/// `only_request` must fail when the call count is wrong, or a body assertion
/// could silently run against the wrong request.
#[tokio::test]
#[should_panic(expected = "expected exactly one request")]
async fn only_request_rejects_an_unexpected_call_count() {
    let (server, client) = mock::spawn().await;
    mock::mount_ok(&server, "sendMessage", message_result()).await;

    client.send_message(42_i64, "one").await.unwrap();
    client.send_message(42_i64, "two").await.unwrap();

    mock::only_request(&server).await;
}

/// A field assertion must actually fail when the field is wrong. Without this,
/// every transport test could be asserting nothing.
#[tokio::test]
#[should_panic(expected = "field `text`")]
async fn field_assertions_fail_on_a_wrong_value() {
    let (server, client) = mock::spawn().await;
    mock::mount_ok(&server, "sendMessage", message_result()).await;

    client.send_message(42_i64, "hello").await.unwrap();

    let body = mock::json_body(&mock::only_request(&server).await);
    mock::assert_field(&body, "text", json!("something else"));
}

/// Two servers must not share state, so tests can run concurrently.
#[tokio::test]
async fn servers_are_isolated_from_each_other() {
    let (server_a, client_a) = mock::spawn().await;
    let (server_b, client_b) = mock::spawn().await;
    mock::mount_ok(&server_a, "sendMessage", message_result()).await;
    mock::mount_ok(&server_b, "sendMessage", message_result()).await;

    assert_ne!(
        server_a.uri(),
        server_b.uri(),
        "each server gets its own port"
    );

    client_a.send_message(1_i64, "a").await.unwrap();
    client_b.send_message(2_i64, "b").await.unwrap();
    client_b.send_message(3_i64, "c").await.unwrap();

    assert_eq!(mock::requests(&server_a).await.len(), 1);
    assert_eq!(mock::requests(&server_b).await.len(), 2);
}

/// Every mock binds to loopback, so the suite runs with no network.
#[tokio::test]
async fn the_mock_server_is_local_only() {
    let (server, _client) = mock::spawn().await;
    let uri = server.uri();
    assert!(
        uri.starts_with("http://127.0.0.1:") || uri.starts_with("http://localhost:"),
        "mock server must bind to loopback, got {uri}"
    );
}
