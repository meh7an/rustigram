//! Routing: which handler runs, and what happens when it goes wrong.
//!
//! The filters and the `Context` accessors were already covered. The dispatcher
//! itself was not — nothing exercised first-match-wins, the fallback, or what
//! becomes of a handler that fails. Those are the parts a bot's behaviour is
//! actually made of: a misordered route sends the wrong reply, and a handler
//! that takes the process down with it turns one bad update into an outage.
//!
//! # Why every test here uses a channel
//!
//! [`Dispatcher::dispatch`] does not run a handler — it `tokio::spawn`s one and
//! returns. Setting a flag in the handler and reading it after the `await` is a
//! race the test usually wins, which is worse than one it always loses: it
//! passes on a fast machine and fails in CI. So handlers report through an
//! `mpsc` channel and the test awaits the report with a timeout. Absence is
//! asserted by draining after that, never by checking a flag.

mod dispatch_support;

use dispatch_support::{
    assert_no_further_reports, chat, client, command_update, failing, message, next_report,
    reporting, reports, text_update,
};
use rustigram_bot::context::Context;
use rustigram_bot::dispatcher::Dispatcher;
use rustigram_bot::filter::{FilterExt, filters};
use rustigram_bot::handler::handler_fn;
use rustigram_types::chat::ChatType;
use rustigram_types::update::{Update, UpdateKind};
use rustigram_types::user::ChatId;
use std::time::Duration;

// ─── Routing ─────────────────────────────────────────────────────────────────

/// The first matching route wins and the rest do not run.
///
/// Both routes here match. Registration order is the whole contract — a bot
/// puts its specific routes before its general ones, and if that stopped being
/// honoured every specific route would be shadowed.
#[tokio::test]
async fn the_first_matching_route_wins() {
    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client())
        .on(filters::message(), reporting(&tx, "first"))
        .on(filters::message(), reporting(&tx, "second"))
        .build();

    dispatcher.dispatch(text_update("hello")).await;

    assert_eq!(next_report(&mut rx).await, "first");
    assert_no_further_reports(&mut rx).await;
}

/// A route whose filter does not match is skipped, and a later one runs.
#[tokio::test]
async fn a_non_matching_route_does_not_shadow_a_later_one() {
    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client())
        .on(filters::command("start"), reporting(&tx, "command"))
        .on(filters::text("hello"), reporting(&tx, "text"))
        .build();

    dispatcher.dispatch(text_update("hello")).await;

    assert_eq!(next_report(&mut rx).await, "text");
    assert_no_further_reports(&mut rx).await;
}

/// The fallback runs when nothing matches.
#[tokio::test]
async fn the_fallback_runs_when_no_route_matches() {
    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client())
        .on(filters::command("start"), reporting(&tx, "route"))
        .fallback(reporting(&tx, "fallback"))
        .build();

    dispatcher.dispatch(text_update("not a command")).await;

    assert_eq!(next_report(&mut rx).await, "fallback");
    assert_no_further_reports(&mut rx).await;
}

/// The fallback does **not** run when a route matched.
///
/// The direction that actually breaks: a fallback firing alongside a matched
/// route sends every user two replies, and only in production.
#[tokio::test]
async fn the_fallback_stays_quiet_when_a_route_matches() {
    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client())
        .on(filters::message(), reporting(&tx, "route"))
        .fallback(reporting(&tx, "fallback"))
        .build();

    dispatcher.dispatch(text_update("hello")).await;

    assert_eq!(next_report(&mut rx).await, "route");
    assert_no_further_reports(&mut rx).await;
}

/// An update matching nothing, with no fallback, is simply dropped.
#[tokio::test]
async fn an_unmatched_update_without_a_fallback_is_harmless() {
    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client())
        .on(filters::command("start"), reporting(&tx, "route"))
        .build();

    dispatcher.dispatch(text_update("nothing matches this")).await;

    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(
        rx.try_recv().is_err(),
        "no handler should have run for an unmatched update"
    );
}

// ─── Failure isolation ───────────────────────────────────────────────────────

/// A handler returning `Err` is logged and the dispatcher carries on.
///
/// The next update must still be routed. A dispatcher that stopped after the
/// first failing handler would take a bot down on one malformed message, and the
/// symptom — silence — looks identical to a network problem.
#[tokio::test]
async fn a_failing_handler_does_not_stop_the_dispatcher() {
    let (tx, mut rx) = reports();
    let dispatcher = Dispatcher::builder(client())
        .on(filters::message(), failing(&tx, "failed"))
        .build();

    dispatcher.dispatch(text_update("first")).await;
    assert_eq!(next_report(&mut rx).await, "failed");

    dispatcher.dispatch(text_update("second")).await;
    assert_eq!(
        next_report(&mut rx).await,
        "failed",
        "the second update was not routed, so the first failure stopped the \
         dispatcher"
    );
}

/// A panicking handler does not take the dispatcher down either.
///
/// `tokio::spawn` contains the panic in its own task. This pins that, because
/// the containment is a property of how the dispatcher runs handlers rather than
/// of the handlers themselves — running them inline would change it silently.
#[tokio::test]
async fn a_panicking_handler_does_not_take_down_the_dispatcher() {
    let (tx, mut rx) = reports();
    let panicking = handler_fn(move |_ctx: Context| async move {
        panic!("handler panicked on purpose");
    });
    let dispatcher = Dispatcher::builder(client())
        .on(filters::text("boom"), panicking)
        .on(filters::text("ping"), reporting(&tx, "survived"))
        .build();

    dispatcher.dispatch(text_update("boom")).await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    dispatcher.dispatch(text_update("ping")).await;
    assert_eq!(
        next_report(&mut rx).await,
        "survived",
        "the dispatcher stopped routing after a handler panicked"
    );
}

// ─── Filter composition ──────────────────────────────────────────────────────

/// Combinators nest, and nesting means what it reads like.
///
/// The flat cases were already covered. Nesting is where precedence mistakes
/// live: `a.and(b).or(c)` and `a.and(b.or(c))` differ, and both compile.
#[tokio::test]
async fn nested_filter_combinators_route_as_written() {
    let (tx, mut rx) = reports();

    // (private AND text "yes") OR command "start"
    let filter = filters::private()
        .and(filters::text("yes"))
        .or(filters::command("start"));

    let dispatcher = Dispatcher::builder(client())
        .on(filter, reporting(&tx, "matched"))
        .build();

    dispatcher.dispatch(text_update("yes")).await;
    assert_eq!(next_report(&mut rx).await, "matched");

    dispatcher.dispatch(command_update("start")).await;
    assert_eq!(
        next_report(&mut rx).await,
        "matched",
        "the right-hand branch of the `or` never matched"
    );

    dispatcher.dispatch(text_update("no")).await;
    assert_no_further_reports(&mut rx).await;
}

/// `not` inverts a composed filter, not just a leaf one.
#[tokio::test]
async fn not_inverts_a_composed_filter() {
    let (tx, mut rx) = reports();
    let filter = filters::text("skip").or(filters::text("ignore")).not();

    let dispatcher = Dispatcher::builder(client())
        .on(filter, reporting(&tx, "matched"))
        .build();

    dispatcher.dispatch(text_update("skip")).await;
    dispatcher.dispatch(text_update("ignore")).await;
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(
        rx.try_recv().is_err(),
        "the negated filter matched something it should have excluded"
    );

    dispatcher.dispatch(text_update("anything else")).await;
    assert_eq!(next_report(&mut rx).await, "matched");
}

// ─── Every update kind ───────────────────────────────────────────────────────

/// One update of every kind reaches a handler.
///
/// `filters::any()` matches everything, so a kind that fails to dispatch is one
/// the dispatcher cannot route at all — the fixture built it, and it went
/// nowhere. Table-driven over the whole enum so a new Bot API update kind is
/// covered by adding one line rather than a test.
#[tokio::test]
async fn every_update_kind_reaches_a_handler() {
    let mut unrouted = Vec::new();

    for (name, kind) in every_update_kind() {
        let (tx, mut rx) = reports();
        let dispatcher = Dispatcher::builder(client())
            .on(filters::any(), reporting(&tx, "handled"))
            .build();

        dispatcher.dispatch(Update { update_id: 1, kind }).await;

        let arrived = tokio::time::timeout(Duration::from_millis(500), rx.recv())
            .await
            .ok()
            .flatten()
            .is_some();
        if !arrived {
            unrouted.push(name);
        }
    }

    assert!(
        unrouted.is_empty(),
        "{} update kind(s) were never routed to a handler: {unrouted:?}",
        unrouted.len()
    );
}

/// The table above covers every variant the crate declares.
///
/// Rust cannot enumerate its own variants at runtime, so the list is written
/// out; this counts the declaration to keep it honest. Without it a new update
/// kind would go untested while the suite stayed green.
#[test]
fn the_update_kind_table_is_complete() {
    let source = include_str!("../../rustigram-types/src/update.rs");
    let body = source
        .split("pub enum UpdateKind {")
        .nth(1)
        .and_then(|s| s.split("\n}").next())
        .expect("the UpdateKind declaration");
    let declared = body
        .lines()
        .filter(|line| {
            let line = line.trim();
            line.starts_with(|c: char| c.is_ascii_uppercase()) && line.contains('(')
        })
        .count();

    assert_eq!(
        every_update_kind().len(),
        declared,
        "UpdateKind declares {declared} variants but the table covers {}; add \
         the new one so it is dispatched at least once",
        every_update_kind().len()
    );
}

/// One value per `UpdateKind` variant, each a payload Telegram could send.
///
/// Built from JSON because eighteen of these types have no `Default`, and
/// `ChatBoostRemoved` is `#[non_exhaustive]` as well — from another crate it can
/// only be produced by deserialising. The JSON form has the further advantage
/// that a Bot API field addition does not turn every fixture into a compile
/// error, which is why the sibling test file moved away from struct literals.
fn every_update_kind() -> Vec<(&'static str, UpdateKind)> {
    use serde_json::json;

    macro_rules! kind {
        ($variant:ident, $value:expr) => {
            (
                stringify!($variant),
                UpdateKind::$variant(
                    serde_json::from_value($value)
                        .unwrap_or_else(|e| panic!("{} fixture: {e}", stringify!($variant))),
                ),
            )
        };
    }

    let msg = json!({
        "message_id": 1, "date": 1_700_000_000,
        "chat": { "id": 42, "type": "private" },
        "from": { "id": 7, "is_bot": false, "first_name": "T" },
        "text": "hi"
    });
    let from = json!({ "id": 7, "is_bot": false, "first_name": "T" });
    let chat_json = json!({ "id": 42, "type": "private" });
    let member = json!({ "status": "member", "user": from });
    let boost_source = json!({ "source": "premium", "user": from });

    vec![
        kind!(Message, msg.clone()),
        kind!(EditedMessage, msg.clone()),
        kind!(ChannelPost, msg.clone()),
        kind!(EditedChannelPost, msg.clone()),
        kind!(BusinessMessage, msg.clone()),
        kind!(EditedBusinessMessage, msg.clone()),
        kind!(InlineQuery, json!({ "id": "1", "from": from, "query": "q", "offset": "" })),
        kind!(ChosenInlineResult, json!({ "result_id": "1", "from": from, "query": "q" })),
        kind!(CallbackQuery, json!({ "id": "1", "from": from, "chat_instance": "c" })),
        kind!(ShippingQuery, json!({
            "id": "1", "from": from, "invoice_payload": "p",
            "shipping_address": {
                "country_code": "TR", "state": "", "city": "Istanbul",
                "street_line1": "a", "street_line2": "", "post_code": "34000"
            }
        })),
        kind!(PreCheckoutQuery, json!({
            "id": "1", "from": from, "currency": "XTR",
            "total_amount": 1, "invoice_payload": "p"
        })),
        kind!(Poll, json!({
            "id": "1", "question": "q", "options": [], "total_voter_count": 0,
            "is_closed": false, "is_anonymous": true, "type": "regular",
            "allows_multiple_answers": false, "allows_revoting": false,
            "members_only": false
        })),
        kind!(PollAnswer, json!({
            "poll_id": "1", "option_ids": [0], "option_persistent_ids": ["a"]
        })),
        kind!(MyChatMember, json!({
            "chat": chat_json, "from": from, "date": 1_700_000_000,
            "old_chat_member": member, "new_chat_member": member
        })),
        kind!(ChatMember, json!({
            "chat": chat_json, "from": from, "date": 1_700_000_000,
            "old_chat_member": member, "new_chat_member": member
        })),
        kind!(ChatJoinRequest, json!({
            "chat": chat_json, "from": from, "user_chat_id": 7, "date": 1_700_000_000
        })),
        kind!(MessageReaction, json!({
            "chat": chat_json, "message_id": 1, "date": 1_700_000_000,
            "old_reaction": [], "new_reaction": []
        })),
        kind!(MessageReactionCount, json!({
            "chat": chat_json, "message_id": 1, "date": 1_700_000_000, "reactions": []
        })),
        kind!(ChatBoost, json!({
            "chat": chat_json,
            "boost": {
                "boost_id": "b", "add_date": 1_700_000_000,
                "expiration_date": 1_700_000_001, "source": boost_source
            }
        })),
        kind!(RemovedChatBoost, json!({
            "chat": chat_json, "boost_id": "b",
            "remove_date": 1_700_000_000, "source": boost_source
        })),
        kind!(ManagedBot, json!({ "user": from, "bot": from })),
        kind!(BusinessConnection, json!({
            "id": "1", "user": from, "user_chat_id": 7,
            "date": 1_700_000_000, "is_enabled": true
        })),
        kind!(DeletedBusinessMessages, json!({
            "business_connection_id": "1", "chat": chat_json, "message_ids": [1]
        })),
        kind!(GuestMessage, msg.clone()),
        kind!(PurchasedPaidMedia, json!({ "from": from, "paid_media_payload": "p" })),
        kind!(Subscription, json!({
            "user": from, "invoice_payload": "p", "state": "active"
        })),
    ]
}

/// `Context` reads the same message out of every kind that carries one.
///
/// Six update kinds wrap a `Message`, and a bot written against `ctx.text()`
/// expects it to work for all of them. Reading only `UpdateKind::Message` would
/// make a bot silently blind to channel posts and business messages.
#[tokio::test]
async fn context_reads_a_message_from_every_kind_that_carries_one() {
    let carriers: Vec<(&str, UpdateKind)> = vec![
        ("Message", UpdateKind::Message(message("hi"))),
        (
            "EditedMessage",
            UpdateKind::EditedMessage(message("hi")),
        ),
        (
            "ChannelPost",
            UpdateKind::ChannelPost(message("hi")),
        ),
        (
            "EditedChannelPost",
            UpdateKind::EditedChannelPost(message("hi")),
        ),
        (
            "BusinessMessage",
            UpdateKind::BusinessMessage(message("hi")),
        ),
        (
            "EditedBusinessMessage",
            UpdateKind::EditedBusinessMessage(message("hi")),
        ),
    ];

    let mut blind = Vec::new();
    for (name, kind) in carriers {
        let ctx = Context::new(Update { update_id: 1, kind }, client());
        if ctx.text() != Some("hi") || ctx.chat_id() != Some(ChatId::Id(42)) || ctx.from_id() != Some(7) {
            blind.push(format!(
                "  {name}: text={:?} chat_id={:?} from_id={:?}",
                ctx.text(),
                ctx.chat_id(),
                ctx.from_id()
            ));
        }
    }

    assert!(
        blind.is_empty(),
        "{} update kind(s) carry a message that `Context` cannot read, so a bot \
         using ctx.text() is blind to them:\n{}",
        blind.len(),
        blind.join("\n")
    );
}

/// A group chat is reported as one, so `Filter::group` has something to read.
#[test]
fn context_reports_the_chat_it_was_given() {
    let mut msg = message("hi");
    msg.chat = chat(ChatType::Supergroup);
    let ctx = Context::new(
        Update {
            update_id: 1,
            kind: UpdateKind::Message(msg),
        },
        client(),
    );
    assert_eq!(ctx.chat_id(), Some(ChatId::Id(42)));
}
