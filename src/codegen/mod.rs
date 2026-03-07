//! SDK code generation from endpoint specifications.
//!
//! Generates Python and TypeScript client libraries using Tera templates
//! and the endpoint definitions from `spec/endpoints.yaml`.

pub mod python;
pub mod spec;
pub mod typescript;

/// Converts a snake_case or camelCase string to PascalCase.
///
/// Examples:
/// - `krx_index_daily` → `KrxIndexDaily`
/// - `getKrxIndexDaily` → `GetKrxIndexDaily`
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
