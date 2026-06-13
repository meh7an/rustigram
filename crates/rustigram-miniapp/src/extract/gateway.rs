//! Tower middleware that signs validated requests for downstream trust.
//!
//! In Mode 3 (Rust gateway + SolidStart BFF), Rust validates initData via
//! [`super::TmaInitData`] and then needs to prove to SolidStart that the
//! request was validated — without sharing the bot token.
//!
//! [`TmaGatewayLayer`] adds an `X-Tma-Gateway` header containing an
//! HMAC-SHA256 of the raw initData string signed with [`GatewaySecret`].
//! SolidStart verifies this signature via `createTmaMiddleware(null, {
//! gatewaySecret })` and only parses requests that carry a valid header.
//!
//! # Security model
//!
//! - `BOT_TOKEN` — stays in Rust only, never reaches SolidStart.
//! - `GATEWAY_SECRET` — shared between Rust and SolidStart. Proves a
//!   request passed through the validated Rust gateway.
//! - Without `GATEWAY_SECRET`, trust is network-level only (VPC / firewall).
//!   Add it for cryptographic proof when network isolation alone is not
//!   sufficient.
//!
//! # Layer ordering
//!
//! Add [`TmaGatewayLayer`] **outside** (after) [`super::BotTokenLayer`] so
//! the bot token is available when [`super::TmaInitData`] runs:
//!
//! ```rust,ignore
//! Router::new()
//!     .route("/tma", post(handler))
//!     .layer(TmaGatewayLayer(GatewaySecret(
//!         std::env::var("GATEWAY_SECRET").unwrap(),
//!     )))
//!     .layer(BotTokenLayer(BotToken(
//!         std::env::var("BOT_TOKEN").unwrap(),
//!     )));
//! ```

use axum::http::{HeaderName, HeaderValue, Request};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::task::{Context, Poll};
use tower::{Layer, Service};

static GATEWAY_HEADER: HeaderName = HeaderName::from_static("x-tma-gateway");

/// Tower layer that adds an HMAC-signed `X-Tma-Gateway` header to every
/// request that has passed [`TmaInitData`] extraction.
///
/// SolidStart (or any downstream) verifies this header via
/// `createTmaMiddleware(null, { gatewaySecret })` so it can trust the
/// request came from the validated Rust gateway rather than directly from
/// the internet.
///
/// Add this layer AFTER `BotTokenLayer` on your router.
#[derive(Clone)]
pub struct TmaGatewayLayer(pub GatewaySecret);

/// Newtype wrapping the gateway shared secret.
#[derive(Debug, Clone)]
pub struct GatewaySecret(pub String);

impl<S> Layer<S> for TmaGatewayLayer {
    type Service = TmaGatewayService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TmaGatewayService {
            inner,
            secret: self.0.clone(),
        }
    }
}

/// Produced by [`TmaGatewayLayer`].
///
/// For each request carrying an `X-Telegram-Init-Data` header, computes
/// `HMAC-SHA256(gateway_secret, raw_init_data)` and injects the result as
/// `X-Tma-Gateway` before delegating to the inner service.
///
/// Requests without the initData header pass through unmodified — the
/// downstream [`super::TmaInitData`] extractor will reject them if the
/// route requires validation.
#[derive(Clone)]
pub struct TmaGatewayService<S> {
    inner: S,
    secret: GatewaySecret,
}

impl<S, ReqBody> Service<Request<ReqBody>> for TmaGatewayService<S>
where
    S: Service<Request<ReqBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // Sign the raw initData with the gateway secret so downstream can
        // verify it without knowing the bot token.
        if let Some(init_data) = req.headers().get("x-telegram-init-data").cloned() {
            type HmacSha256 = Hmac<Sha256>;
            let mut mac = HmacSha256::new_from_slice(self.secret.0.as_bytes()).unwrap();
            mac.update(init_data.as_bytes());
            let sig = hex::encode(mac.finalize().into_bytes());
            if let Ok(val) = HeaderValue::from_str(&sig) {
                req.headers_mut().insert(GATEWAY_HEADER.clone(), val);
            }
        }
        self.inner.call(req)
    }
}
