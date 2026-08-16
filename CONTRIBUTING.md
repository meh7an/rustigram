# Contributing to rustigram

## Running the suite

```bash
cargo test --all-features --workspace
```

No test reaches the network. Every HTTP test runs against a `wiremock` server on
an ephemeral loopback port, and the Bot API spec is read from a committed
snapshot rather than fetched. The suite passes offline, and it must stay that
way — a test that needs the internet is a test that fails on a train.

## How the suite is organised

| Area | Where | What it establishes |
|---|---|---|
| Spec conformance | `crates/rustigram-types/tests/conformance_*.rs` | Every type in the Bot API decodes, re-encodes, and keeps its fields |
| Public surface | `conformance_surface.rs` | No orphan types, no untyped `serde_json::Value` escape hatches |
| Request construction | `crates/rustigram-api/tests/request_construction.rs` | Every method is reachable and puts its required parameters on the wire |
| Send paths | `media_send_paths.rs` | A media option survives whichever way the file travels |
| Responses and errors | `response_handling.rs` | Flood control, retries, error mapping, downloads |
| Routing | `crates/rustigram-bot/tests/dispatch_routing.rs` | Which handler runs, and what happens when one fails |
| Update sources | `update_sources.rs` | Polling offsets and webhook delivery |

The conformance tests are **table-driven over a committed snapshot** of the Bot
API. Adding a Bot API version means regenerating that snapshot, not writing new
tests:

```bash
python3 ~/.claude/skills/telegram-docs-setup/scripts/make_spec_snapshot.py \
    --tdocs tdocs --out crates/rustigram-types/tests/spec/bot-api-10.2.json
```

The snapshot is committed on purpose. A spec change should arrive as a reviewable
diff, never as a silent shift in what the tests believe.

## Found a bug? Write the test first, then watch it fail

**A test is done only when it has failed on the real bug.** Write the test,
reintroduce the defect, watch the test go red, then restore the fix. A test that
has never failed proves nothing — it may be asserting something that was always
true, or nothing at all.

This is not a formality. Three concrete ways it has caught worthless tests in
this repository:

**The mutation did not mutate.** A retry test was verified by widening a loop
bound, which turned out not to govern the retry count — the guard *inside* the
loop did. The test stayed green, and looked verified. If your mutation leaves the
suite passing, first ask whether you changed the behaviour you meant to.

**The test passed for the wrong reason.** A flood-control test asserted the mock
server saw two or more requests after a 429. It did — but `BotClient` retries
internally up to `max_retries`, so the count could not distinguish the client's
retry from the polling loop's. Disabling the loop's retry branch entirely left
the test green. It now uses a zero-retry client and an exact count.

**The edit silently did not apply.** A check on `RichBlockMap` "passed" under
mutation because the pattern being replaced was `Location`, and the field is
declared `crate::chat::Location`. Assert that your mutation landed before
concluding anything from the result.

## Writing a good failure message

Every failure names the offending item. A loop over 388 types that reports
`assertion failed` is useless at three in the morning:

```rust
assert!(
    missing.is_empty(),
    "{} spec field(s) are missing from their Rust type:\n{}",
    missing.len(),
    missing.join("\n")
);
```

Collect failures and report them together rather than stopping at the first, so
one run tells you the whole story.

## Exceptions and allowlists

Several suites carry a list of documented exceptions — types that cannot round
trip, `serde_json::Value` sites that are genuinely correct, builder options
deliberately not exposed. Two rules:

1. **Every entry states its reason**, in prose, in the constant itself.
2. **Every list has a freshness test** asserting each entry still describes
   something real. An exception for code that no longer exists is dead
   permission, and the next person to need one inherits it.

Adding an exception should feel uncomfortable. It is permission for something to
go unchecked indefinitely.

## Guards that protect the suite from itself

A test that silently checks nothing is worse than no test: it reports success
over a shrinking surface. Several guards exist for this and should be kept:

- Scans assert they found a plausible number of files, types, or fields before
  checking anything, so a layout change fails loudly instead of passing
  vacuously.
- The generated-payload sweeps assert the maximal payloads carry far more fields
  than the required-only ones, so a generator change cannot quietly reduce one
  to the other.
- CI's **Suite Integrity** job asserts every conformance suite actually ran and
  that no test was skipped. A conformance test that stops running looks exactly
  like one that passes.

## Before opening a pull request

```bash
cargo fmt --all
cargo clippy --workspace --all-features --all-targets
cargo test --all-features --workspace
```

CI additionally runs `cargo-semver-checks` on pull requests. Note that it does
**not** detect a changed public field type, so state breaking changes explicitly
in the pull request description rather than relying on the check.
