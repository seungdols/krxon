//! Output formatters for fetch results.
//!
//! Converts `Vec<serde_json::Value>` (rows from KRX API) into
//! CSV or human-readable table format.

use serde_json::Value;

/// Formats data rows as CSV with a header row.
///
/// Uses keys from the first row as column headers.
/// Returns an empty string if `data` is empty.
pub fn format_as_csv(data: &[Value]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let headers = extract_headers(data);
    let mut lines = Vec::with_capacity(data.len() + 1);

    // Header line
    lines.push(headers.join(","));

    // Data rows
    for row in data {
        let values: Vec<String> = headers
            .iter()
            .map(|h| csv_escape(row.get(h.as_str()).and_then(|v| v.as_str()).unwrap_or("")))
            .collect();
        lines.push(values.join(","));
    }

    lines.join("\n")
}

/// Formats data rows as a fixed-width text table.
///
/// Auto-calculates column widths for alignment.
/// Returns an empty string if `data` is empty.
pub fn format_as_table(data: &[Value]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let headers = extract_headers(data);

    // Calculate column widths (max of header and all values)
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in data {
        for (i, header) in headers.iter().enumerate() {
            let val = row
                .get(header.as_str())
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if val.len() > widths[i] {
                widths[i] = val.len();
            }
        }
    }

    let mut lines = Vec::with_capacity(data.len() + 3);

    // Header
    let header_line: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h, width = widths[i]))
        .collect();
    lines.push(header_line.join("  "));

    // Separator
    let sep: Vec<String> = widths.iter().map(|&w| "-".repeat(w)).collect();
    lines.push(sep.join("  "));

    // Rows
    for row in data {
        let row_line: Vec<String> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let val = row.get(h.as_str()).and_then(|v| v.as_str()).unwrap_or("");
                format!("{:<width$}", val, width = widths[i])
            })
            .collect();
        lines.push(row_line.join("  "));
    }

    lines.join("\n")
}

/// Extracts ordered column headers from the first data row.
fn extract_headers(data: &[Value]) -> Vec<String> {
    match data.first().and_then(|v| v.as_object()) {
        Some(obj) => obj.keys().cloned().collect(),
        None => Vec::new(),
    }
}

/// Escapes a CSV field value.
///
/// Wraps the value in quotes if it contains a comma, quote, or newline.
fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_format_as_csv_empty() {
        assert_eq!(format_as_csv(&[]), "");
    }

    #[test]
    fn test_format_as_csv_single_row() {
        let data = vec![json!({"BAS_DD": "20250301", "IDX_NM": "KOSPI"})];
        let csv = format_as_csv(&data);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 2);
        // Header should contain both keys
        assert!(lines[0].contains("BAS_DD"));
        assert!(lines[0].contains("IDX_NM"));
        // Data should contain values
        assert!(lines[1].contains("20250301"));
        assert!(lines[1].contains("KOSPI"));
    }

    #[test]
    fn test_format_as_csv_escapes_commas() {
        let data = vec![json!({"value": "1,000"})];
        let csv = format_as_csv(&data);
        assert!(csv.contains("\"1,000\""));
    }

    #[test]
    fn test_format_as_table_empty() {
        assert_eq!(format_as_table(&[]), "");
    }

    #[test]
    fn test_format_as_table_has_header_and_separator() {
        let data = vec![json!({"BAS_DD": "20250301", "IDX_NM": "KOSPI"})];
        let table = format_as_table(&data);
        let lines: Vec<&str> = table.lines().collect();

        assert_eq!(lines.len(), 3); // header + separator + 1 data row
        assert!(lines[1].contains("---")); // separator line
    }

    #[test]
    fn test_format_as_table_column_alignment() {
        let data = vec![
            json!({"NAME": "A", "VALUE": "12345"}),
            json!({"NAME": "BB", "VALUE": "1"}),
        ];
        let table = format_as_table(&data);
        let lines: Vec<&str> = table.lines().collect();

        assert_eq!(lines.len(), 4); // header + separator + 2 rows
    }

    #[test]
    fn test_csv_escape_no_special_chars() {
        assert_eq!(csv_escape("hello"), "hello");
    }

    #[test]
    fn test_csv_escape_with_quotes() {
        assert_eq!(csv_escape("say \"hi\""), "\"say \"\"hi\"\"\"");
    }
}
