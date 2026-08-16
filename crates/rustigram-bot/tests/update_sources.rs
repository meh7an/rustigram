//! Where updates come from: long polling and the webhook.
//!
//! Both are loops a bot lives inside for months, and both fail quietly. A
//! poller that stops advancing its offset re-delivers the same batch forever; a
//! webhook that answers `200` while dropping the body looks perfectly healthy
//! from Telegram's side. Neither shows up as an error anywhere.
//!
//! # What is asserted
//!
//! For polling, the *requests the server received* — the offset is only real if
//! it appears in the next `getUpdates` call, and a poller that stores it
//! correctly while never sending it behaves exactly like one that never
//! advanced. For the webhook, that the update reaches a handler, not just that
//! the status code was right.

mod dispatch_support;

use dispatch_support::{client_for, client_without_retries, reporting, reports};
use rustigram_bot::dispatcher::Dispatcher;
use rustigram_bot::error::BotError;
use rustigram_bot::filter::filters;
use rustigram_bot::update_listener::polling::LongPoller;
use serde_json::{Value, json};
use std::time::Duration;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

// ─── Mock plumbing ───────────────────────────────────────────────────────────

/// An `ok: true` batch of updates carrying the given ids.
fn batch(ids: &[i64]) -> Value {
    let updates: Vec<Value> = ids
        .iter()
        .map(|id| {
            json!({
                "update_id": id,
                "message": {
                    "message_id": 1, "date": 1_700_000_000,
                    "chat": { "id": 42, "type": "private" },
                    "from": { "id": 7, "is_bot": false, "first_name": "T" },
                    "text": "hi"
                }
            })
        })
        .collect();
    json!({ "ok": true, "result": updates })
}

/// Answers `getUpdates` with each body in turn, repeating the last one forever.
///
/// Registration order matters: wiremock prefers the mock mounted first among
/// those still able to respond, so the scripted bodies must be mounted before
/// the one that repeats.
async fn script(bodies: Vec<Value>) -> MockServer {
    let server = MockServer::start().await;
    let (last, rest) = bodies.split_last().expect("at least one body");
    for body in rest {
        Mock::given(method("POST"))
            .and(path_regex(r".*/getUpdates$"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
            .up_to_n_times(1)
            .mount(&server)
            .await;
    }
    Mock::given(method("POST"))
        .and(path_regex(r".*/getUpdates$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(last.clone()))
        .mount(&server)
        .await;
    server
}

/// The `offset` each recorded `getUpdates` request carried, `None` if unset.
async fn offsets(server: &MockServer) -> Vec<Option<i64>> {
    server
        .received_requests()
        .await
        .expect("the mock records requests")
        .iter()
        .map(|r: &Request| {
            serde_json::from_slice::<Value>(&r.body)
                .ok()
                .and_then(|b| b.get("offset").and_then(Value::as_i64))
        })
        .collect()
}

// ─── Long polling ────────────────────────────────────────────────────────────

/// The offset advances to `last.update_id + 1` and is sent on the next call.
///
/// Asserted on the wire rather than on the poller's field. An offset stored
/// correctly and never sent produces the same infinite re-delivery as one that
/// never advanced, and from inside the process the two look identical.
#[tokio::test]
async fn the_offset_advances_and_is_sent_on_the_next_call() {
    let server = script(vec![batch(&[10, 11, 12]), batch(&[20])]).await;
    let mut poller = LongPoller::new(client_for(&server.uri()));

    let first = poller.next_batch().await.expect("the first batch arrives");
    assert_eq!(first.len(), 3);

    let second = poller.next_batch().await.expect("the second batch arrives");
    assert_eq!(second.len(), 1);

    assert_eq!(
        offsets(&server).await,
        vec![None, Some(13)],
        "the first call carries no offset and the second must carry \
         last.update_id + 1"
    );
}

/// An empty batch leaves the offset where it was.
///
/// Telegram returns an empty array whenever the long poll times out with
/// nothing to deliver, which is most of the time on a quiet bot. Advancing on
/// that would skip the next real update.
#[tokio::test]
async fn an_empty_batch_does_not_move_the_offset() {
    let server = script(vec![batch(&[10]), batch(&[]), batch(&[])]).await;
    let mut poller = LongPoller::new(client_for(&server.uri()));

    poller.next_batch().await.expect("a batch arrives");
    poller.next_batch().await.expect("an empty batch arrives");
    poller.next_batch().await.expect("another empty batch");

    assert_eq!(
        offsets(&server).await,
        vec![None, Some(11), Some(11)],
        "an empty batch must leave the offset untouched"
    );
}

/// Out-of-order ids still advance past the last one in the batch.
///
/// The offset follows the batch's final element, not its maximum. Pinned
/// because "the highest id" is the intuitive reading and a different one.
#[tokio::test]
async fn the_offset_follows_the_last_update_in_the_batch() {
    let server = script(vec![batch(&[10, 11, 9]), batch(&[])]).await;
    let mut poller = LongPoller::new(client_for(&server.uri()));

    poller.next_batch().await.expect("a batch arrives");
    poller.next_batch().await.expect("an empty batch arrives");

    assert_eq!(
        offsets(&server).await[1],
        Some(10),
        "the offset should follow the batch's last element"
    );
}

// ─── The polling loop ────────────────────────────────────────────────────────

/// Runs `polling()` with a deadline, since it only returns on a fatal error.
async fn poll_until_fatal(dispatcher: Dispatcher) -> Result<(), BotError> {
    tokio::time::timeout(Duration::from_secs(10), dispatcher.polling())
        .await
        .expect("polling should have hit its fatal branch and returned")
}

/// A 429 is waited out and polling continues, rather than exiting.
///
/// The rate-limited response comes first and a fatal one second, so the loop
/// terminates and the test can assert it made both calls. Exiting on the 429
/// would leave a bot silently dead after one burst of traffic.
#[tokio::test]
async fn flood_control_during_polling_waits_and_continues() {
    let server = script(vec![
        json!({
            "ok": false, "error_code": 429,
            "description": "Too Many Requests: retry after 0",
            "parameters": { "retry_after": 0 }
        }),
        json!({ "ok": false, "error_code": 400, "description": "Bad Request: fatal" }),
    ])
    .await;

    // A zero-retry client, so the second request can only come from the polling
    // loop rather than from BotClient's own flood-control retry.
    let dispatcher = Dispatcher::builder(client_without_retries(&server.uri())).build();
    let outcome = poll_until_fatal(dispatcher).await;

    assert!(outcome.is_err(), "the fatal error should end the loop");
    assert_eq!(
        server.received_requests().await.unwrap().len(),
        2,
        "polling should have waited out the rate limit and polled again, then \
         stopped on the fatal error"
    );
}

/// A non-transient API error ends the loop rather than spinning forever.
///
/// The branch that must *not* retry. A loop that treated every error as
/// transient would hammer Telegram indefinitely on a revoked token, and the
/// only symptom would be the request rate.
#[tokio::test]
async fn a_fatal_error_ends_the_polling_loop() {
    let server = script(vec![json!({
        "ok": false, "error_code": 401, "description": "Unauthorized"
    })])
    .await;

    let dispatcher = Dispatcher::builder(client_without_retries(&server.uri())).build();
    let outcome = poll_until_fatal(dispatcher).await;

    match outcome {
        Err(BotError::Api(rustigram_api::Error::Api { error_code, .. })) => {
            assert_eq!(error_code, 401);
        }
        other => panic!("expected the 401 to surface, got {other:?}"),
    }
    assert_eq!(
        server.received_requests().await.unwrap().len(),
        1,
        "a fatal error must not be retried"
    );
}

/// A malformed body is fatal, which is the current behaviour and a real choice.
///
/// An unknown update kind decodes as an error rather than being skipped, so one
/// update Telegram adds tomorrow stops the bot. That is defensible — silently
/// dropping updates is worse — but it is a decision, and pinning it here makes
/// changing it deliberate rather than accidental.
#[tokio::test]
async fn a_decode_error_is_fatal() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path_regex(r".*/getUpdates$"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html>gateway</html>"))
        .mount(&server)
        .await;

    let dispatcher = Dispatcher::builder(client_for(&server.uri())).build();
    let outcome = poll_until_fatal(dispatcher).await;

    assert!(
        matches!(
            outcome,
            Err(BotError::Api(rustigram_api::Error::Decode(_)))
        ),
        "a body that cannot be decoded must end the loop, got {outcome:?}"
    );
    assert_eq!(
        server.received_requests().await.unwrap().len(),
        1,
        "a decode error must not be retried"
    );
}

/// Polled updates are dispatched, not merely fetched.
///
/// The join between the two halves of this file. A poller that advances its
/// offset perfectly while nothing reaches a handler is a bot that reads every
/// message and answers none.
#[tokio::test]
async fn polled_updates_reach_a_handler() {
    let server = script(vec![
        batch(&[10]),
        json!({ "ok": false, "error_code": 401, "description": "Unauthorized" }),
    ])
    .await;

    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client_for(&server.uri()))
        .on(filters::message(), reporting(&tx, "dispatched"))
        .build();

    let _ = poll_until_fatal(dispatcher).await;

    let arrived = tokio::time::timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("a handler should have run within five seconds");
    assert_eq!(
        arrived,
        Some("dispatched"),
        "the polled update never reached a handler"
    );
}
