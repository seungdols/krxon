//! Python SDK code generator.
//!
//! Reads `spec/endpoints.yaml` and produces a Python client library
//! using Tera templates from `templates/python/`.

use std::fs;
use std::path::Path;

use serde::Serialize;
use tera::{Context, Tera};

use super::spec::{load_spec, Spec};
use super::to_pascal_case;

/// Category names in the order they should appear in generated code.
const CATEGORIES: &[&str] = &["index", "stock", "etp", "derivatives"];

/// Category descriptions for generated docstrings.
const CATEGORY_DESCRIPTIONS: &[(&str, &str)] = &[
    ("index", "Index (KRX/KOSPI/KOSDAQ/Derivatives)"),
    ("stock", "Stock (KOSPI/KOSDAQ daily trading and info)"),
    ("etp", "ETP (ETF/ETN)"),
    ("derivatives", "Derivatives (Futures/Options)"),
];

/// Tera context type definition for Python TypedDict generation.
#[derive(Debug, Serialize)]
struct TypeDef {
    /// Python class name (e.g. `KrxIndexDailyRecord`).
    class_name: String,
    /// Endpoint description.
    description: String,
    /// Response fields.
    fields: Vec<TypeField>,
}

/// A single field in a TypedDict class.
#[derive(Debug, Serialize)]
struct TypeField {
    /// Field name (e.g. `BAS_DD`).
    name: String,
    /// Field description in Korean.
    description: String,
}

/// Tera context method info for endpoint mixin generation.
#[derive(Debug, Serialize)]
struct MethodInfo {
    /// Python method name (e.g. `get_krx_index_daily`).
    python_name: String,
    /// API path (e.g. `/idx/krx_dd_trd`).
    path: String,
    /// Endpoint description.
    description: String,
    /// Request parameters.
    params: Vec<ParamInfo>,
    /// Return type name (e.g. `KrxIndexDailyRecord`).
    response_type: String,
}

/// A request parameter for template rendering.
#[derive(Debug, Serialize)]
struct ParamInfo {
    /// Parameter name (e.g. `basDd`).
    name: String,
    /// Parameter description.
    description: String,
}

/// Generates the Python SDK from the spec and writes it to `out_dir`.
///
/// Creates the following structure under `out_dir`:
/// ```text
/// krx/
/// ├── __init__.py
/// ├── client.py
/// ├── types.py
/// └── endpoints/
///     ├── __init__.py
///     ├── index.py
///     ├── stock.py
///     ├── etp.py
///     └── derivatives.py
/// ```
///
/// # Errors
///
/// Returns an error if spec loading, template rendering, or file I/O fails.
pub fn generate(out_dir: &str) -> anyhow::Result<()> {
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("spec/endpoints.yaml");
    let spec = load_spec(&spec_path)?;

    let template_dir = format!("{}/templates/python/**/*.tera", env!("CARGO_MANIFEST_DIR"));
    let tera = Tera::new(&template_dir)?;

    let base = Path::new(out_dir).join("krx");
    let endpoints_dir = base.join("endpoints");
    fs::create_dir_all(&endpoints_dir)?;

    render_init(&tera, &base)?;
    render_types(&tera, &spec, &base)?;
    render_client(&tera, &spec, &base)?;
    render_endpoints_init(&tera, &endpoints_dir)?;

    for category in CATEGORIES {
        if let Some(eps) = spec.endpoints.get(*category) {
            render_endpoint_module(&tera, &spec, category, eps, &endpoints_dir)?;
        }
    }

    eprintln!("Python SDK generated at: {}/krx/", out_dir);
    Ok(())
}

/// Derives a TypedDict class name from a Python method name.
///
/// Strips the `get_` prefix and converts to PascalCase + `Record` suffix.
/// Example: `get_krx_index_daily` → `KrxIndexDailyRecord`.
fn method_name_to_type_name(python_method: &str) -> String {
    let stripped = python_method.strip_prefix("get_").unwrap_or(python_method);
    to_pascal_case(stripped) + "Record"
}

/// Renders `__init__.py`.
fn render_init(tera: &Tera, base: &Path) -> anyhow::Result<()> {
    let ctx = Context::new();
    let rendered = tera.render("__init__.py.tera", &ctx)?;
    fs::write(base.join("__init__.py"), rendered)?;
    Ok(())
}

/// Renders `types.py` with TypedDict definitions for all endpoints.
fn render_types(tera: &Tera, spec: &Spec, base: &Path) -> anyhow::Result<()> {
    let mut type_defs: Vec<TypeDef> = Vec::new();

    for category in CATEGORIES {
        if let Some(eps) = spec.endpoints.get(*category) {
            let mut endpoint_names: Vec<&String> = eps.keys().collect();
            endpoint_names.sort();

            for ep_name in endpoint_names {
                let ep = &eps[ep_name];
                let mapping_key = format!("{}.{}", category, ep_name);
                let python_method = spec
                    .method_mapping
                    .get(&mapping_key)
                    .map(|m| m.python.as_str())
                    .unwrap_or(ep_name);

                let class_name = method_name_to_type_name(python_method);
                let fields: Vec<TypeField> = ep
                    .response_fields
                    .iter()
                    .map(|f| TypeField {
                        name: f.name.clone(),
                        description: f.description.clone(),
                    })
                    .collect();

                type_defs.push(TypeDef {
                    class_name,
                    description: ep.description.clone(),
                    fields,
                });
            }
        }
    }

    let mut ctx = Context::new();
    ctx.insert("type_defs", &type_defs);
    let rendered = tera.render("types.py.tera", &ctx)?;
    fs::write(base.join("types.py"), rendered)?;
    Ok(())
}

/// Renders `client.py` with KrxClient class inheriting category mixins.
fn render_client(tera: &Tera, spec: &Spec, base: &Path) -> anyhow::Result<()> {
    let categories: Vec<&str> = CATEGORIES
        .iter()
        .filter(|c| spec.endpoints.contains_key(**c))
        .copied()
        .collect();

    let mut ctx = Context::new();
    ctx.insert("base_url", &spec.base_url);
    ctx.insert("categories", &categories);
    ctx.insert("notes", &spec.notes);
    let rendered = tera.render("client.py.tera", &ctx)?;
    fs::write(base.join("client.py"), rendered)?;
    Ok(())
}

/// Renders `endpoints/__init__.py`.
fn render_endpoints_init(tera: &Tera, endpoints_dir: &Path) -> anyhow::Result<()> {
    let ctx = Context::new();
    let rendered = tera.render("endpoints/__init__.py.tera", &ctx)?;
    fs::write(endpoints_dir.join("__init__.py"), rendered)?;
    Ok(())
}

/// Renders a single endpoint module (e.g. `endpoints/index.py`).
fn render_endpoint_module(
    tera: &Tera,
    spec: &Spec,
    category: &str,
    endpoints: &std::collections::HashMap<String, super::spec::EndpointDef>,
    endpoints_dir: &Path,
) -> anyhow::Result<()> {
    let mut endpoint_names: Vec<&String> = endpoints.keys().collect();
    endpoint_names.sort();

    let mut methods: Vec<MethodInfo> = Vec::new();

    for ep_name in endpoint_names {
        let ep = &endpoints[ep_name];
        let mapping_key = format!("{}.{}", category, ep_name);
        let python_method = spec
            .method_mapping
            .get(&mapping_key)
            .map(|m| m.python.clone())
            .unwrap_or_else(|| ep_name.clone());

        let response_type = method_name_to_type_name(&python_method);
        let params: Vec<ParamInfo> = ep
            .params
            .iter()
            .map(|p| ParamInfo {
                name: p.name.clone(),
                description: p.description.clone(),
            })
            .collect();

        methods.push(MethodInfo {
            python_name: python_method,
            path: ep.path.clone(),
            description: ep.description.clone(),
            params,
            response_type,
        });
    }

    let category_desc = CATEGORY_DESCRIPTIONS
        .iter()
        .find(|(c, _)| *c == category)
        .map(|(_, d)| *d)
        .unwrap_or(category);

    let mixin_class_name = format!("{}Mixin", to_pascal_case(category));

    let mut ctx = Context::new();
    ctx.insert("category", category);
    ctx.insert("category_description", category_desc);
    ctx.insert("mixin_class_name", &mixin_class_name);
    ctx.insert("methods", &methods);

    let rendered = tera.render("endpoints/category.py.tera", &ctx)?;
    fs::write(endpoints_dir.join(format!("{}.py", category)), rendered)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("krx_index_daily"), "KrxIndexDaily");
        assert_eq!(to_pascal_case("etf_daily"), "EtfDaily");
        assert_eq!(to_pascal_case("kospi_stock_daily"), "KospiStockDaily");
        assert_eq!(to_pascal_case("hello"), "Hello");
    }

    #[test]
    fn test_method_name_to_type_name() {
        assert_eq!(
            method_name_to_type_name("get_krx_index_daily"),
            "KrxIndexDailyRecord"
        );
        assert_eq!(method_name_to_type_name("get_etf_daily"), "EtfDailyRecord");
        assert_eq!(
            method_name_to_type_name("get_kospi_stock_daily"),
            "KospiStockDailyRecord"
        );
    }

    #[test]
    fn test_generate_creates_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_dir = temp_dir.path();

        generate(out_dir.to_str().unwrap()).unwrap();

        assert!(out_dir.join("krx/__init__.py").exists());
        assert!(out_dir.join("krx/client.py").exists());
        assert!(out_dir.join("krx/types.py").exists());
        assert!(out_dir.join("krx/endpoints/__init__.py").exists());
        assert!(out_dir.join("krx/endpoints/index.py").exists());
        assert!(out_dir.join("krx/endpoints/stock.py").exists());
        assert!(out_dir.join("krx/endpoints/etp.py").exists());
        assert!(out_dir.join("krx/endpoints/derivatives.py").exists());

        // Verify AUTO-GENERATED header.
        let client = std::fs::read_to_string(out_dir.join("krx/client.py")).unwrap();
        assert!(client.starts_with("# AUTO-GENERATED"));

        let types = std::fs::read_to_string(out_dir.join("krx/types.py")).unwrap();
        assert!(types.starts_with("# AUTO-GENERATED"));

        let index = std::fs::read_to_string(out_dir.join("krx/endpoints/index.py")).unwrap();
        assert!(index.starts_with("# AUTO-GENERATED"));
        assert!(index.contains("def get_krx_index_daily"));
        // temp_dir is automatically cleaned up on drop.
    }
}
