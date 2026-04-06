//! Integration-style tests for the API client layer.
//!
//! These tests use `mockito` to intercept HTTP calls so no real bot token is
//! needed. Add `mockito = "1"` to dev-dependencies to run them.

#[cfg(test)]
mod error_handling {
    use rustigram_api::{BotClient, Error};

    #[test]
    fn rejects_token_without_colon() {
        let err = BotClient::from_token("invalidtoken")
            .err()
            .expect("invalid token should return Error::InvalidToken");
        assert!(matches!(err, Error::InvalidToken));
    }

    #[test]
    fn rejects_token_with_non_numeric_id() {
        let err = BotClient::from_token("abc:DEF123")
            .err()
            .expect("invalid token should return Error::InvalidToken");
        assert!(matches!(err, Error::InvalidToken));
    }

    #[test]
    fn rejects_empty_secret_part() {
        let err = BotClient::from_token("123456:")
            .err()
            .expect("invalid token should return Error::InvalidToken");
        assert!(matches!(err, Error::InvalidToken));
    }

    #[test]
    fn accepts_valid_token() {
        let client = BotClient::from_token("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11");
        assert!(client.is_ok());
    }

    #[test]
    fn rate_limit_error_exposes_retry_after() {
        let err = Error::RateLimit { retry_after: 42 };
        assert!(err.is_rate_limit());
        assert_eq!(err.retry_after(), Some(42));
    }

    #[test]
    fn api_429_is_rate_limit() {
        let err = Error::Api {
            error_code: 429,
            description: "Too Many Requests: retry after 5".to_owned(),
            migrate_to_chat_id: None,
            retry_after: Some(5),
        };
        assert!(err.is_rate_limit());
        assert_eq!(err.retry_after(), Some(5));
    }

    #[test]
    fn api_403_blocked_detection() {
        let err = Error::Api {
            error_code: 403,
            description: "Forbidden: bot was blocked by the user".to_owned(),
            migrate_to_chat_id: None,
            retry_after: None,
        };
        assert!(err.is_blocked());
        assert!(!err.is_rate_limit());
    }

    #[test]
    fn api_400_chat_not_found() {
        let err = Error::Api {
            error_code: 400,
            description: "Bad Request: chat not found".to_owned(),
            migrate_to_chat_id: None,
            retry_after: None,
        };
        assert!(err.is_chat_not_found());
    }
}

#[cfg(test)]
mod client_config {
    use rustigram_api::ClientConfig;
    use std::time::Duration;

    #[test]
    fn custom_timeout_is_stored() {
        let cfg = ClientConfig::new("123:abc")
            .unwrap()
            .timeout(Duration::from_secs(60));
        assert_eq!(cfg.timeout, Duration::from_secs(60));
    }

    #[test]
    fn custom_api_base_url() {
        let cfg = ClientConfig::new("123:abc")
            .unwrap()
            .api_base_url("http://localhost:8080");
        assert_eq!(cfg.api_base_url, "http://localhost:8080");
    }

    #[test]
    fn max_retries_is_stored() {
        let cfg = ClientConfig::new("123:abc").unwrap().max_retries(5);
        assert_eq!(cfg.max_retries, 5);
    }
}
