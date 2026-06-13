//! Cross-language validation test vectors.
//!
//! Each case uses the same constants and algorithm as the corresponding
//! TypeScript test in `@rustigram/tma-server`. Passing on the same inputs
//! establishes cross-language parity between the two implementations.
//!
//! Shared constants mirror the TypeScript test files:
//!
//! | Constant | Source |
//! |---|---|
//! | `HMAC_BOT_TOKEN` | `validate-hmac.test.ts` |
//! | `ED25519_BOT_ID` | `validate-ed25519.test.ts` |
//!
//! Ed25519 positive-path validation requires Telegram's private key to sign a
//! test vector, which is not available. The positive path is covered by unit
//! tests in `src/validate/ed25519.rs` using runtime-generated key pairs.

use rustigram_miniapp::{
    validate_ed25519, validate_hmac, Ed25519ValidateOpts, HmacValidateOpts, MiniAppError,
};

const HMAC_BOT_TOKEN: &str = "123456789:test-bot-token-for-unit-tests";
const ED25519_BOT_ID: i64 = 12345678;

// ── HMAC-SHA256 ────────────────────────────────────────────────────────────

mod hmac_vectors {
    use super::*;
    use form_urlencoded;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    fn now_secs() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    /// Mirrors `buildValidHmacInitData` from the TypeScript test helpers.
    fn build_valid(token: &str, auth_date: i64) -> String {
        let user_json = r#"{"id":42,"first_name":"Mehran"}"#;

        let mut entries = vec![
            ("auth_date", auth_date.to_string()),
            ("query_id", "test_query_001".to_string()),
            ("user", user_json.to_string()),
        ];
        entries.sort_by_key(|(a, _)| *a);

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

    // ── Valid ──────────────────────────────────────────────────────────────

    #[test]
    fn valid_no_expiry_check() {
        let raw = build_valid(HMAC_BOT_TOKEN, 1_700_000_000);
        assert!(validate_hmac(&raw, HMAC_BOT_TOKEN, HmacValidateOpts::default()).is_ok());
    }

    #[test]
    fn valid_within_max_age_window() {
        let raw = build_valid(HMAC_BOT_TOKEN, now_secs());
        assert!(validate_hmac(
            &raw,
            HMAC_BOT_TOKEN,
            HmacValidateOpts {
                max_age_secs: Some(3600)
            },
        )
        .is_ok());
    }

    #[test]
    fn valid_user_fields_correctly_parsed() {
        let raw = build_valid(HMAC_BOT_TOKEN, 1_700_000_000);
        let data = validate_hmac(&raw, HMAC_BOT_TOKEN, HmacValidateOpts::default()).unwrap();
        let user = data.user.expect("user field must be populated");
        assert_eq!(user.id, 42);
        assert_eq!(user.first_name, "Mehran");
    }

    // ── Invalid ────────────────────────────────────────────────────────────

    #[test]
    fn invalid_wrong_bot_token() {
        let raw = build_valid(HMAC_BOT_TOKEN, 1_700_000_000);
        let err = validate_hmac(&raw, "999:wrong-token", HmacValidateOpts::default()).unwrap_err();
        assert!(matches!(err, MiniAppError::InvalidHmac));
    }

    #[test]
    fn invalid_field_appended_after_signing() {
        let raw = build_valid(HMAC_BOT_TOKEN, 1_700_000_000);
        // Appending a field changes the data-check-string, breaking the hash.
        let tampered = format!("{raw}&injected=payload");
        let err =
            validate_hmac(&tampered, HMAC_BOT_TOKEN, HmacValidateOpts::default()).unwrap_err();
        assert!(matches!(err, MiniAppError::InvalidHmac));
    }

    #[test]
    fn invalid_expired_auth_date() {
        let stale = now_secs() - 7_200; // 2 hours ago; max_age is 1 hour.
        let raw = build_valid(HMAC_BOT_TOKEN, stale);
        let err = validate_hmac(
            &raw,
            HMAC_BOT_TOKEN,
            HmacValidateOpts {
                max_age_secs: Some(3600),
            },
        )
        .unwrap_err();
        assert!(matches!(err, MiniAppError::Expired));
    }

    #[test]
    fn invalid_missing_hash_field() {
        let err = validate_hmac(
            "auth_date=1700000000&user=%7B%22id%22%3A1%7D",
            HMAC_BOT_TOKEN,
            HmacValidateOpts::default(),
        )
        .unwrap_err();
        assert!(matches!(err, MiniAppError::MalformedInitData(_)));
    }

    #[test]
    fn invalid_hash_not_valid_hex() {
        let err = validate_hmac(
            "auth_date=1700000000&hash=not-hex!!",
            HMAC_BOT_TOKEN,
            HmacValidateOpts::default(),
        )
        .unwrap_err();
        assert!(matches!(err, MiniAppError::InvalidHmac));
    }
}

// ── Ed25519 ────────────────────────────────────────────────────────────────

mod ed25519_vectors {
    use super::*;

    #[test]
    fn invalid_signature_field_absent() {
        let err = validate_ed25519(
            "auth_date=1700000000&hash=placeholder",
            ED25519_BOT_ID,
            Ed25519ValidateOpts::default(),
        )
        .unwrap_err();
        assert!(matches!(err, MiniAppError::InvalidSignature));
    }

    #[test]
    fn invalid_signature_not_valid_base64url() {
        let err = validate_ed25519(
            "auth_date=1700000000&hash=placeholder&signature=!!!notbase64!!!",
            ED25519_BOT_ID,
            Ed25519ValidateOpts::default(),
        )
        .unwrap_err();
        assert!(matches!(err, MiniAppError::InvalidSignature));
    }

    #[test]
    fn invalid_signature_wrong_length_after_decode() {
        // Valid base64url but decodes to fewer than 64 bytes — wrong for Ed25519.
        let err = validate_ed25519(
            "auth_date=1700000000&hash=placeholder&signature=dGVzdA",
            ED25519_BOT_ID,
            Ed25519ValidateOpts::default(),
        )
        .unwrap_err();
        assert!(matches!(err, MiniAppError::InvalidSignature));
    }
}
