//! HMAC-SHA256 initData validation.
//!
//! Implements the first-party validation algorithm defined in the Telegram
//! Bot API docs. Mirrors `validateInitData` from `@rustigram/tma-server`.

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::{MiniAppError, Result};
use crate::parse::{build_hmac_check_string, build_init_data, get_auth_date, parse_raw};
use crate::types::WebAppInitData;

use super::HmacValidateOpts;

type HmacSha256 = Hmac<Sha256>;

/// Validate Telegram Mini App `initData` using HMAC-SHA256 against the bot token.
///
/// This is the standard first-party validation method. The bot token is the
/// secret — keep it server-side and never expose it to clients.
///
/// # Algorithm
///
/// 1. `secret_key = HMAC-SHA256(key = "WebAppData", message = bot_token)`
/// 2. `hash = HMAC-SHA256(key = secret_key, message = data_check_string)`
/// 3. Constant-time compare against the `hash` field in `init_data`.
///
/// Mirrors `validateInitData` from `@rustigram/tma-server` byte-for-byte.
///
/// # Errors
///
/// - [`MiniAppError::MalformedInitData`] — `auth_date` or `hash` missing, or
///   a sub-object field contained unparseable JSON.
/// - [`MiniAppError::InvalidHmac`] — hash mismatch (tampered data or wrong token).
/// - [`MiniAppError::Expired`] — `auth_date` older than `opts.max_age_secs`.
pub fn validate_hmac(
    init_data: &str,
    bot_token: &str,
    opts: HmacValidateOpts,
) -> Result<WebAppInitData> {
    let params = parse_raw(init_data);

    let received_hash = params
        .iter()
        .find(|(k, _)| k == "hash")
        .map(|(_, v)| v.as_str())
        .ok_or_else(|| MiniAppError::MalformedInitData("`hash` field missing".into()))?;

    let received_bytes = hex::decode(received_hash).map_err(|_| MiniAppError::InvalidHmac)?;

    // Step 1: secret_key = HMAC-SHA256(key="WebAppData", message=bot_token)
    let mut mac = HmacSha256::new_from_slice(b"WebAppData").expect("HMAC accepts any key length");
    mac.update(bot_token.as_bytes());
    let secret_key = mac.finalize().into_bytes();

    // Step 2: hash = HMAC-SHA256(key=secret_key, message=data_check_string)
    let dcs = build_hmac_check_string(&params);
    let mut mac = HmacSha256::new_from_slice(&secret_key).expect("HMAC accepts any key length");
    mac.update(dcs.as_bytes());

    // verify_slice uses constant-time comparison internally.
    mac.verify_slice(&received_bytes)
        .map_err(|_| MiniAppError::InvalidHmac)?;

    check_expiry(&params, opts.max_age_secs)?;

    build_init_data(&params)
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
    use form_urlencoded;

    const BOT_TOKEN: &str = "123456789:test-bot-token-for-unit-tests";

    // Mirrors buildValidHmacInitData from @rustigram/tma-server/tests/helpers.ts
    fn build_valid_init_data(token: &str, auth_date: i64) -> String {
        let user_json = r#"{"id":42,"first_name":"Mehran"}"#;

        let mut entries = vec![
            ("auth_date", auth_date.to_string()),
            ("query_id", "test_query_001".to_string()),
            ("user", user_json.to_string()),
        ];
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));

        let dcs = entries
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");

        let mut mac = HmacSha256::new_from_slice(b"WebAppData").unwrap();
        mac.update(token.as_bytes());
        let sk = mac.finalize().into_bytes();

        let mut mac = HmacSha256::new_from_slice(&sk).unwrap();
        mac.update(dcs.as_bytes());
        let hash = hex::encode(mac.finalize().into_bytes());

        form_urlencoded::Serializer::new(String::new())
            .append_pair("auth_date", &auth_date.to_string())
            .append_pair("query_id", "test_query_001")
            .append_pair("user", user_json)
            .append_pair("hash", &hash)
            .finish()
    }

    #[test]
    fn valid_init_data_returns_ok() {
        let raw = build_valid_init_data(BOT_TOKEN, 1700000000);
        let result = validate_hmac(&raw, BOT_TOKEN, HmacValidateOpts::default());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().user.unwrap().first_name, "Mehran");
    }

    #[test]
    fn wrong_token_returns_invalid_hmac() {
        let raw = build_valid_init_data(BOT_TOKEN, 1700000000);
        let err = validate_hmac(&raw, "wrong-token", HmacValidateOpts::default());
        assert!(matches!(err, Err(MiniAppError::InvalidHmac)));
    }

    #[test]
    fn tampered_hash_returns_invalid_hmac() {
        let raw = build_valid_init_data(BOT_TOKEN, 1700000000);
        // let tampered = raw.replace(|c: char| c == '&' || c.is_ascii_hexdigit(), "x");
        // Simpler tamper: append a known-bad hash param
        let tampered = format!("{}&extra=1", raw).replace("hash=", "hash=dead");
        let err = validate_hmac(&tampered, BOT_TOKEN, HmacValidateOpts::default());
        assert!(matches!(err, Err(MiniAppError::InvalidHmac)));
    }

    #[test]
    fn missing_hash_field_returns_malformed() {
        let err = validate_hmac(
            "auth_date=1700000000&user=%7B%7D",
            BOT_TOKEN,
            HmacValidateOpts::default(),
        );
        assert!(matches!(err, Err(MiniAppError::MalformedInitData(_))));
    }

    #[test]
    fn expired_auth_date_returns_expired() {
        // auth_date 3 hours in the past, max_age 1 hour
        let stale = chrono_now_secs() - 10_800;
        let raw = build_valid_init_data(BOT_TOKEN, stale as i64);
        let err = validate_hmac(
            &raw,
            BOT_TOKEN,
            HmacValidateOpts {
                max_age_secs: Some(3600),
            },
        );
        assert!(matches!(err, Err(MiniAppError::Expired)));
    }

    #[test]
    fn fresh_auth_date_within_max_age_returns_ok() {
        let now = chrono_now_secs() as i64;
        let raw = build_valid_init_data(BOT_TOKEN, now);
        let result = validate_hmac(
            &raw,
            BOT_TOKEN,
            HmacValidateOpts {
                max_age_secs: Some(3600),
            },
        );
        assert!(result.is_ok());
    }

    fn chrono_now_secs() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
