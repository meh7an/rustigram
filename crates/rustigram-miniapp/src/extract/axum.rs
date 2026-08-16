//! [`TmaInitData`] Axum extractor implementation.

use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    error::{MiniAppError, Result},
    parse::parse_raw,
    types::WebAppInitData,
    validate::{validate_ed25519, validate_hmac, Ed25519ValidateOpts, HmacValidateOpts},
};

use super::BotToken;

/// Axum extractor that validates `initData` from the incoming request and
/// provides a typed [`WebAppInitData`] to the handler.
///
/// Reads the raw initData string from — in priority order:
/// 1. `X-Tma-Init-Data` header
/// 2. `Authorization: tma <data>` header
///
/// Validation method is auto-detected: if the raw string contains a
/// `signature` field, Ed25519 is used; otherwise HMAC-SHA256.
///
/// Requires [`BotTokenLayer`] on the router. Returns `401` on auth failure,
/// `400` on malformed input, `500` if the layer is missing.
///
/// # Example
///
/// ```rust,ignore
/// async fn handler(TmaInitData(data): TmaInitData) {
///     println!("user: {:?}", data.user);
/// }
/// ```
pub struct TmaInitData(pub WebAppInitData);

impl<S> FromRequestParts<S> for TmaInitData
where
    S: Send + Sync,
{
    type Rejection = MiniAppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let raw = read_init_data_header(parts)?;

        let token = parts
            .extensions
            .get::<BotToken>()
            .ok_or(MiniAppError::MissingBotToken)?;

        // Auto-detect validation method: Ed25519 when `signature` is present.
        let params = parse_raw(&raw);
        let has_signature = params.iter().any(|(k, _)| k == "signature");

        let init_data = if has_signature {
            let bot_id = extract_bot_id(&token.0)?;
            validate_ed25519(&raw, bot_id, Ed25519ValidateOpts::default())?
        } else {
            validate_hmac(&raw, &token.0, HmacValidateOpts::default())?
        };

        Ok(TmaInitData(init_data))
    }
}

/// Reads the raw initData string, or reports which headers were expected.
fn read_init_data_header(parts: &Parts) -> Result<String> {
    super::init_data_from_headers(&parts.headers).ok_or_else(|| {
        MiniAppError::MalformedInitData(
            "missing X-Tma-Init-Data header (or Authorization: tma <data>)".into(),
        )
    })
}

/// Extracts the numeric bot ID from the bot token (`123456789:ABC...` → `123456789`).
fn extract_bot_id(token: &str) -> Result<i64> {
    token
        .split(':')
        .next()
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| MiniAppError::MalformedInitData("cannot extract bot ID from token".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use form_urlencoded;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use tower::ServiceExt;

    use crate::extract::{layer::BotTokenLayer, BotToken};

    type HmacSha256 = Hmac<Sha256>;

    const BOT_TOKEN: &str = "123456789:test-bot-token-for-unit-tests";

    async fn handler(_: TmaInitData) -> &'static str {
        "ok"
    }

    fn app_with_layer() -> Router {
        Router::new()
            .route("/tma", get(handler))
            .layer(BotTokenLayer(BotToken(BOT_TOKEN.to_owned())))
    }

    fn build_valid_init_data(token: &str) -> String {
        let auth_date = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let user_json = r#"{"id":42,"first_name":"Mehran"}"#;

        let mut entries = [
            ("auth_date", auth_date.clone()),
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
            .append_pair("auth_date", &auth_date)
            .append_pair("query_id", "test_query_001")
            .append_pair("user", user_json)
            .append_pair("hash", &hash)
            .finish()
    }

    #[tokio::test]
    async fn valid_header_returns_200() {
        let raw = build_valid_init_data(BOT_TOKEN);
        let req = Request::builder()
            .uri("/tma")
            .header("x-tma-init-data", raw)
            .body(Body::empty())
            .unwrap();

        let status = app_with_layer().oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn authorization_bearer_fallback_works() {
        let raw = build_valid_init_data(BOT_TOKEN);
        let req = Request::builder()
            .uri("/tma")
            .header("authorization", format!("tma {raw}"))
            .body(Body::empty())
            .unwrap();

        let status = app_with_layer().oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn invalid_hash_returns_401() {
        let req = Request::builder()
            .uri("/tma")
            .header("x-tma-init-data", "auth_date=1234&hash=badhash")
            .body(Body::empty())
            .unwrap();

        let status = app_with_layer().oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn missing_header_returns_400() {
        let req = Request::builder().uri("/tma").body(Body::empty()).unwrap();

        let status = app_with_layer().oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn missing_bot_token_layer_returns_500() {
        let app = Router::new().route("/tma", get(handler));
        let req = Request::builder()
            .uri("/tma")
            .header("x-tma-init-data", "auth_date=1234&hash=abc")
            .body(Body::empty())
            .unwrap();

        let status = app.oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
