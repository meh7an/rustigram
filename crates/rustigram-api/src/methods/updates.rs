use std::future::{Future, IntoFuture};
use std::pin::Pin;

use reqwest::multipart::{Form, Part};
use serde::Serialize;

use rustigram_types::update::Update;
use rustigram_types::webhook::WebhookInfo;

use rustigram_types::file::InputFile;

use crate::client::BotClient;
use crate::error::Result;

// ─── getUpdates ───────────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
/// Parameters sent with a `getUpdates` request.
pub struct GetUpdatesParams {
    /// Identifier of the first update to return. Confirms all updates before this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Maximum number of updates to return (1–100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    /// Timeout in seconds for long polling. `0` for short polling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// List of update types to receive. All types received if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// Builder for the [`getUpdates`](https://core.telegram.org/bots/api#getupdates) method.
pub struct GetUpdates {
    client: BotClient,
    params: GetUpdatesParams,
}

impl GetUpdates {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: GetUpdatesParams::default(),
        }
    }

    /// Sets the update offset — all updates with `update_id < offset` are
    /// acknowledged and will not be returned again.
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }

    /// Limits the number of updates returned (1–100, default 100).
    pub fn limit(mut self, limit: u8) -> Self {
        self.params.limit = Some(limit.clamp(1, 100));
        self
    }

    /// Sets the long-poll server-side timeout in seconds. Use 0 for short polling.
    pub fn timeout(mut self, secs: u32) -> Self {
        self.params.timeout = Some(secs);
        self
    }

    /// Restricts which update types are returned (e.g. `["message", "callback_query"]`).
    pub fn allowed_updates(mut self, types: Vec<impl Into<String>>) -> Self {
        self.params.allowed_updates = Some(types.into_iter().map(Into::into).collect());
        self
    }
}

impl IntoFuture for GetUpdates {
    type Output = Result<Vec<Update>>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("getUpdates", &self.params).await })
    }
}

// ─── setWebhook ───────────────────────────────────────────────────────────────

/// Parameters for a `setWebhook` request.
#[derive(Serialize)]
pub struct SetWebhookParams {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_token: Option<String>,
}

/// Builder for the [`setWebhook`](https://core.telegram.org/bots/api#setwebhook) method.
pub struct SetWebhook {
    client: BotClient,
    params: SetWebhookParams,
    /// Public key certificate for a self-signed setup.
    ///
    /// Held outside `params` because uploading one switches the request from
    /// JSON to multipart, and an `InputFile` is not meaningfully serialisable
    /// into the JSON body.
    certificate: Option<InputFile>,
}

impl SetWebhook {
    pub(crate) fn new(client: BotClient, url: impl Into<String>) -> Self {
        Self {
            client,
            params: SetWebhookParams {
                url: url.into(),
                ip_address: None,
                max_connections: None,
                allowed_updates: None,
                drop_pending_updates: None,
                secret_token: None,
            },
            certificate: None,
        }
    }

    /// Uploads a public key certificate so Telegram can verify a self-signed
    /// setup.
    ///
    /// Supplying one switches the request to multipart. Telegram requires the
    /// certificate to be uploaded as a file, so `InputFile::Url` and
    /// `InputFile::FileId` are not accepted here — use `InputFile::Bytes`.
    pub fn certificate(mut self, certificate: InputFile) -> Self {
        self.certificate = Some(certificate);
        self
    }
    /// Overrides the resolved IP address of the webhook server.
    pub fn ip_address(mut self, ip: impl Into<String>) -> Self {
        self.params.ip_address = Some(ip.into());
        self
    }
    /// Sets the maximum number of concurrent HTTPS connections (1–100, default 40).
    pub fn max_connections(mut self, n: u8) -> Self {
        self.params.max_connections = Some(n.clamp(1, 100));
        self
    }
    /// List of update types to receive. All types received if omitted.
    pub fn allowed_updates(mut self, types: Vec<impl Into<String>>) -> Self {
        self.params.allowed_updates = Some(types.into_iter().map(Into::into).collect());
        self
    }
    /// If `true`, the webhook server will remove all pending updates.
    pub fn drop_pending_updates(mut self, v: bool) -> Self {
        self.params.drop_pending_updates = Some(v);
        self
    }
    /// Sets the secret token Telegram sends in `X-Telegram-Bot-Api-Secret-Token`
    /// on every webhook request.
    pub fn secret_token(mut self, token: impl Into<String>) -> Self {
        self.params.secret_token = Some(token.into());
        self
    }
}

impl IntoFuture for SetWebhook {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let Some(certificate) = self.certificate else {
                return self.client.post_json("setWebhook", &self.params).await;
            };

            let InputFile::Bytes {
                filename,
                data,
                mime_type,
            } = certificate
            else {
                return Err(crate::error::Error::MissingParam(
                    "setWebhook certificate must be InputFile::Bytes — Telegram \
                     requires the certificate to be uploaded, not referenced",
                ));
            };

            let part = Part::bytes(data)
                .file_name(filename)
                .mime_str(&mime_type)
                .map_err(|e| crate::error::Error::Decode(e.to_string()))?;

            let p = &self.params;
            let mut form = Form::new()
                .part("certificate", part)
                .text("url", p.url.clone());
            if let Some(v) = &p.ip_address {
                form = form.text("ip_address", v.clone());
            }
            if let Some(v) = p.max_connections {
                form = form.text("max_connections", v.to_string());
            }
            if let Some(v) = &p.allowed_updates {
                if let Ok(json) = serde_json::to_string(v) {
                    form = form.text("allowed_updates", json);
                }
            }
            if let Some(v) = p.drop_pending_updates {
                form = form.text("drop_pending_updates", v.to_string());
            }
            if let Some(v) = &p.secret_token {
                form = form.text("secret_token", v.clone());
            }

            self.client.post_multipart("setWebhook", form).await
        })
    }
}

// ─── deleteWebhook ────────────────────────────────────────────────────────────

#[derive(Serialize, Default)]
struct DeleteWebhookParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
}

/// Builder for the [`deleteWebhook`](https://core.telegram.org/bots/api#deletewebhook) method.
pub struct DeleteWebhook {
    client: BotClient,
    params: DeleteWebhookParams,
}

impl DeleteWebhook {
    pub(crate) fn new(client: BotClient) -> Self {
        Self {
            client,
            params: DeleteWebhookParams::default(),
        }
    }
    /// If `true`, the webhook server will remove all pending updates.
    pub fn drop_pending_updates(mut self, v: bool) -> Self {
        self.params.drop_pending_updates = Some(v);
        self
    }
}

impl IntoFuture for DeleteWebhook {
    type Output = Result<bool>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.client.post_json("deleteWebhook", &self.params).await })
    }
}

// ─── getWebhookInfo ───────────────────────────────────────────────────────────

/// Builder for the [`getWebhookInfo`](https://core.telegram.org/bots/api#getwebhookinfo) method.
pub struct GetWebhookInfo {
    client: BotClient,
}

impl GetWebhookInfo {
    pub(crate) fn new(client: BotClient) -> Self {
        Self { client }
    }
}

impl IntoFuture for GetWebhookInfo {
    type Output = Result<WebhookInfo>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            self.client
                .post_json("getWebhookInfo", &serde_json::json!({}))
                .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::BotClient;

    fn client() -> BotClient {
        BotClient::from_token("123456:test-token-for-unit-tests").unwrap()
    }

    /// Without a certificate the request must stay on the JSON path, unchanged.
    #[test]
    fn without_certificate_the_params_serialize_as_before() {
        let w = SetWebhook::new(client(), "https://example.com")
            .secret_token("s3cret")
            .max_connections(40);
        let json = serde_json::to_value(&w.params).unwrap();
        assert_eq!(json["url"], "https://example.com");
        assert_eq!(json["secret_token"], "s3cret");
        assert_eq!(json["max_connections"], 40);
        assert!(w.certificate.is_none());
        // The certificate is deliberately not part of the JSON body.
        assert!(json.get("certificate").is_none());
    }

    #[test]
    fn certificate_is_held_outside_the_json_params() {
        let w = SetWebhook::new(client(), "https://example.com").certificate(InputFile::Bytes {
            filename: "cert.pem".to_owned(),
            data: b"-----BEGIN CERTIFICATE-----".to_vec(),
            mime_type: "application/x-pem-file".to_owned(),
        });
        assert!(w.certificate.is_some());
        assert!(serde_json::to_value(&w.params)
            .unwrap()
            .get("certificate")
            .is_none());
    }

    /// Telegram requires the certificate to be uploaded, so a URL or file_id
    /// reference is rejected with a clear error rather than silently ignored.
    #[tokio::test]
    async fn non_uploaded_certificate_is_rejected() {
        let err = SetWebhook::new(client(), "https://example.com")
            .certificate(InputFile::Url("https://example.com/cert.pem".to_owned()))
            .await
            .unwrap_err();
        assert!(
            matches!(err, crate::error::Error::MissingParam(m) if m.contains("must be InputFile::Bytes")),
            "unexpected error: {err:?}"
        );
    }
}
