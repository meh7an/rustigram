use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use crate::context::Context;
use crate::error::BotResult;

/// Type alias for the boxed future returned by handlers.
pub type HandlerFuture = Pin<Box<dyn Future<Output = BotResult<()>> + Send>>;

/// Core trait for all update handlers.
///
/// Implementors receive a [`Context`] and return a future that resolves to
/// `BotResult<()>`. Returning `Ok(())` allows subsequent handlers to run;
/// the dispatcher stops the chain on `Err`.
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    /// Processes an incoming update and returns whether the dispatcher should continue.
    async fn handle(&self, ctx: Context) -> BotResult<()>;
}

/// A boxed, type-erased handler.
pub type BoxHandler = Arc<dyn Handler>;

/// Wraps an async function (or closure) as a [`Handler`].
pub struct HandlerFn<F> {
    f: F,
}

impl<F, Fut> HandlerFn<F>
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<()>> + Send + 'static,
{
    /// Creates a new `HandlerFn` from a function or closure.
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

#[async_trait]
impl<F, Fut> Handler for HandlerFn<F>
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<()>> + Send + 'static,
{
    async fn handle(&self, ctx: Context) -> BotResult<()> {
        (self.f)(ctx).await
    }
}

/// Creates a [`BoxHandler`] from any async function or closure.
///
/// # Example
///
/// ```rust,ignore
/// use rustigram_bot::{handler_fn, Context, BotResult};
///
/// let h = handler_fn(|ctx: Context| async move {
///     if let Some(r) = ctx.reply("pong") { r.await?; }
///     Ok(())
/// });
/// ```
pub fn handler_fn<F, Fut>(f: F) -> BoxHandler
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<()>> + Send + 'static,
{
    Arc::new(HandlerFn::new(f))
}

/// A handler that logs an error and continues the chain.
pub struct LoggingHandler {
    inner: BoxHandler,
}

impl LoggingHandler {
    /// Wraps a handler in a logging layer that catches errors and logs them
    /// instead of propagating them, allowing the dispatcher to continue.
    pub fn wrap(inner: BoxHandler) -> BoxHandler {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl Handler for LoggingHandler {
    async fn handle(&self, ctx: Context) -> BotResult<()> {
        let update_id = ctx.update_id();
        if let Err(e) = self.inner.handle(ctx).await {
            tracing::error!("Handler error on update {}: {}", update_id, e);
        }
        Ok(())
    }
}

// Arc<dyn Handler> itself implements Handler so BoxHandler can be passed
// directly to DispatcherBuilder::on() without unwrapping.
#[async_trait]
impl Handler for std::sync::Arc<dyn Handler> {
    async fn handle(&self, ctx: Context) -> BotResult<()> {
        (**self).handle(ctx).await
    }
}
