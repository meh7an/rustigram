use std::sync::Arc;

use rustigram_api::BotClient;
use rustigram_types::update::Update;
use tracing::{debug, error, info, warn};

use crate::context::Context;
use crate::error::{BotError, BotResult};
use crate::filter::Filter;
use crate::handler::{BoxHandler, Handler};

/// A single routing rule: a filter + handler pair.
struct Route {
    filter: Box<dyn Filter>,
    handler: BoxHandler,
}

/// Builder for constructing a [`Dispatcher`].
///
/// Obtain one via [`Dispatcher::builder`] or [`crate::bot::Bot::dispatcher`].
/// Register filter-handler pairs with [`on`](DispatcherBuilder::on),
/// then call [`build`](DispatcherBuilder::build) to finalise.
pub struct DispatcherBuilder {
    client: BotClient,
    routes: Vec<Route>,
    error_handler: Option<BoxHandler>,
}

impl DispatcherBuilder {
    fn new(client: BotClient) -> Self {
        Self {
            client,
            routes: Vec::new(),
            error_handler: None,
        }
    }

    /// Registers a filter-handler pair.
    ///
    /// Routes are evaluated in registration order. The first route whose filter
    /// returns `true` for the incoming update wins — subsequent routes are not
    /// checked.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// dispatcher_builder
    ///     .on(filters::command("start"), handler_fn(start_handler))
    ///     .on(filters::message(),        handler_fn(echo_handler));
    /// ```
    #[must_use]
    pub fn on<F, H>(mut self, filter: F, handler: H) -> Self
    where
        F: Filter,
        H: Handler,
    {
        self.routes.push(Route {
            filter: Box::new(filter),
            handler: Arc::new(handler),
        });
        self
    }

    /// Registers a handler called when no other route matches.
    ///
    /// If no fallback is set, unmatched updates are silently ignored.
    #[must_use]
    pub fn fallback<H: Handler>(mut self, handler: H) -> Self {
        self.error_handler = Some(Arc::new(handler));
        self
    }

    /// Finalises the builder into a [`Dispatcher`].
    #[must_use]
    pub fn build(self) -> Dispatcher {
        Dispatcher {
            client: self.client,
            routes: Arc::new(self.routes),
            fallback: self.error_handler,
        }
    }
}

/// Routes incoming updates to the first matching handler.
///
/// # Concurrency
///
/// Each update is dispatched in its own `tokio::spawn` task so handlers run
/// concurrently. The dispatcher itself is cheaply cloneable (all state is
/// `Arc`-backed).
#[derive(Clone)]
/// Routes incoming updates to the first matching handler.
///
/// Build a dispatcher with [`Dispatcher::builder`] or the convenience
/// method [`crate::bot::Bot::dispatcher`]. Once built, start receiving updates with
/// [`polling`](Dispatcher::polling) or [`webhook`](Dispatcher::webhook).
///
/// # Concurrency
///
/// `Dispatcher` is cheaply cloneable — all state is `Arc`-backed.
/// Each update is dispatched in its own [`tokio::spawn`] task so handlers
/// run concurrently.
pub struct Dispatcher {
    client: BotClient,
    routes: Arc<Vec<Route>>,
    fallback: Option<BoxHandler>,
}

impl Dispatcher {
    /// Creates a new [`DispatcherBuilder`] for the given client.
    #[must_use]
    pub fn builder(client: BotClient) -> DispatcherBuilder {
        DispatcherBuilder::new(client)
    }

    /// Dispatches a single update.
    ///
    /// Finds the first matching route and spawns its handler. If no route
    /// matches and a fallback is registered, the fallback is called.
    pub async fn dispatch(&self, update: Update) {
        let ctx = Context::new(update, self.client.clone());
        debug!("Dispatching update {}", ctx.update_id());

        for route in self.routes.as_ref() {
            if route.filter.check(&ctx) {
                let handler = route.handler.clone();
                let ctx = ctx.clone();
                tokio::spawn(async move {
                    if let Err(e) = handler.handle(ctx).await {
                        error!("Handler error: {}", e);
                    }
                });
                return;
            }
        }

        // No route matched.
        if let Some(fallback) = &self.fallback {
            let fallback = fallback.clone();
            let ctx = ctx.clone();
            tokio::spawn(async move {
                if let Err(e) = fallback.handle(ctx).await {
                    error!("Fallback handler error: {}", e);
                }
            });
        } else {
            debug!("No handler matched update {}", ctx.update_id());
        }
    }

    /// Starts a long-polling loop, blocking until a fatal error occurs.
    ///
    /// Transient network errors (timeouts, connection resets) are retried
    /// automatically after a short delay. Rate-limit responses (HTTP 429)
    /// honour the `retry_after` value from Telegram. Only non-recoverable
    /// errors (invalid token, unexpected API error) propagate to the caller.
    ///
    /// # Errors
    ///
    /// Returns a [`BotError`] on unrecoverable failure.
    pub async fn polling(self) -> BotResult<()> {
        use crate::update_listener::polling::LongPoller;
        info!("Starting long-polling dispatcher");

        let mut poller = LongPoller::new(self.client.clone());

        loop {
            match poller.next_batch().await {
                Ok(updates) => {
                    for update in updates {
                        self.dispatch(update).await;
                    }
                }
                Err(BotError::Api(rustigram_api::Error::RateLimit { retry_after })) => {
                    warn!("Rate-limited during polling, waiting {}s", retry_after);
                    tokio::time::sleep(std::time::Duration::from_secs(u64::from(retry_after)))
                        .await;
                }
                Err(BotError::Api(rustigram_api::Error::Http(ref e)))
                    if e.is_timeout() || e.is_connect() =>
                {
                    warn!(
                        "Transient network error during polling, retrying in 3s: {}",
                        e
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                }
                Err(e) => {
                    error!("Fatal polling error: {}", e);
                    return Err(e);
                }
            }
        }
    }

    /// Starts an axum-based webhook server on `addr`, blocking until shutdown.
    ///
    /// Telegram must already be configured to deliver updates to your server.
    /// Call [`rustigram_api::BotClient::set_webhook`] once before starting this server, or
    /// use `update_listener::webhook::WebhookServer` directly for more control.
    ///
    /// # Errors
    ///
    /// Returns a [`BotError`] if the TCP listener cannot be bound.
    pub async fn webhook(self, addr: std::net::SocketAddr) -> BotResult<()> {
        use crate::update_listener::webhook::WebhookServer;
        info!("Starting webhook dispatcher on {}", addr);
        WebhookServer::new(addr, self).serve().await
    }
}
