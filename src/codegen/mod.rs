//! SDK code generation from endpoint specifications.
//!
//! Generates Python and TypeScript client libraries using Tera templates
//! and the endpoint definitions from `spec/endpoints.yaml`.

pub mod python;
pub mod spec;
pub mod typescript;

/// Category names in the order they should appear in generated code.
pub(crate) const CATEGORIES: &[&str] = &["index", "stock", "etp", "derivatives"];

/// Category descriptions for generated doc comments.
pub(crate) const CATEGORY_DESCRIPTIONS: &[(&str, &str)] = &[
    ("index", "Index (KRX/KOSPI/KOSDAQ/Derivatives)"),
    ("stock", "Stock (KOSPI/KOSDAQ daily trading and info)"),
    ("etp", "ETP (ETF/ETN)"),
    ("derivatives", "Derivatives (Futures/Options)"),
];

/// Converts a snake_case string to PascalCase.
///
/// Examples:
/// - `krx_index_daily` → `KrxIndexDaily`
/// - `hello` → `Hello`
pub(crate) fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    upper + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect()
}
