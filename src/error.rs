//! Error types for the krxon application.
//!
//! Uses `thiserror` for domain-level errors.

use thiserror::Error;

/// Domain-level errors for KRX API operations.
#[derive(Debug, Error)]
pub enum KrxError {
    /// Authentication failed (HTTP 401) — the server rejected the API key.
    #[error("API 인증 실패 (HTTP 401): API 키가 유효하지 않거나 만료되었습니다")]
    Unauthorized,

    /// API key is missing — not provided via CLI, env var, or config file.
    #[error("API 키를 찾을 수 없습니다. --key 플래그, KRX_API_KEY 환경 변수, 또는 ~/.krxon/config.json 을 확인하세요")]
    MissingApiKey,

    /// API key contains invalid characters for HTTP header.
    #[error("API 키 형식 오류: HTTP 헤더에 허용되지 않는 문자가 포함되어 있습니다")]
    InvalidApiKey,

    /// The endpoint requires a service subscription.
    #[error("서비스 이용 신청이 필요합니다: {service}")]
    ServiceNotSubscribed { service: String },

    /// Daily API call limit exceeded.
    #[error("호출 한도 초과 (일 10,000회)")]
    RateLimitExceeded,

    /// Invalid date format provided.
    #[error("유효하지 않은 날짜 형식: {0} (YYYYMMDD 필요)")]
    InvalidDate(String),

    /// No data returned (possibly a market holiday).
    #[error("데이터 없음 (휴장일 가능성): {0}")]
    NoData(String),

    /// API returned an unexpected HTTP error.
    #[error("KRX API 오류: {0}")]
    ApiError(String),

    /// Deserialization / response parsing failed.
    #[error("응답 파싱 실패: {0}")]
    ParseError(String),

    /// HTTP transport error.
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}
