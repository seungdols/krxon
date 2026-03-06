//! Error types for the krxon application.
//!
//! Uses `thiserror` for domain-level errors.

use thiserror::Error;

/// Domain-level errors for KRX API operations.
#[derive(Debug, Error)]
pub enum KrxError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("KRX API error: {0}")]
    ApiError(String),

    /// Invalid date format provided.
    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    /// API key is missing.
    #[error("API key not found. Provide via --key, KRX_API_KEY env var, or ~/.krxon/config.toml")]
    MissingApiKey,

    /// Deserialization failed.
    #[error("Failed to parse response: {0}")]
    ParseError(String),
}
