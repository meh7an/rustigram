//! Raw initData query string parsing utilities.
//!
//! All functions in this module are `pub(crate)` — they are implementation
//! details of the validation layer and not part of the public API.

use form_urlencoded;

use crate::error::{MiniAppError, Result};
use crate::types::{InitDataChatType, WebAppChat, WebAppInitData, WebAppUser};

pub(crate) type RawParams = Vec<(String, String)>;

/// URL-decodes a raw `initData` query string into an ordered list of key-value
/// pairs. Order is preserved — callers sort before use where needed.
pub(crate) fn parse_raw(raw: &str) -> RawParams {
    form_urlencoded::parse(raw.as_bytes())
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}

/// Builds the HMAC-SHA256 data-check string.
///
/// Algorithm: all params excluding `hash`, sorted alphabetically by key,
/// formatted as `key=value` pairs joined by `\n`. Mirrors `buildDataCheckString`
/// in `@rustigram/tma-server` exactly.
pub(crate) fn build_hmac_check_string(params: &RawParams) -> String {
    let mut entries: Vec<(&str, &str)> = params
        .iter()
        .filter(|(k, _)| k != "hash")
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    entries.sort_by_key(|(a, _)| *a);

    entries
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Builds the Ed25519 data-check string.
///
/// Algorithm: `${bot_id}:WebAppData\n` followed by all params excluding `hash`
/// and `signature`, sorted alphabetically, formatted as `key=value` pairs
/// joined by `\n`. Mirrors `buildEd25519DataCheckString` exactly.
pub(crate) fn build_ed25519_check_string(params: &RawParams, bot_id: i64) -> String {
    let mut entries: Vec<(&str, &str)> = params
        .iter()
        .filter(|(k, _)| k != "hash" && k != "signature")
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    entries.sort_by_key(|(a, _)| *a);

    let fields = entries
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{bot_id}:WebAppData\n{fields}")
}

/// Reads `auth_date` from raw params without building the full struct.
/// Used by validation functions to check expiry before constructing the output.
pub(crate) fn get_auth_date(params: &RawParams) -> Option<i64> {
    params
        .iter()
        .find(|(k, _)| k == "auth_date")
        .and_then(|(_, v)| v.parse().ok())
}

/// Converts raw key-value params into a typed [`WebAppInitData`].
///
/// Sub-object fields (`user`, `chat`, `receiver`) are JSON-encoded strings
/// in the raw query. Integer fields (`auth_date`, `can_send_after`) are
/// decimal strings. Unknown keys are silently ignored for forward compat.
pub(crate) fn build_init_data(params: &RawParams) -> Result<WebAppInitData> {
    let mut data = WebAppInitData {
        query_id: None,
        user: None,
        receiver: None,
        chat: None,
        chat_type: None,
        chat_instance: None,
        start_param: None,
        can_send_after: None,
        auth_date: 0,
        hash: String::new(),
        signature: None,
    };

    let mut has_auth_date = false;
    let mut has_hash = false;

    for (key, value) in params {
        match key.as_str() {
            "query_id" => data.query_id = Some(value.clone()),
            "user" => {
                data.user = Some(serde_json::from_str::<WebAppUser>(value).map_err(|e| {
                    MiniAppError::MalformedInitData(format!("invalid `user` JSON: {e}"))
                })?)
            }
            "receiver" => {
                data.receiver = Some(serde_json::from_str::<WebAppUser>(value).map_err(|e| {
                    MiniAppError::MalformedInitData(format!("invalid `receiver` JSON: {e}"))
                })?)
            }
            "chat" => {
                data.chat = Some(serde_json::from_str::<WebAppChat>(value).map_err(|e| {
                    MiniAppError::MalformedInitData(format!("invalid `chat` JSON: {e}"))
                })?)
            }
            "chat_type" => {
                // Wrap in quotes so serde_json reads it as a JSON string → enum variant.
                let quoted = format!("\"{value}\"");
                data.chat_type = Some(serde_json::from_str::<InitDataChatType>(&quoted).map_err(
                    |e| MiniAppError::MalformedInitData(format!("invalid `chat_type`: {e}")),
                )?)
            }
            "chat_instance" => data.chat_instance = Some(value.clone()),
            "start_param" => data.start_param = Some(value.clone()),
            "can_send_after" => {
                data.can_send_after = Some(value.parse::<i64>().map_err(|_| {
                    MiniAppError::MalformedInitData("`can_send_after` must be an integer".into())
                })?)
            }
            "auth_date" => {
                data.auth_date = value.parse::<i64>().map_err(|_| {
                    MiniAppError::MalformedInitData("`auth_date` must be an integer".into())
                })?;
                has_auth_date = true;
            }
            "hash" => {
                data.hash = value.clone();
                has_hash = true;
            }
            "signature" => data.signature = Some(value.clone()),
            _ => {}
        }
    }

    if !has_auth_date {
        return Err(MiniAppError::MalformedInitData(
            "`auth_date` field is required".into(),
        ));
    }
    if !has_hash {
        return Err(MiniAppError::MalformedInitData(
            "`hash` field is required".into(),
        ));
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_raw_decodes_url_encoding() {
        let raw = "auth_date=1700000000&user=%7B%22id%22%3A42%7D&hash=abc";
        let params = parse_raw(raw);
        let user = params
            .iter()
            .find(|(k, _)| k == "user")
            .map(|(_, v)| v.as_str());
        assert_eq!(user, Some(r#"{"id":42}"#));
    }

    #[test]
    fn hmac_check_string_excludes_hash_and_sorts() {
        let params = vec![
            ("z_field".into(), "last".into()),
            ("auth_date".into(), "1700000000".into()),
            ("hash".into(), "should_be_excluded".into()),
        ];
        let dcs = build_hmac_check_string(&params);
        assert!(!dcs.contains("hash="));
        assert!(dcs.starts_with("auth_date="));
        assert!(dcs.ends_with("z_field=last"));
    }

    #[test]
    fn ed25519_check_string_has_correct_prefix() {
        let params = vec![
            ("auth_date".into(), "1700000000".into()),
            ("signature".into(), "sig_excluded".into()),
            ("hash".into(), "hash_excluded".into()),
        ];
        let dcs = build_ed25519_check_string(&params, 123456789);
        assert!(dcs.starts_with("123456789:WebAppData\n"));
        assert!(!dcs.contains("signature="));
        assert!(!dcs.contains("hash="));
    }

    #[test]
    fn build_init_data_errors_on_missing_auth_date() {
        let params = vec![("hash".into(), "abc".into())];
        assert!(matches!(
            build_init_data(&params),
            Err(MiniAppError::MalformedInitData(_))
        ));
    }

    #[test]
    fn build_init_data_errors_on_missing_hash() {
        let params = vec![("auth_date".into(), "1700000000".into())];
        assert!(matches!(
            build_init_data(&params),
            Err(MiniAppError::MalformedInitData(_))
        ));
    }

    #[test]
    fn build_init_data_ignores_unknown_fields() {
        let params = vec![
            ("auth_date".into(), "1700000000".into()),
            ("hash".into(), "abc".into()),
            ("unknown_future_field".into(), "ignored".into()),
        ];
        assert!(build_init_data(&params).is_ok());
    }
}
