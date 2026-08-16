//! Fixtures and synchronisation shared by the routing and update-source tests.
//!
//! Handlers report through an `mpsc` channel rather than setting a flag, because
//! `Dispatcher::dispatch` spawns its handler and returns. Reading a flag after
//! the `await` is a race the test usually wins — green locally, flaky in CI.

#![allow(dead_code)] // each test binary uses a different subset

use rustigram_api::{BotClient, ClientConfig};
use rustigram_bot::context::Context;
use rustigram_bot::error::BotError;
use rustigram_bot::handler::{BoxHandler, handler_fn};
use rustigram_types::chat::{Chat, ChatType};
use rustigram_types::message::Message;
use rustigram_types::update::{Update, UpdateKind};
use rustigram_types::user::User;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

// ─── Fixtures ────────────────────────────────────────────────────────────────

/// A client pointed at `base_url`.
pub fn client_for(base_url: &str) -> BotClient {
    let config = ClientConfig::new("123456:test-token-for-dispatch-tests")
        .expect("the test token is well-formed")
        .api_base_url(base_url.to_owned());
    BotClient::new(config).expect("client builds")
}

/// A client that does not retry, so a repeated request can only come from the
/// caller's own loop.
///
/// `BotClient` retries flood control internally up to `max_retries` (three by
/// default). Counting requests against a rate-limiting server therefore cannot
/// tell the client's retry from the polling loop's — both produce more than one.
/// Setting the budget to zero makes the polling loop the only possible source.
pub fn client_without_retries(base_url: &str) -> BotClient {
    let config = ClientConfig::new("123456:test-token-for-dispatch-tests")
        .expect("the test token is well-formed")
        .api_base_url(base_url.to_owned())
        .max_retries(0);
    BotClient::new(config).expect("client builds")
}

/// A client pointed nowhere, for tests where no call may leave the process.
pub fn client() -> BotClient {
    client_for("http://127.0.0.1:1")
}

/// A message update carrying `text`, with the given update id.
pub fn message_update(update_id: i64, text: &str) -> Update {
    Update {
        update_id,
        kind: UpdateKind::Message(message(text)),
    }
}

pub fn user() -> User {
    let mut user = User::default();
    user.id = 7;
    user.first_name = "Test".into();
    user
}

pub fn chat(kind: ChatType) -> Chat {
    let mut chat = Chat::default();
    chat.id = 42;
    chat.kind = kind;
    chat
}

pub fn message(text: &str) -> Message {
    let mut message = Message::default();
    message.message_id = 1;
    message.date = 1_700_000_000;
    message.chat = chat(ChatType::Private);
    message.from = Some(user());
    message.text = Some(text.to_owned());
    message
}

/// A `/command` update, with the `bot_command` entity Telegram attaches.
///
/// The entity is what `filters::command` reads — text starting with a slash is
/// not a command on its own, and a fixture without it silently matches nothing.
pub fn command_update(command: &str) -> Update {
    use rustigram_types::message::{MessageEntity, MessageEntityKind};
    let text = format!("/{command}");
    let length = text.len() as u32;
    let mut update = text_update(&text);
    if let UpdateKind::Message(ref mut msg) = update.kind {
        msg.entities = Some(vec![MessageEntity {
            kind: MessageEntityKind::BotCommand,
            offset: 0,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
            unix_time: None,
            date_time_format: None,
        }]);
    }
    update
}

pub fn text_update(text: &str) -> Update {
    Update {
        update_id: 1,
        kind: UpdateKind::Message(message(text)),
    }
}

// ─── Reporting ───────────────────────────────────────────────────────────────

/// A channel handlers report through, so the test can await them.
pub type Reports = (UnboundedSender<&'static str>, UnboundedReceiver<&'static str>);

pub fn reports() -> Reports {
    unbounded_channel()
}

/// A handler that reports `name` and succeeds.
pub fn reporting(tx: &UnboundedSender<&'static str>, name: &'static str) -> BoxHandler {
    let tx = tx.clone();
    handler_fn(move |_ctx: Context| {
        let tx = tx.clone();
        async move {
            let _ = tx.send(name);
            Ok(())
        }
    })
}

/// A handler that reports `name` and then fails.
pub fn failing(tx: &UnboundedSender<&'static str>, name: &'static str) -> BoxHandler {
    let tx = tx.clone();
    handler_fn(move |_ctx: Context| {
        let tx = tx.clone();
        async move {
            let _ = tx.send(name);
            Err(BotError::Handler(anyhow::anyhow!("handler failed on purpose")))
        }
    })
}

/// Awaits the next report, failing with a usable message rather than hanging.
pub async fn next_report(rx: &mut UnboundedReceiver<&'static str>) -> &'static str {
    tokio::time::timeout(Duration::from_secs(5), rx.recv())
        .await
        .expect("a handler should have reported within five seconds")
        .expect("the report channel is still open")
}

/// Asserts nothing else reported.
///
/// Called after the expected report has arrived, so any extra handler has
/// already been spawned; a short settle covers the gap between spawning and
/// running.
pub async fn assert_no_further_reports(rx: &mut UnboundedReceiver<&'static str>) {
    tokio::time::sleep(Duration::from_millis(50)).await;
    if let Ok(extra) = rx.try_recv() {
        panic!("`{extra}` also ran, but only one handler should have");
    }
}

