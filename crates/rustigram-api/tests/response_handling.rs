//! What happens when Telegram answers, and when it misbehaves.
//!
//! These are the paths that only run on a bad day: flood control, an `ok: false`
//! envelope, a truncated body, a null result. None of them had ever executed
//! under test, and they are exactly the paths a bot depends on at the moment it
//! is under the most load.
//!
//! The retry loop in particular cannot be checked by reading it. Whether a 429
//! is retried, how many times, and whether the caller ends up with the right
//! error are all statements about a sequence of requests — so every retry test
//! here asserts on the number of requests the server actually received, not on
//! the returned value alone. A retry that never fired and a retry that fired and
//! succeeded return the same `Ok`.

mod mock;

use rustigram_api::error::Error;
use rustigram_api::{BotClient, ClientConfig};
use serde_json::json;
use wiremock::MockServer;

/// A minimal `Message`, enough for `sendMessage` to decode.
fn message() -> serde_json::Value {
    json!({ "message_id": 1, "date": 1_700_000_000, "chat": { "id": 42, "type": "private" } })
}

/// A client pointed at `server` with a specific retry budget.
fn client_with_retries(server: &MockServer, max_retries: u8) -> BotClient {
    let config = ClientConfig::new(mock::TOKEN)
        .expect("the test token is well-formed")
        .api_base_url(server.uri())
        .max_retries(max_retries);
    BotClient::new(config).expect("client builds")
}

// ─── The success envelope ────────────────────────────────────────────────────

/// `ok: true` with a result decodes into the method's return type.
#[tokio::test]
async fn a_successful_response_decodes_into_its_result_type() {
    let (server, client) = mock::spawn().await;
    mock::mount_ok(&server, "sendMessage", message()).await;

    let sent = client
        .send_message(42_i64, "hello")
        .await
        .expect("a well-formed success decodes");

    assert_eq!(sent.message_id, 1);
    assert_eq!(sent.chat.id, 42);
}

/// `ok: true` with a null result is an error, not a panic.
///
/// Telegram does not normally do this, but a proxy or a local Bot API server
/// can. The distinction that matters is that the caller gets a `Result` to
/// handle rather than a process that stops.
#[tokio::test]
async fn a_success_with_no_result_is_an_error_not_a_panic() {
    let (server, client) = mock::spawn().await;
    mock::mount_raw(&server, "sendMessage", 200, r#"{"ok":true,"result":null}"#).await;

    let error = client
        .send_message(42_i64, "hello")
        .await
        .expect_err("a null result cannot produce a Message");

    assert!(
        matches!(error, Error::Decode(_)),
        "expected a decode error, got {error:?}"
    );
}

// ─── The failure envelope ────────────────────────────────────────────────────

/// `ok: false` maps to `Error::Api`, carrying the code and description intact.
#[tokio::test]
async fn an_api_error_keeps_its_code_and_description() {
    let (server, client) = mock::spawn().await;
    mock::mount_api_error(&server, "sendMessage", 400, "Bad Request: chat not found", None).await;

    let error = client
        .send_message(42_i64, "hello")
        .await
        .expect_err("ok: false is an error");

    match error {
        Error::Api {
            error_code,
            description,
            ..
        } => {
            assert_eq!(error_code, 400);
            assert_eq!(description, "Bad Request: chat not found");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

/// `migrate_to_chat_id` survives into the error, so a caller can act on it.
///
/// It is the one piece of an error envelope a bot is expected to *use*: the
/// group became a supergroup and the send should be retried against the new ID.
#[tokio::test]
async fn a_migration_error_carries_the_new_chat_id() {
    let (server, client) = mock::spawn().await;
    mock::mount_api_error(
        &server,
        "sendMessage",
        400,
        "Bad Request: group chat was upgraded to a supergroup chat",
        Some(json!({ "migrate_to_chat_id": -1_001_234_567_890_i64 })),
    )
    .await;

    let error = client.send_message(42_i64, "hi").await.unwrap_err();
    assert_eq!(
        error_migrate_id(&error),
        Some(-1_001_234_567_890),
        "the new chat id was dropped, so a caller cannot follow the migration"
    );
}

fn error_migrate_id(error: &Error) -> Option<i64> {
    match error {
        Error::Api {
            migrate_to_chat_id, ..
        } => *migrate_to_chat_id,
        _ => None,
    }
}

/// A body that is not an API envelope at all maps to `Error::Decode`.
#[tokio::test]
async fn a_malformed_body_maps_to_a_decode_error() {
    let (server, client) = mock::spawn().await;
    mock::mount_raw(&server, "sendMessage", 200, "<html>502 Bad Gateway</html>").await;

    let error = client.send_message(42_i64, "hi").await.unwrap_err();
    assert!(
        matches!(error, Error::Decode(_)),
        "an unparseable body must be a decode error, got {error:?}"
    );
}

// ─── Flood control ───────────────────────────────────────────────────────────

/// A 429 is retried, and the second attempt's success is returned.
///
/// Asserted on the request count. Returning `Ok` proves only that the call
/// eventually worked — it cannot distinguish a retry that fired from a server
/// that never rate-limited in the first place.
#[tokio::test]
async fn a_flood_control_error_is_retried_and_then_succeeds() {
    let (server, client) = mock::spawn().await;
    mock::mount_then(
        &server,
        "sendMessage",
        mock::flood_control(0),
        1,
        json!({ "ok": true, "result": message() }),
    )
    .await;

    let sent = client
        .send_message(42_i64, "hello")
        .await
        .expect("the retry succeeds");
    assert_eq!(sent.message_id, 1);

    assert_eq!(
        mock::requests(&server).await.len(),
        2,
        "the call should have been sent twice: once rate-limited, once accepted"
    );
}

/// The retry budget is spent exactly, and then the caller sees `RateLimit`.
///
/// `max_retries` is a count of *retries*, so a budget of two means three
/// requests in total. Off-by-one here is invisible in production until a bot
/// hammers Telegram one extra time per call under load.
#[tokio::test]
async fn the_retry_budget_is_spent_exactly_and_then_surfaces() {
    for budget in [0_u8, 1, 3] {
        let server = MockServer::start().await;
        let client = client_with_retries(&server, budget);
        mock::mount_api_error(
            &server,
            "sendMessage",
            429,
            "Too Many Requests: retry after 0",
            Some(json!({ "retry_after": 0 })),
        )
        .await;

        let error = client.send_message(42_i64, "hi").await.unwrap_err();
        assert!(
            matches!(error, Error::RateLimit { .. }),
            "an exhausted budget must surface as RateLimit, got {error:?}"
        );
        assert_eq!(
            mock::requests(&server).await.len(),
            usize::from(budget) + 1,
            "with max_retries({budget}) the call should be sent {} time(s)",
            budget + 1
        );
    }
}

/// The `retry_after` Telegram asks for reaches the caller unchanged.
#[tokio::test]
async fn the_requested_wait_reaches_the_caller() {
    let server = MockServer::start().await;
    let client = client_with_retries(&server, 0);
    mock::mount_api_error(
        &server,
        "sendMessage",
        429,
        "Too Many Requests: retry after 42",
        Some(json!({ "retry_after": 42 })),
    )
    .await;

    let error = client.send_message(42_i64, "hi").await.unwrap_err();
    assert_eq!(
        error.retry_after(),
        Some(42),
        "the caller cannot back off correctly without the wait Telegram named"
    );
}

/// A byte upload is **not** retried on flood control.
///
/// Pinned rather than fixed. `post_json` retries and `post_multipart` returns
/// immediately, because `reqwest`'s multipart `Form` is consumed by the send and
/// cannot be rebuilt for a second attempt. A caller who sets `max_retries(5)`
/// gets five attempts on a JSON send and one on an upload — worth knowing, and
/// worth failing here if it ever changes silently in either direction.
#[tokio::test]
async fn a_byte_upload_is_not_retried_on_flood_control() {
    let server = MockServer::start().await;
    let client = client_with_retries(&server, 5);
    mock::mount_api_error(
        &server,
        "sendPhoto",
        429,
        "Too Many Requests: retry after 0",
        Some(json!({ "retry_after": 0 })),
    )
    .await;

    let error = client
        .send_photo(42_i64, mock::fixtures::uploaded_file())
        .await
        .unwrap_err();

    assert!(matches!(error, Error::RateLimit { .. }), "got {error:?}");
    assert_eq!(
        mock::requests(&server).await.len(),
        1,
        "the multipart path does not retry, so exactly one request should have \
         been sent despite max_retries(5)"
    );
}

// ─── The error predicates ────────────────────────────────────────────────────

/// Each predicate matches the payload Telegram actually sends for it.
///
/// The predicates match on substrings of a description, which is a contract with
/// Telegram's prose rather than with its schema. Wording is what breaks them, so
/// each case uses the real message text.
#[tokio::test]
async fn the_error_predicates_match_real_payloads() {
    for (code, description, check, name) in [
        (
            403_u16,
            "Forbidden: bot was blocked by the user",
            Error::is_blocked as fn(&Error) -> bool,
            "is_blocked",
        ),
        (
            400,
            "Bad Request: chat not found",
            Error::is_chat_not_found,
            "is_chat_not_found",
        ),
        (
            429,
            "Too Many Requests: retry after 5",
            Error::is_rate_limit,
            "is_rate_limit",
        ),
    ] {
        let server = MockServer::start().await;
        // No retries: a 429 would otherwise be retried away before it is seen.
        let client = client_with_retries(&server, 0);
        mock::mount_api_error(&server, "sendMessage", code, description, None).await;

        let error = client.send_message(42_i64, "hi").await.unwrap_err();
        assert!(
            check(&error),
            "`{name}` did not match the payload Telegram sends for it: \
             {code} {description:?} produced {error:?}"
        );
    }
}

/// A predicate says no to an error that is not its case.
///
/// Without this the suite would pass just as well if every predicate returned
/// `true` unconditionally.
#[tokio::test]
async fn the_error_predicates_reject_unrelated_errors() {
    let server = MockServer::start().await;
    let client = client_with_retries(&server, 0);
    mock::mount_api_error(&server, "sendMessage", 400, "Bad Request: message is too long", None)
        .await;

    let error = client.send_message(42_i64, "hi").await.unwrap_err();
    assert!(!error.is_blocked(), "is_blocked matched an unrelated 400");
    assert!(
        !error.is_chat_not_found(),
        "is_chat_not_found matched an unrelated 400"
    );
    assert!(
        !error.is_rate_limit(),
        "is_rate_limit matched an unrelated 400"
    );
    assert_eq!(error.retry_after(), None, "an unrelated 400 named a wait");
}

// ─── File download ───────────────────────────────────────────────────────────

/// `download_file` returns the bytes, from the `/file/bot<token>/` endpoint.
///
/// A different URL shape from every other call — it is not a Bot API method, so
/// nothing else in the suite would notice it breaking.
#[tokio::test]
async fn download_file_fetches_the_bytes_from_the_file_endpoint() {
    let (server, client) = mock::spawn().await;
    mock::mount_file(&server, "photos/file_1.jpg", b"\xff\xd8\xffbytes").await;

    let bytes = client
        .download_file("photos/file_1.jpg")
        .await
        .expect("the download succeeds");
    assert_eq!(&bytes[..], b"\xff\xd8\xffbytes");

    let request = mock::only_request(&server).await;
    assert_eq!(
        request.url.path(),
        format!("/file/bot{}/photos/file_1.jpg", mock::TOKEN),
        "the download URL must include the token and the file path verbatim"
    );
}

/// A failed download is an error, not bytes.
///
/// A `file_path` from `get_file` is valid for about an hour; after that the
/// endpoint answers 404 with an error envelope. That body used to be returned
/// as the file itself, so a bot expecting a JPEG received thirty bytes of JSON
/// and a successful `Result` — it would write that to disk, or hand it to an
/// image decoder, with nothing anywhere reporting a problem.
#[tokio::test]
async fn an_expired_file_path_is_an_error_not_the_error_page() {
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, ResponseTemplate};

    let (server, client) = mock::spawn().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(404).set_body_string(
            r#"{"ok":false,"error_code":404,"description":"Not Found: file is temporarily unavailable"}"#,
        ))
        .mount(&server)
        .await;

    let error = client
        .download_file("photos/expired.jpg")
        .await
        .expect_err("a 404 from the file endpoint is a failure");

    match error {
        Error::Api {
            error_code,
            description,
            ..
        } => {
            assert_eq!(error_code, 404);
            assert!(
                description.contains("temporarily unavailable"),
                "the reason Telegram gave was lost: {description}"
            );
        }
        other => panic!("expected Error::Api carrying Telegram's reason, got {other:?}"),
    }
}

/// A non-JSON failure body still produces an error, keeping what it can.
///
/// A proxy or a local Bot API server in front of Telegram may answer with plain
/// text or HTML. The status code is then the only fact available, and it must
/// still be enough to fail on.
#[tokio::test]
async fn a_download_failure_without_an_envelope_still_errors() {
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, ResponseTemplate};

    let (server, client) = mock::spawn().await;
    Mock::given(method("GET"))
        .and(path_regex(".*"))
        .respond_with(ResponseTemplate::new(502).set_body_string("<html>502 Bad Gateway</html>"))
        .mount(&server)
        .await;

    let error = client.download_file("photos/x.jpg").await.unwrap_err();
    match error {
        Error::Api { error_code, .. } => assert_eq!(
            error_code, 502,
            "with no envelope the HTTP status is what the caller gets"
        ),
        other => panic!("expected Error::Api, got {other:?}"),
    }
}
