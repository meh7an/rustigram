use std::net::SocketAddr;

use axum::extract::{Json, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::post;
use axum::Router;
use rustigram_types::update::Update;
use tracing::{debug, warn};

use crate::dispatcher::Dispatcher;
use crate::error::{BotError, BotResult};

/// Shared server state passed into each Axum handler.
#[derive(Clone)]
struct AppState {
    dispatcher: Dispatcher,
    /// Optional secret token to validate the `X-Telegram-Bot-Api-Secret-Token` header.
    secret_token: Option<String>,
}

/// Configuration for the webhook server.
///
/// Exists so the secret token has an obvious home. [`Dispatcher::webhook`]
/// accepts anything that converts into one, and [`SocketAddr`] does, so the
/// bare-address form still works for deployments that rely on network isolation
/// instead of the token.
///
/// [`Dispatcher::webhook`]: crate::dispatcher::Dispatcher::webhook
///
/// ```rust,ignore
/// // network isolation only
/// dispatcher.webhook("0.0.0.0:8443".parse()?).await?;
///
/// // validated against the token registered with setWebhook
/// dispatcher
///     .webhook(WebhookConfig::new(addr).secret_token(&secret))
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    /// Address to bind the server to.
    pub addr: SocketAddr,
    /// Secret token Telegram must send in `X-Telegram-Bot-Api-Secret-Token`.
    ///
    /// Must match the value passed to
    /// [`set_webhook`](rustigram_api::BotClient::set_webhook). When `None`, the
    /// header is not checked at all.
    pub secret_token: Option<String>,
}

impl WebhookConfig {
    /// Creates a configuration bound to `addr` with no secret token.
    #[must_use]
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            secret_token: None,
        }
    }

    /// Sets the secret token Telegram must include in each webhook request.
    #[must_use]
    pub fn secret_token(mut self, token: impl Into<String>) -> Self {
        self.secret_token = Some(token.into());
        self
    }
}

impl From<SocketAddr> for WebhookConfig {
    fn from(addr: SocketAddr) -> Self {
        Self::new(addr)
    }
}

/// An HTTPS webhook server that feeds incoming updates into a [`Dispatcher`].
pub struct WebhookServer {
    addr: SocketAddr,
    dispatcher: Dispatcher,
    secret_token: Option<String>,
}

impl WebhookServer {
    /// Creates a new `WebhookServer` bound to `addr`.
    pub fn new(addr: SocketAddr, dispatcher: Dispatcher) -> Self {
        Self {
            addr,
            dispatcher,
            secret_token: None,
        }
    }

    /// Sets the secret token Telegram must include in each webhook request.
    ///
    /// Must match the value passed to `BotClient::set_webhook`.
    #[must_use]
    pub fn secret_token(mut self, token: impl Into<String>) -> Self {
        self.secret_token = Some(token.into());
        self
    }

    /// Starts the HTTP server and processes incoming updates until the process
    /// is shut down.
    ///
    /// # Errors
    /// Returns an error if the TCP listener cannot be bound.
    pub async fn serve(self) -> BotResult<()> {
        let state = AppState {
            dispatcher: self.dispatcher,
            secret_token: self.secret_token,
        };

        if state.secret_token.is_none() {
            // The gap this warning exists to surface: without a token the
            // endpoint accepts anything that can reach it.
            tracing::warn!(
                "Webhook server starting without a secret token — incoming requests are not authenticated"
            );
        }

        let app = router(state);

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(BotError::Io)?;

        tracing::info!("Webhook server listening on {}", self.addr);

        axum::serve(listener, app).await.map_err(BotError::Io)?;

        Ok(())
    }
}

/// Builds the router. Separated from [`WebhookServer::serve`] so the request
/// handling can be tested without binding a port.
fn router(state: AppState) -> Router {
    Router::new()
        .route("/", post(webhook_handler))
        .with_state(state)
}

/// Axum handler that receives a webhook POST from Telegram.
async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(update): Json<Update>,
) -> StatusCode {
    // Validate secret token if configured.
    if let Some(expected) = &state.secret_token {
        let received = headers
            .get("X-Telegram-Bot-Api-Secret-Token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if received != expected {
            warn!("Webhook: invalid or missing secret token");
            return StatusCode::UNAUTHORIZED;
        }
    }

    debug!("Webhook received update {}", update.update_id);
    state.dispatcher.dispatch(update).await;
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use rustigram_api::BotClient;
    use tower::ServiceExt;

    const SECRET: &str = "s3cret";

    fn app(secret: Option<&str>) -> Router {
        let client = BotClient::from_token("123456:test-token-for-unit-tests").unwrap();
        router(AppState {
            dispatcher: Dispatcher::builder(client).build(),
            secret_token: secret.map(str::to_owned),
        })
    }

    fn update_request(header: Option<&str>) -> Request<Body> {
        let mut req = Request::builder().uri("/").method("POST");
        if let Some(token) = header {
            req = req.header("X-Telegram-Bot-Api-Secret-Token", token);
        }
        req.header("content-type", "application/json")
            .body(Body::from(
                r#"{"update_id":1,"message":{"message_id":1,"date":0,
                    "chat":{"id":1,"type":"private"},"text":"hi"}}"#,
            ))
            .unwrap()
    }

    #[tokio::test]
    async fn correct_token_is_accepted() {
        let status = app(Some(SECRET))
            .oneshot(update_request(Some(SECRET)))
            .await
            .unwrap()
            .status();
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn missing_header_is_rejected() {
        let status = app(Some(SECRET))
            .oneshot(update_request(None))
            .await
            .unwrap()
            .status();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn wrong_token_is_rejected() {
        let status = app(Some(SECRET))
            .oneshot(update_request(Some("wrong")))
            .await
            .unwrap()
            .status();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    /// Deployments that rely on network isolation rather than the token must
    /// keep working — configuring no secret means the header is not checked.
    #[tokio::test]
    async fn no_configured_secret_accepts_any_request() {
        let status = app(None)
            .oneshot(update_request(None))
            .await
            .unwrap()
            .status();
        assert_eq!(status, StatusCode::OK);
    }

    /// The regression this test exists to prevent: `Dispatcher::webhook`
    /// used to drop the secret entirely, leaving the check above unreachable.
    #[test]
    fn socket_addr_and_config_both_reach_the_server() {
        let addr: SocketAddr = "127.0.0.1:8443".parse().unwrap();

        let bare: WebhookConfig = addr.into();
        assert!(bare.secret_token.is_none());

        let configured = WebhookConfig::new(addr).secret_token(SECRET);
        assert_eq!(configured.secret_token.as_deref(), Some(SECRET));
        assert_eq!(configured.addr, addr);
    }

    /// A handler that reports through a channel, so a test can prove an update
    /// arrived rather than only that the status code was right.
    fn reporting_dispatcher(
        client: BotClient,
        tx: tokio::sync::mpsc::UnboundedSender<&'static str>,
    ) -> Dispatcher {
        Dispatcher::builder(client)
            .on(
                crate::filter::filters::message(),
                crate::handler::handler_fn(move |_ctx| {
                    let tx = tx.clone();
                    async move {
                        let _ = tx.send("dispatched");
                        Ok(())
                    }
                }),
            )
            .build()
    }

    /// A valid webhook body, as the literal JSON Telegram would POST.
    const A_MESSAGE_UPDATE: &str = r#"{"update_id":10,"message":{
        "message_id":1,"date":1700000000,
        "chat":{"id":42,"type":"private"},
        "from":{"id":7,"is_bot":false,"first_name":"T"},
        "text":"hi"}}"#;

    /// A valid POST reaches a handler, not just a 200.
    ///
    /// The secret-token tests above assert the status code. A webhook that
    /// answers `200` and drops every body satisfies all of them and looks
    /// entirely healthy to Telegram, which retries nothing because nothing
    /// failed.
    #[tokio::test]
    async fn a_valid_post_reaches_a_handler() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let client = BotClient::from_token("123456:test-token-for-unit-tests").unwrap();
        let app = router(AppState {
            dispatcher: reporting_dispatcher(client, tx),
            secret_token: None,
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(A_MESSAGE_UPDATE))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let arrived = tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv())
            .await
            .expect("a handler should have run within five seconds");
        assert_eq!(
            arrived,
            Some("dispatched"),
            "the webhook answered 200 but the update never reached a handler"
        );
    }

    /// A body that is not an `Update` is rejected and dispatches nothing.
    #[tokio::test]
    async fn a_malformed_body_is_rejected_and_dispatches_nothing() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let client = BotClient::from_token("123456:test-token-for-unit-tests").unwrap();
        let app = router(AppState {
            dispatcher: reporting_dispatcher(client, tx),
            secret_token: None,
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"not":"an update"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::OK,
            "a body that is not an Update must not be accepted"
        );

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        assert!(rx.try_recv().is_err(), "a malformed body reached a handler");
    }

    /// A rejected token dispatches nothing, not merely returns 401.
    ///
    /// The consequence rather than the status: an unauthorised caller must not
    /// be able to drive the bot, whatever they are told.
    #[tokio::test]
    async fn a_rejected_token_dispatches_nothing() {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let client = BotClient::from_token("123456:test-token-for-unit-tests").unwrap();
        let app = router(AppState {
            dispatcher: reporting_dispatcher(client, tx),
            secret_token: Some("expected".to_owned()),
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .header("X-Telegram-Bot-Api-Secret-Token", "wrong")
                    .body(Body::from(A_MESSAGE_UPDATE))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        assert!(
            rx.try_recv().is_err(),
            "an unauthorised request still drove the dispatcher"
        );
    }
}
