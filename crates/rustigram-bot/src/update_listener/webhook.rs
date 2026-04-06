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

        let app = Router::new()
            .route("/", post(webhook_handler))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(self.addr)
            .await
            .map_err(BotError::Io)?;

        tracing::info!("Webhook server listening on {}", self.addr);

        axum::serve(listener, app).await.map_err(BotError::Io)?;

        Ok(())
    }
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
