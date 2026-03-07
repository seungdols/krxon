//! Output formatters for fetch results.
//!
//! Provides CSV and `comfy-table` based table formatting for KRX API data.

use comfy_table::{ContentArrangement, Table};
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
    lines.push(
        headers
            .iter()
            .map(|h| csv_escape(h))
            .collect::<Vec<_>>()
            .join(","),
    );

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

/// Formats data rows as a `comfy-table` text table.
///
/// Auto-calculates column widths for alignment.
/// Returns an empty string if `data` is empty.
pub fn format_as_table(data: &[Value]) -> String {
    if data.is_empty() {
        return String::new();
    }

    let headers = extract_headers(data);

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(&headers);

    for row in data {
        let cells: Vec<String> = headers
            .iter()
            .map(|h| {
                row.get(h.as_str())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        table.add_row(cells);
    }

    table.to_string()
}

/// Builds a `comfy-table` from explicit headers and typed record rows.
///
/// Each record is converted to a row via the `to_row` closure.
/// Returns the formatted table string, or `"No data found."` if empty.
pub fn format_records_table<T, F>(headers: &[&str], records: &[T], to_row: F) -> String
where
    F: Fn(&T) -> Vec<String>,
{
    if records.is_empty() {
        return "No data found.".to_string();
    }

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(headers);

    for record in records {
        table.add_row(to_row(record));
    }

    table.to_string()
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
        assert!(lines[0].contains("BAS_DD"));
        assert!(lines[0].contains("IDX_NM"));
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
    fn test_format_as_table_has_header_and_data() {
        let data = vec![json!({"BAS_DD": "20250301", "IDX_NM": "KOSPI"})];
        let table = format_as_table(&data);

        assert!(table.contains("BAS_DD"));
        assert!(table.contains("IDX_NM"));
        assert!(table.contains("20250301"));
        assert!(table.contains("KOSPI"));
    }

    #[test]
    fn test_format_as_table_multiple_rows() {
        let data = vec![
            json!({"NAME": "A", "VALUE": "12345"}),
            json!({"NAME": "BB", "VALUE": "1"}),
        ];
        let table = format_as_table(&data);

        assert!(table.contains("A"));
        assert!(table.contains("BB"));
        assert!(table.contains("12345"));
    }

    #[test]
    fn test_format_records_table_empty() {
        let result = format_records_table::<String, _>(&["A", "B"], &[], |_| vec![]);
        assert_eq!(result, "No data found.");
    }

    #[test]
    fn test_format_records_table_with_data() {
        let data = vec![("Samsung", "80000"), ("LG", "50000")];
        let result = format_records_table(&["Name", "Price"], &data, |r| {
            vec![r.0.to_string(), r.1.to_string()]
        });

        assert!(result.contains("Name"));
        assert!(result.contains("Price"));
        assert!(result.contains("Samsung"));
        assert!(result.contains("80000"));
        assert!(result.contains("LG"));
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
