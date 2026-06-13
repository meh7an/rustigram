//! Ed25519 initData signature validation.
//!
//! Implements third-party validation using Telegram's published public keys.
//! Mirrors `validateInitDataSignature` from `@rustigram/tma-server`.
//! No bot token is required — suitable for external services.

use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};

use crate::error::{MiniAppError, Result};
use crate::parse::{build_ed25519_check_string, build_init_data, get_auth_date, parse_raw};
use crate::types::WebAppInitData;

use super::{Ed25519ValidateOpts, TelegramEnv};

// Telegram's published Ed25519 public keys — static per Telegram's spec.
// Update only on an explicit Telegram announcement.
// @see https://core.telegram.org/bots/webapps#validating-data-for-third-party-use
const PRODUCTION_KEY_HEX: &str = "e7bf03a2fa4602af4580703d88dda5bb59f32ed8b02a56c187fe7d34caed242d";
const TEST_KEY_HEX: &str = "40055058a4ee38156a06562e52eece92a771bcd8346a8c4615cb7376eddf72ec";

/// Validate Telegram Mini App `initData` using Ed25519 signature verification.
///
/// This is the third-party validation method — no bot token required. Use
/// this when your backend is not the bot's primary server but still needs to
/// trust TMA-originated requests.
///
/// # Arguments
///
/// - `init_data` — raw `window.Telegram.WebApp.initData` string.
/// - `bot_id` — numeric bot ID (the part before `:` in the bot token).
/// - `opts` — expiry check and environment selection.
///
/// # Algorithm
///
/// 1. Build `data_check_string = "${bot_id}:WebAppData\n<sorted fields>"`.
/// 2. Decode the `signature` field from base64url.
/// 3. Verify against Telegram's hardcoded Ed25519 public key for the env.
///
/// Mirrors `validateInitDataSignature` from `@rustigram/tma-server` exactly.
///
/// # Errors
///
/// - [`MiniAppError::InvalidSignature`] — `signature` absent or did not verify.
/// - [`MiniAppError::MalformedInitData`] — `auth_date` or `hash` missing.
/// - [`MiniAppError::Expired`] — `auth_date` older than `opts.max_age_secs`.
pub fn validate_ed25519(
    init_data: &str,
    bot_id: i64,
    opts: Ed25519ValidateOpts,
) -> Result<WebAppInitData> {
    let params = parse_raw(init_data);

    let signature_b64url = params
        .iter()
        .find(|(k, _)| k == "signature")
        .map(|(_, v)| v.as_str())
        .ok_or(MiniAppError::InvalidSignature)?;

    let public_key_hex = match opts.env {
        TelegramEnv::Production => PRODUCTION_KEY_HEX,
        TelegramEnv::Test => TEST_KEY_HEX,
    };

    let verifying_key = load_verifying_key(public_key_hex)?;
    let signature = decode_signature(signature_b64url)?;
    let dcs = build_ed25519_check_string(&params, bot_id);

    verifying_key
        .verify_strict(dcs.as_bytes(), &signature)
        .map_err(|_| MiniAppError::InvalidSignature)?;

    check_expiry(&params, opts.max_age_secs)?;

    build_init_data(&params)
}

fn load_verifying_key(hex_key: &str) -> Result<VerifyingKey> {
    let bytes = hex::decode(hex_key).map_err(|_| MiniAppError::InvalidSignature)?;
    let key_bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| MiniAppError::InvalidSignature)?;
    VerifyingKey::from_bytes(&key_bytes).map_err(|_| MiniAppError::InvalidSignature)
}

fn decode_signature(b64url: &str) -> Result<Signature> {
    // Telegram omits padding from base64url output — strip any stray `=` defensively.
    let stripped = b64url.trim_end_matches('=');
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(stripped)
        .map_err(|_| MiniAppError::InvalidSignature)?;

    let sig_bytes: [u8; 64] = bytes
        .try_into()
        .map_err(|_| MiniAppError::InvalidSignature)?;
    Ok(Signature::from_bytes(&sig_bytes))
}

fn check_expiry(params: &crate::parse::RawParams, max_age_secs: Option<u64>) -> Result<()> {
    let Some(max_age) = max_age_secs else {
        return Ok(());
    };

    let auth_date = get_auth_date(params).unwrap_or(0) as u64;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if now.saturating_sub(auth_date) > max_age {
        return Err(MiniAppError::Expired);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use form_urlencoded;
    use rand::rngs::OsRng;

    const BOT_ID: i64 = 12345678;

    // Mirrors buildValidEd25519InitData from @rustigram/tma-server/tests/helpers.ts
    fn build_valid_init_data(bot_id: i64, auth_date: i64) -> (String, VerifyingKey) {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let user_json = r#"{"id":99999,"first_name":"Test"}"#;
        let params: Vec<(&str, String)> = vec![
            ("auth_date", auth_date.to_string()),
            ("user", user_json.to_string()),
        ];

        let mut sorted = params.clone();
        sorted.sort_by_key(|(a, _)| *a);
        let fields = sorted
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");
        let dcs = format!("{bot_id}:WebAppData\n{fields}");

        use ed25519_dalek::Signer;
        let signature_bytes = signing_key.sign(dcs.as_bytes()).to_bytes();
        let signature_b64url =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature_bytes);

        let raw = form_urlencoded::Serializer::new(String::new())
            .append_pair("auth_date", &auth_date.to_string())
            .append_pair("user", user_json)
            .append_pair("signature", &signature_b64url)
            .finish();

        // Inject a placeholder hash so build_init_data doesn't complain.
        let raw = format!("{raw}&hash=placeholder");

        (raw, verifying_key)
    }

    // Injects a custom public key for testing — overrides the hardcoded constant.
    fn validate_with_key(
        init_data: &str,
        bot_id: i64,
        verifying_key: &VerifyingKey,
        max_age_secs: Option<u64>,
    ) -> Result<WebAppInitData> {
        let params = parse_raw(init_data);

        let signature_b64url = params
            .iter()
            .find(|(k, _)| k == "signature")
            .map(|(_, v)| v.as_str())
            .ok_or(MiniAppError::InvalidSignature)?;

        let signature = decode_signature(signature_b64url)?;
        let dcs = build_ed25519_check_string(&params, bot_id);

        verifying_key
            .verify_strict(dcs.as_bytes(), &signature)
            .map_err(|_| MiniAppError::InvalidSignature)?;

        if let Some(max_age) = max_age_secs {
            let auth_date = get_auth_date(&params).unwrap_or(0) as u64;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            if now.saturating_sub(auth_date) > max_age {
                return Err(MiniAppError::Expired);
            }
        }

        build_init_data(&params)
    }

    #[test]
    fn valid_signature_returns_ok() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let (raw, vk) = build_valid_init_data(BOT_ID, now);
        let result = validate_with_key(&raw, BOT_ID, &vk, None);
        assert!(result.is_ok());
    }

    #[test]
    fn wrong_key_returns_invalid_signature() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let (raw, _) = build_valid_init_data(BOT_ID, now);
        let other_key = SigningKey::generate(&mut OsRng).verifying_key();
        let err = validate_with_key(&raw, BOT_ID, &other_key, None);
        assert!(matches!(err, Err(MiniAppError::InvalidSignature)));
    }

    #[test]
    fn missing_signature_field_returns_invalid_signature() {
        let err = validate_ed25519(
            "auth_date=1700000000&hash=abc",
            BOT_ID,
            Ed25519ValidateOpts::default(),
        );
        assert!(matches!(err, Err(MiniAppError::InvalidSignature)));
    }

    #[test]
    fn expired_auth_date_returns_expired() {
        let stale = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - 10_800; // 3 hours ago
        let (raw, vk) = build_valid_init_data(BOT_ID, stale);
        let err = validate_with_key(&raw, BOT_ID, &vk, Some(3600));
        assert!(matches!(err, Err(MiniAppError::Expired)));
    }
}
