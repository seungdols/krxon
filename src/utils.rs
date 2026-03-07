//! Utility functions for date validation and formatting.

use crate::error::KrxError;

/// Validates that a date string is in YYYYMMDD format (exactly 8 ASCII digits).
///
/// # Errors
///
/// Returns [`KrxError::InvalidDate`] if the format is invalid.
pub fn validate_date(date: &str) -> Result<(), KrxError> {
    if date.len() != 8 || !date.chars().all(|c| c.is_ascii_digit()) {
        return Err(KrxError::InvalidDate(date.to_string()));
    }
    Ok(())
}

/// Returns today's date as a `YYYYMMDD` string in the local timezone.
pub fn today() -> String {
    chrono::Local::now().format("%Y%m%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_date_valid() {
        assert!(validate_date("20250301").is_ok());
        assert!(validate_date("20000101").is_ok());
        assert!(validate_date("99991231").is_ok());
    }

    #[test]
    fn test_validate_date_invalid_format() {
        // Contains hyphens
        let err = validate_date("2025-03-01").unwrap_err();
        assert!(matches!(err, KrxError::InvalidDate(_)));

        // Too short
        assert!(validate_date("2025031").is_err());

        // Too long
        assert!(validate_date("202503011").is_err());

        // Non-digit characters
        assert!(validate_date("abcdefgh").is_err());

        // Empty string
        assert!(validate_date("").is_err());
    }

    #[test]
    fn test_today_returns_valid_format() {
        let result = today();
        assert!(validate_date(&result).is_ok());
    }
}
