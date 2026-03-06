//! Utility functions for date validation and formatting.

/// Validates that a date string is in YYYYMMDD format.
///
/// Returns `true` if the string is exactly 8 digits.
pub fn is_valid_date_format(date: &str) -> bool {
    if date.len() != 8 {
        return false;
    }
    date.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        assert!(is_valid_date_format("20250301"));
    }

    #[test]
    fn test_invalid_date() {
        assert!(!is_valid_date_format("2025-03-01"));
        assert!(!is_valid_date_format("abcdefgh"));
        assert!(!is_valid_date_format("2025031"));
    }
}
