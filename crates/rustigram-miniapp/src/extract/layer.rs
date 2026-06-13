//! [`BotTokenLayer`] Tower middleware implementation.

use std::task::{Context, Poll};

use axum::http::Request;
use tower::{Layer, Service};

use super::BotToken;

/// Tower middleware layer that injects a [`BotToken`] extension into every
/// request.
///
/// Add this to your Axum router so that [`TmaInitData`] can read the token
/// during extraction without coupling handlers to a specific `AppState` shape.
///
/// # Example
///
/// ```rust,ignore
/// let app = Router::new()
///     .route("/tma", post(my_handler))
///     .layer(BotTokenLayer(BotToken(std::env::var("BOT_TOKEN").unwrap())));
/// ```
#[derive(Clone)]
pub struct BotTokenLayer(pub BotToken);

impl<S> Layer<S> for BotTokenLayer {
    type Service = BotTokenService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BotTokenService {
            inner,
            token: self.0.clone(),
        }
    }
}

/// Produced by [`BotTokenLayer`]. Clones the [`BotToken`] into each request's
/// extension map before delegating to the inner service.
#[derive(Clone)]
pub struct BotTokenService<S> {
    inner: S,
    token: BotToken,
}

impl<S, ReqBody> Service<Request<ReqBody>> for BotTokenService<S>
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
        req.extensions_mut().insert(self.token.clone());
        self.inner.call(req)
    }
}
