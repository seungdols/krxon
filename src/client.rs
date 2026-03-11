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
#[derive(Debug)]
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
    ///
    /// # Errors
    ///
    /// - [`KrxError::InvalidApiKey`] if the API key contains invalid HTTP header characters
    /// - [`KrxError::Http`] if the HTTP client fails to initialize
    pub fn new(api_key: &str) -> Result<Self, KrxError> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        default_headers.insert(
            "AUTH_KEY",
            HeaderValue::from_str(api_key).map_err(|_| KrxError::InvalidApiKey)?,
        );

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .default_headers(default_headers)
            .build()?;

        Ok(Self {
            api_key: api_key.to_string(),
            base_url: DEFAULT_BASE_URL.to_string(),
            client,
        })
    }

    /// Creates a client with a custom base URL (for testing).
    #[cfg(test)]
    pub(crate) fn with_base_url(api_key: &str, base_url: &str) -> Result<Self, KrxError> {
        let mut client = Self::new(api_key)?;
        client.base_url = base_url.to_string();
        Ok(client)
    }

    /// Builds the full URL from `base_url` and `path`, normalizing slashes.
    fn build_url(&self, path: &str) -> String {
        if path.is_empty() {
            return self.base_url.clone();
        }

        let base_ends_with_slash = self.base_url.ends_with('/');
        let path_starts_with_slash = path.starts_with('/');

        match (base_ends_with_slash, path_starts_with_slash) {
            (true, true) => {
                format!("{}{}", &self.base_url[..self.base_url.len() - 1], path)
            }
            (false, false) => {
                format!("{}/{}", self.base_url, path)
            }
            _ => {
                format!("{}{}", self.base_url, path)
            }
        }
    }

    /// Sends a POST request to a KRX API endpoint and returns the response data.
    ///
    /// Automatically injects the `AUTH_KEY` header and extracts the `OutBlock_1`
    /// array from the response.
    ///
    /// # Errors
    ///
    /// - [`KrxError::Unauthorized`] if the API key is invalid (HTTP 401)
    /// - [`KrxError::ServiceNotSubscribed`] if the endpoint requires subscription (HTTP 403)
    /// - [`KrxError::RateLimitExceeded`] if the daily call limit is exceeded (HTTP 429)
    /// - [`KrxError::ApiError`] for other unexpected HTTP status codes
    /// - [`KrxError::ParseError`] if `OutBlock_1` is missing or not an array
    pub async fn post(&self, path: &str, body: Value) -> Result<Vec<Value>, KrxError> {
        let url = self.build_url(path);

        let response = self.client.post(&url).json(&body).send().await?;

        let status = response.status();
        if !status.is_success() {
            return match status.as_u16() {
                401 => Err(KrxError::Unauthorized),
                403 => Err(KrxError::ServiceNotSubscribed {
                    service: path.to_string(),
                }),
                429 => Err(KrxError::RateLimitExceeded),
                _ => {
                    let body_text = match response.text().await {
                        Ok(text) => text,
                        Err(e) => format!("<failed to read body: {}>", e),
                    };
                    Err(KrxError::ApiError(format!(
                        "HTTP {}: {}",
                        status.as_u16(),
                        body_text
                    )))
                }
            };
        }

        let json: Value = response.json().await?;

        let out_block = json
            .get("OutBlock_1")
            .ok_or_else(|| KrxError::ParseError("Response missing OutBlock_1 field".to_string()))?;

        match out_block {
            Value::Array(arr) => Ok(arr.clone()),
            _ => Err(KrxError::ParseError(
                "OutBlock_1 is not an array".to_string(),
            )),
        }
    }
}

/// Resolves the API key using the priority chain:
///
/// 1. Explicit key (from CLI `--key` flag)
/// 2. `KRX_API_KEY` environment variable
/// 3. `~/.krxon/config.json` file (`{ "api_key": "..." }`)
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

    // 3. Config file (~/.krxon/config.json)
    if let Some(key) = load_api_key_from_config() {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    Err(KrxError::MissingApiKey)
}

/// Loads the API key from `~/.krxon/config.json`.
///
/// Expected format: `{ "api_key": "your_api_key" }`
fn load_api_key_from_config() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let config_path = std::path::Path::new(&home).join(".krxon").join("config.json");
    let content = std::fs::read_to_string(config_path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    json.get("api_key")?.as_str().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    // -- EnvVarGuard: RAII pattern for safe env var mutation in tests --

    static ENV_MUTEX: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        ENV_MUTEX
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .expect("failed to lock ENV_MUTEX")
    }

    struct EnvVarGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let original = std::env::var(key).ok();
            // SAFETY: Protected by ENV_MUTEX — only one test mutates env at a time.
            unsafe { std::env::set_var(key, value) };
            Self { key, original }
        }

        fn unset(key: &'static str) -> Self {
            let original = std::env::var(key).ok();
            // SAFETY: Protected by ENV_MUTEX — only one test mutates env at a time.
            unsafe { std::env::remove_var(key) };
            Self { key, original }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            // SAFETY: Protected by ENV_MUTEX — only one test mutates env at a time.
            match &self.original {
                Some(val) => unsafe { std::env::set_var(self.key, val) },
                None => unsafe { std::env::remove_var(self.key) },
            }
        }
    }

    // -- helpers --

    fn test_client(server: &Server, api_key: &str) -> KrxClient {
        KrxClient::with_base_url(api_key, &server.url()).expect("failed to create test client")
    }

    // -- KrxClient::new tests --

    #[test]
    fn test_new_returns_error_for_invalid_api_key() {
        // Header values cannot contain \n
        let result = KrxClient::new("bad\nkey");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KrxError::InvalidApiKey));
    }

    #[test]
    fn test_new_succeeds_with_valid_api_key() {
        let result = KrxClient::new("valid_key_123");
        assert!(result.is_ok());
    }

    // -- build_url tests --

    #[test]
    fn test_build_url_normal() {
        let client = KrxClient::new("key").unwrap();
        assert_eq!(
            client.build_url("/idx/krx_dd_trd"),
            "https://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd"
        );
    }

    #[test]
    fn test_build_url_no_leading_slash() {
        let client = KrxClient::new("key").unwrap();
        assert_eq!(
            client.build_url("idx/krx_dd_trd"),
            "https://data-dbg.krx.co.kr/svc/apis/idx/krx_dd_trd"
        );
    }

    #[test]
    fn test_build_url_empty_path() {
        let client = KrxClient::new("key").unwrap();
        assert_eq!(client.build_url(""), "https://data-dbg.krx.co.kr/svc/apis");
    }

    // -- post() tests --

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
        assert!(matches!(result.unwrap_err(), KrxError::Unauthorized));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_403_returns_service_not_subscribed() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/some/endpoint")
            .with_status(403)
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/some/endpoint", json!({})).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            KrxError::ServiceNotSubscribed { .. }
        ));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_post_429_returns_rate_limit_exceeded() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .with_status(429)
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = client.post("/idx/krx_dd_trd", json!({})).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KrxError::RateLimitExceeded));
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
        assert!(matches!(result.unwrap_err(), KrxError::ParseError(_)));
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
        assert!(matches!(result.unwrap_err(), KrxError::ApiError(_)));
        mock.assert_async().await;
    }

    // -- resolve_api_key tests (with EnvVarGuard) --

    #[test]
    fn test_resolve_api_key_cli_takes_priority() {
        let _lock = env_lock();
        let _guard = EnvVarGuard::set("KRX_API_KEY", "env_key");

        let result = resolve_api_key(Some("cli_key"));
        assert_eq!(result.unwrap(), "cli_key");
    }

    #[test]
    fn test_resolve_api_key_env_fallback() {
        let _lock = env_lock();
        let _guard = EnvVarGuard::set("KRX_API_KEY", "env_key");

        let result = resolve_api_key(None);
        assert_eq!(result.unwrap(), "env_key");
    }

    #[test]
    fn test_resolve_api_key_missing() {
        let _lock = env_lock();
        let _guard = EnvVarGuard::unset("KRX_API_KEY");

        let result = resolve_api_key(None);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KrxError::MissingApiKey));
    }
}
