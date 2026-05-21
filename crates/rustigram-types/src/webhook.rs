use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Current status of a registered webhook.
///
/// Returned by [`getWebhookInfo`](https://core.telegram.org/bots/api#getwebhookinfo).
/// Check `last_error_message` and `last_error_date` to diagnose delivery
/// failures.
pub struct WebhookInfo {
    /// Webhook URL.
    pub url: String,
    /// `true` if a custom certificate was provided for the webhook.
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery.
    pub pending_update_count: u32,
    /// Currently used webhook IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Unix timestamp of the most recent error when delivering updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    /// Description of the most recent delivery error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Unix timestamp of the most recent error when synchronising with the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    /// List of update types the bot is subscribed to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
