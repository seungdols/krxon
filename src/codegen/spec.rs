//! Spec parser for `spec/endpoints.yaml`.
//!
//! Deserializes the YAML endpoint specification into strongly-typed
//! Rust structs for use by code generators.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Root of the spec file.
#[derive(Debug, Deserialize)]
pub struct Spec {
    /// Spec version.
    pub version: String,
    /// KRX API base URL.
    pub base_url: String,
    /// Global notes and metadata.
    pub notes: SpecNotes,
    /// Python/TypeScript method name mappings keyed by `"category.endpoint_name"`.
    pub method_mapping: HashMap<String, MethodNames>,
    /// Endpoint definitions grouped by category (index, stock, etp, derivatives).
    pub endpoints: HashMap<String, HashMap<String, EndpointDef>>,
}

/// Global notes / metadata from the spec.
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecNotes {
    /// HTTP method (always POST).
    pub http_method: String,
    /// Content type header value.
    pub content_type: String,
    /// Authentication header name.
    pub auth_header: String,
    /// Authentication description.
    pub auth_description: String,
    /// Response root key name.
    pub response_root_key: String,
    /// Response root key description.
    pub response_root_description: String,
    /// Date format string.
    pub date_format: String,
    /// Date format description.
    pub date_description: String,
    /// Field types description.
    pub field_types: String,
    /// Daily rate limit.
    pub rate_limit: u32,
    /// Rate limit description.
    pub rate_limit_description: String,
    /// Data update time description.
    pub data_update_time: String,
    /// ISIN omission warning.
    pub isin_omit_warning: String,
    /// Subscription notice.
    pub subscription_notice: String,
}

/// Python and TypeScript method name pair.
#[derive(Debug, Deserialize)]
pub struct MethodNames {
    /// Python method name (e.g. `get_krx_index_daily`).
    pub python: String,
    /// TypeScript method name (e.g. `getKrxIndexDaily`).
    pub typescript: String,
}

/// A single endpoint definition.
#[derive(Debug, Deserialize)]
pub struct EndpointDef {
    /// API path (e.g. `/idx/krx_dd_trd`).
    pub path: String,
    /// Endpoint description in Korean.
    pub description: String,
    /// Request parameters.
    pub params: Vec<ParamDef>,
    /// Response fields.
    pub response_fields: Vec<ResponseFieldDef>,
}

/// A request parameter definition.
#[derive(Debug, Deserialize)]
pub struct ParamDef {
    /// Parameter name (e.g. `basDd`).
    pub name: String,
    /// Parameter type (always `string`).
    #[serde(rename = "type")]
    pub field_type: String,
    /// Whether the parameter is required.
    #[serde(default)]
    pub required: bool,
    /// Parameter description.
    pub description: String,
}

/// A response field definition.
#[derive(Debug, Deserialize)]
pub struct ResponseFieldDef {
    /// Field name as returned by KRX API (e.g. `BAS_DD`).
    pub name: String,
    /// Field type (always `string`).
    #[serde(rename = "type")]
    pub field_type: String,
    /// Field description in Korean.
    pub description: String,
}

/// Loads and parses the spec YAML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or if YAML deserialization fails.
pub fn load_spec(path: &Path) -> anyhow::Result<Spec> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read spec file '{}': {}", path.display(), e))?;
    let spec: Spec = serde_yaml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse spec file '{}': {}", path.display(), e))?;
    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec_path() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("spec/endpoints.yaml")
    }

    #[test]
    fn test_load_spec_parses_all_categories() {
        let spec = load_spec(&spec_path()).unwrap();
        assert_eq!(spec.endpoints.len(), 4);
        assert!(spec.endpoints.contains_key("index"));
        assert!(spec.endpoints.contains_key("stock"));
        assert!(spec.endpoints.contains_key("etp"));
        assert!(spec.endpoints.contains_key("derivatives"));
    }

    #[test]
    fn test_load_spec_method_mapping_count() {
        let spec = load_spec(&spec_path()).unwrap();
        assert_eq!(spec.method_mapping.len(), 16);
    }

    #[test]
    fn test_load_spec_endpoint_details() {
        let spec = load_spec(&spec_path()).unwrap();
        let index = spec.endpoints.get("index").unwrap();
        let krx = index.get("krx_daily").unwrap();
        assert_eq!(krx.path, "/idx/krx_dd_trd");
        assert_eq!(krx.params.len(), 1);
        assert_eq!(krx.params[0].name, "basDd");
        assert!(krx.params[0].required);
    }

    #[test]
    fn test_load_spec_base_url() {
        let spec = load_spec(&spec_path()).unwrap();
        assert_eq!(spec.base_url, "https://data-dbg.krx.co.kr/svc/apis");
    }

    #[test]
    fn test_load_spec_response_fields() {
        let spec = load_spec(&spec_path()).unwrap();
        let index = spec.endpoints.get("index").unwrap();
        let krx = index.get("krx_daily").unwrap();
        assert!(!krx.response_fields.is_empty());
        assert_eq!(krx.response_fields[0].name, "BAS_DD");
    }

    #[test]
    fn test_load_spec_python_method_names() {
        let spec = load_spec(&spec_path()).unwrap();
        let mapping = spec.method_mapping.get("index.krx_daily").unwrap();
        assert_eq!(mapping.python, "get_krx_index_daily");
    }
}
