# rustigram-miniapp

[![Crates.io](https://img.shields.io/crates/v/rustigram-miniapp.svg)](https://crates.io/crates/rustigram-miniapp)
[![Docs.rs](https://img.shields.io/badge/docs-rustigram.io-blue.svg)](https://rustigram.io/docs/rustigram_miniapp/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE)

Server-side Telegram Mini App bridge for the [rustigram](../../README.md) framework.

Validates `initData` from `window.Telegram.WebApp.initData`, provides typed Axum
extractors, and keeps TypeScript types in sync with Rust structs via
[ts-rs](https://github.com/Aleph-Alpha/ts-rs).

```toml
[dependencies]
rustigram-miniapp = "0.10.0"
```

---

## What it does

| Feature                | Description                                                                             |
| ---------------------- | --------------------------------------------------------------------------------------- |
| HMAC-SHA256 validation | First-party validation against the bot token                                            |
| Ed25519 validation     | Third-party validation using Telegram's public key                                      |
| Axum extractor         | `TmaInitData` — validates on extraction, rejects automatically                          |
| Tower middleware       | `BotTokenLayer` — injects the bot token into every request                              |
| Type generation        | `cargo run --example gen-types --features ts` syncs Rust types to `@rustigram/tma-core` |

---

## Quick start

### Axum extractor

Add `BotTokenLayer` to your router once. Then declare `TmaInitData` as a handler
parameter — initData is validated automatically on every request.

```rust
use axum::{Router, routing::post};
use rustigram_miniapp::{BotToken, BotTokenLayer, extract::TmaInitData};

async fn tma_handler(TmaInitData(data): TmaInitData) -> &'static str {
    println!("user: {:?}", data.user);
    "ok"
}

let app = Router::new()
    .route("/tma", post(tma_handler))
    .layer(BotTokenLayer(BotToken(std::env::var("BOT_TOKEN").unwrap())));
```

The extractor reads initData from the `X-Tma-Init-Data` header, falling back to
`Authorization: tma <data>`. It returns `401 Unauthorized` on auth failure and
`400 Bad Request` on malformed input.

### Manual validation

```rust
use rustigram_miniapp::{validate_hmac, validate_ed25519, HmacValidateOpts, Ed25519ValidateOpts};

// First-party: validate against the bot token
let data = validate_hmac(
    &raw_init_data,
    &bot_token,
    HmacValidateOpts { max_age_secs: Some(3600) },
)?;

// Third-party: validate without the bot token (Bot API 8.0+)
let data = validate_ed25519(
    &raw_init_data,
    bot_id,
    Ed25519ValidateOpts::default(),
)?;

println!("user id: {}", data.user.unwrap().id);
```

---

## Feature flags

| Flag                       | Effect                                                                         |
| -------------------------- | ------------------------------------------------------------------------------ |
| `ts`                       | Enables `ts-rs` derives on all public types. Required for `gen-types`.         |
| `tma` _(on rustigram-bot)_ | Adds `Context::tma_data()` and `filters::web_app_data()` to the bot framework. |

---

## Type generation

When using rustigram as your Rust backend, Rust structs are the source of truth
for all TMA types. Generate TypeScript declarations into `@rustigram/tma-core`:

```bash
# Set once in .cargo/config.toml
# [env]
# TMA_TYPES_OUT = "/path/to/rustigram-tma/packages/core/src/generated"

cargo run --example gen-types --features ts
```

Import generated types in TypeScript:

```ts
import type { WebAppUser, WebAppInitData } from "@rustigram/tma-core/generated";
```

---

## Error handling

All functions return `rustigram_miniapp::Result<T>`. The error type implements
`axum::response::IntoResponse` for direct use in handlers.

| Variant             | HTTP status | Cause                                   |
| ------------------- | ----------- | --------------------------------------- |
| `InvalidHmac`       | 401         | Hash mismatch or wrong bot token        |
| `InvalidSignature`  | 401         | Ed25519 signature did not verify        |
| `Expired`           | 401         | `auth_date` older than `max_age_secs`   |
| `MalformedInitData` | 400         | Missing required field or invalid JSON  |
| `MissingBotToken`   | 500         | `BotTokenLayer` not added to the router |

---

## Related

- [rustigram](../../README.md) — the full Rust bot framework
- [@rustigram/tma-core](https://www.npmjs.com/package/@rustigram/tma-core) — TypeScript Mini App primitives
- [@rustigram/tma-server](https://www.npmjs.com/package/@rustigram/tma-server) — TypeScript server-side validation
