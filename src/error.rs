//! Error types for the krxon application.
//!
//! Uses `thiserror` for domain-level errors.

use thiserror::Error;

/// Domain-level errors for KRX API operations.
#[derive(Debug, Error)]
pub enum KrxError {
    /// Authentication failed — invalid or missing API key.
    #[error("API 인증 실패: API 키를 확인해주세요")]
    Unauthorized,

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

    /// Deserialization / response parsing failed.
    #[error("응답 파싱 실패: {0}")]
    ParseError(String),

    /// HTTP transport error.
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}
