//! HTTP client for the KRX Open API.

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;

use crate::error::KrxError;

const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_BASE_URL: &str = "https://data-dbg.krx.co.kr/svc/apis";

/// KRX API HTTP client.
///
/// Handles authentication, request construction, and response parsing
/// for all KRX Open API endpoints.
pub struct KrxClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

impl KrxClient {
    /// Creates a new KRX API client with the given API key.
    ///
    /// The client is preconfigured with a 30-second timeout and
    /// default headers (`AUTH_KEY`, `Content-Type: application/json`).
    pub fn new(api_key: &str) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        default_headers.insert(
            "AUTH_KEY",
            HeaderValue::from_str(api_key).expect("API key contains invalid header characters"),
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .default_headers(default_headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key: api_key.to_string(),
            base_url: DEFAULT_BASE_URL.to_string(),
            client,
        }
    }

    /// Creates a client with a custom base URL (for testing).
    #[cfg(test)]
    fn with_base_url(api_key: &str, base_url: &str) -> Self {
        let mut client = Self::new(api_key);
        client.base_url = base_url.to_string();
        client
    }

    /// Sends a POST request to a KRX API endpoint and returns the response data.
    ///
    /// Automatically injects the `AUTH_KEY` header and extracts the `OutBlock_1`
    /// array from the response.
    ///
    /// # Errors
    ///
    /// - [`KrxError::Unauthorized`] if the API key is invalid (HTTP 401)
    /// - [`KrxError::Forbidden`] if the endpoint requires subscription (HTTP 403)
    /// - [`KrxError::RateLimited`] if the daily call limit is exceeded (HTTP 429)
    /// - [`KrxError::ApiError`] for other HTTP failures
    /// - [`KrxError::ParseError`] if `OutBlock_1` is missing or not an array
    pub async fn post(&self, path: &str, body: Value) -> anyhow::Result<Vec<Value>> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(KrxError::from)?;

        let status = response.status();
        if !status.is_success() {
            return match status.as_u16() {
                401 => Err(KrxError::Unauthorized.into()),
                403 => Err(KrxError::Forbidden.into()),
                429 => Err(KrxError::RateLimited.into()),
                _ => {
                    let body_text = response.text().await.unwrap_or_default();
                    Err(
                        KrxError::ApiError(format!("HTTP {}: {}", status.as_u16(), body_text))
                            .into(),
                    )
                }
            };
        }

        let json: Value = response.json().await.map_err(KrxError::from)?;

        let out_block = json
            .get("OutBlock_1")
            .ok_or_else(|| KrxError::ParseError("Response missing OutBlock_1 field".to_string()))?;

        match out_block {
            Value::Array(arr) => Ok(arr.clone()),
            _ => Err(KrxError::ParseError("OutBlock_1 is not an array".to_string()).into()),
        }
    }
}

/// Resolves the API key using the priority chain:
///
/// 1. Explicit key (from CLI `--key` flag)
/// 2. `KRX_API_KEY` environment variable
/// 3. `~/.krxon/config.toml` (not yet implemented)
///
/// Returns [`KrxError::MissingApiKey`] if no key is found.
pub fn resolve_api_key(cli_key: Option<&str>) -> Result<String, KrxError> {
    // 1. CLI flag
    if let Some(key) = cli_key {
        if !key.is_empty() {
            return Ok(key.to_string());
        }
    }

    // 2. Environment variable
    if let Ok(key) = std::env::var("KRX_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    // TODO: 3. ~/.krxon/config.toml (requires toml crate dependency)

    Err(KrxError::MissingApiKey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    fn test_client(server: &Server, api_key: &str) -> KrxClient {
        KrxClient::with_base_url(api_key, &server.url())
    }

    #[tokio::test]
    async fn test_post_success_extracts_outblock() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .match_header("AUTH_KEY", "test_key")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "OutBlock_1": [
                        {"BAS_DD": "20250301", "IDX_NM": "KOSPI"}
                    ]
                })
                .to_string(),
            )
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client
            .post("/idx/krx_dd_trd", json!({"basDd": "20250301"}))
            .await;

        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0]["IDX_NM"], "KOSPI");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_401_returns_unauthorized() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .with_status(401)
            .create_async()
            .await;

        let client = test_client(&server, "bad_key");
        let result = client
            .post("/idx/krx_dd_trd", json!({"basDd": "20250301"}))
            .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<KrxError>()
            .is_some_and(|e| matches!(e, KrxError::Unauthorized)));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_403_returns_forbidden() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/some/endpoint")
            .with_status(403)
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/some/endpoint", json!({})).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<KrxError>()
            .is_some_and(|e| matches!(e, KrxError::Forbidden)));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_429_returns_rate_limited() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .with_status(429)
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/idx/krx_dd_trd", json!({})).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<KrxError>()
            .is_some_and(|e| matches!(e, KrxError::RateLimited)));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_missing_outblock_returns_parse_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"SomeOtherKey": []}).to_string())
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/idx/krx_dd_trd", json!({})).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<KrxError>()
            .is_some_and(|e| matches!(e, KrxError::ParseError(_))));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_empty_outblock_returns_empty_vec() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"OutBlock_1": []}).to_string())
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/idx/krx_dd_trd", json!({})).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_auth_key_header_is_sent() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .match_header("AUTH_KEY", "my_secret_key_123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"OutBlock_1": []}).to_string())
            .create_async()
            .await;

        let client = test_client(&server, "my_secret_key_123");
        let _ = client.post("/idx/krx_dd_trd", json!({})).await;

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_500_returns_api_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/idx/krx_dd_trd", json!({})).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<KrxError>()
            .is_some_and(|e| matches!(e, KrxError::ApiError(_))));
        mock.assert_async().await;
    }

    #[test]
    fn test_resolve_api_key_cli_takes_priority() {
        std::env::set_var("KRX_API_KEY", "env_key");
        let result = resolve_api_key(Some("cli_key"));
        assert_eq!(result.unwrap(), "cli_key");
        std::env::remove_var("KRX_API_KEY");
    }

    #[test]
    fn test_resolve_api_key_env_fallback() {
        std::env::set_var("KRX_API_KEY", "env_key");
        let result = resolve_api_key(None);
        assert_eq!(result.unwrap(), "env_key");
        std::env::remove_var("KRX_API_KEY");
    }

    #[test]
    fn test_resolve_api_key_missing() {
        std::env::remove_var("KRX_API_KEY");
        let result = resolve_api_key(None);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KrxError::MissingApiKey));
    }
}
