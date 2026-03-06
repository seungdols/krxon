//! HTTP client for the KRX Open API.

/// KRX API HTTP client.
///
/// Handles authentication, request construction, and response parsing
/// for all KRX Open API endpoints.
pub struct KrxClient {
    /// The reqwest HTTP client instance.
    _http: reqwest::Client,
    /// API key for authentication.
    _api_key: String,
    /// Base URL for the KRX API.
    _base_url: String,
}

impl KrxClient {
    /// Creates a new KRX API client.
    pub fn new(api_key: String) -> Self {
        Self {
            _http: reqwest::Client::new(),
            _api_key: api_key,
            _base_url: "https://data-dbg.krx.co.kr/svc/apis".to_string(),
        }
    }
}
