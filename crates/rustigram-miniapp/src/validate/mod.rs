//! initData validation — HMAC-SHA256 and Ed25519.
//!
//! Choose the right function based on your use case:
//! - [`validate_hmac`] — first-party validation using the bot token.
//! - [`validate_ed25519`] — third-party validation without the bot token.

pub mod ed25519;
pub mod hmac;

pub use ed25519::validate_ed25519;
pub use hmac::validate_hmac;

/// Options for HMAC-SHA256 initData validation.
#[derive(Debug, Default, Clone)]
pub struct HmacValidateOpts {
    /// Reject initData whose `auth_date` is older than this many seconds.
    /// When `None`, no expiry check is performed.
    pub max_age_secs: Option<u64>,
}

/// Telegram deployment environment for Ed25519 key selection.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum TelegramEnv {
    /// Production Telegram environment (default).
    #[default]
    Production,
    /// Telegram test environment.
    Test,
}

/// Options for Ed25519 initData signature validation.
#[derive(Debug, Default, Clone)]
pub struct Ed25519ValidateOpts {
    /// Reject initData whose `auth_date` is older than this many seconds.
    pub max_age_secs: Option<u64>,
    /// Which Telegram environment's public key to verify against.
    pub env: TelegramEnv,
}
