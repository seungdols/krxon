//! HTTP client for the KRX Open API.

use crate::error::KrxError;

/// KRX API HTTP client.
///
/// Handles authentication, request construction, and response parsing
/// for all KRX Open API endpoints.
pub struct KrxClient {
    /// The reqwest HTTP client instance.
    http: reqwest::Client,
    /// API key for authentication.
    api_key: String,
    /// Base URL for the KRX API.
    base_url: String,
}

impl KrxClient {
    /// Creates a new KRX API client with the default base URL.
    pub fn new(api_key: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            base_url: "https://data-dbg.krx.co.kr/svc/apis".to_string(),
        }
    }

    /// Creates a new KRX API client with a custom base URL (for testing).
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            base_url,
        }
    }

    /// Sends a POST request to the given endpoint path and returns the `OutBlock_1` array.
    pub async fn fetch(
        &self,
        path: &str,
        params: serde_json::Value,
    ) -> Result<Vec<serde_json::Value>, KrxError> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http
            .post(&url)
            .header("AUTH_KEY", &self.api_key)
            .json(&params)
            .send()
            .await?;

        let status = response.status();
        let body: serde_json::Value = response.json().await?;

        if !status.is_success() {
            return Err(KrxError::ApiError(format!("HTTP {}: {}", status, body)));
        }

        let records = body
            .get("OutBlock_1")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        Ok(records)
    }
}

/// Resolves the API key from CLI flag or `KRX_API_KEY` environment variable.
///
/// Priority: CLI flag > environment variable.
pub fn resolve_api_key(cli_key: Option<&str>) -> Result<String, KrxError> {
    if let Some(key) = cli_key {
        return Ok(key.to_string());
    }

    if let Ok(key) = std::env::var("KRX_API_KEY") {
        if !key.is_empty() {
            return Ok(key);
        }
    }

    Err(KrxError::MissingApiKey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Mutex to serialize tests that modify the KRX_API_KEY env var.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_resolve_api_key_cli_flag() {
        let key = resolve_api_key(Some("my_key")).unwrap();
        assert_eq!(key, "my_key");
    }

    #[test]
    fn test_resolve_api_key_env_scenarios() {
        let _lock = ENV_MUTEX.lock().unwrap();

        // CLI flag overrides env var.
        std::env::set_var("KRX_API_KEY", "env_key");
        let key = resolve_api_key(Some("cli_key")).unwrap();
        assert_eq!(key, "cli_key");

        // Falls back to env var when no CLI flag.
        let key = resolve_api_key(None).unwrap();
        assert_eq!(key, "env_key");

        // Missing: no CLI flag and no env var.
        std::env::remove_var("KRX_API_KEY");
        let result = resolve_api_key(None);
        assert!(result.is_err());
    }
}
