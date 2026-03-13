//! TypeScript SDK code generator.
//!
//! Reads `spec/endpoints.yaml` and produces a TypeScript client library
//! using Tera templates from `templates/typescript/`.

use std::fs;
use std::path::Path;

use serde::Serialize;
use tera::{Context, Tera};

use super::spec::{load_spec, Spec};
use super::{to_pascal_case, CATEGORIES, CATEGORY_DESCRIPTIONS};

/// Tera context type definition for TypeScript interface generation.
#[derive(Debug, Serialize)]
struct TypeDef {
    /// TypeScript interface name (e.g. `KrxIndexDailyRecord`).
    class_name: String,
    /// Endpoint description.
    description: String,
    /// Response fields.
    fields: Vec<TypeField>,
}

/// A single field in a TypeScript interface.
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
    /// TypeScript method name (e.g. `getKrxIndexDaily`).
    ts_name: String,
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

/// Generates the TypeScript SDK from the spec and writes it to `out_dir`.
///
/// Creates the following structure under `out_dir`:
/// ```text
/// krx/
/// ├── package.json
/// ├── README.md
/// ├── tsconfig.json
/// └── src/
///     ├── index.ts
///     ├── client.ts
///     ├── types.ts
///     └── endpoints/
///         ├── index.ts
///         ├── stock.ts
///         ├── etp.ts
///         └── derivatives.ts
/// ```
///
/// # Errors
///
/// Returns an error if spec loading, template rendering, or file I/O fails.
pub fn generate(out_dir: &str) -> anyhow::Result<()> {
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("spec/endpoints.yaml");
    let spec = load_spec(&spec_path)?;

    let template_dir = format!(
        "{}/templates/typescript/**/*.tera",
        env!("CARGO_MANIFEST_DIR")
    );
    let tera = Tera::new(&template_dir)?;

    let base = Path::new(out_dir).join("krx");
    let src_dir = base.join("src");
    let endpoints_dir = src_dir.join("endpoints");
    fs::create_dir_all(&endpoints_dir)?;

    render_package_json(&tera, &base)?;
    render_readme(&tera, &base)?;
    render_tsconfig(&tera, &base)?;
    render_index(&tera, &src_dir)?;
    render_types(&tera, &spec, &src_dir)?;
    render_client(&tera, &spec, &src_dir)?;

    for category in CATEGORIES {
        if let Some(eps) = spec.endpoints.get(*category) {
            render_endpoint_module(&tera, &spec, category, eps, &endpoints_dir)?;
        }
    }

    eprintln!("TypeScript SDK generated at: {}/krx/", out_dir);
    Ok(())
}

/// Derives a TypeScript interface name from a TypeScript method name.
///
/// Strips the `get` prefix (camelCase) and converts to PascalCase + `Record` suffix.
/// Example: `getKrxIndexDaily` → `KrxIndexDailyRecord`.
fn method_name_to_type_name(ts_method: &str) -> String {
    let stripped = ts_method.strip_prefix("get").unwrap_or(ts_method);
    // The remainder is already PascalCase after stripping `get`.
    format!("{}Record", stripped)
}

/// Renders `package.json`.
fn render_package_json(tera: &Tera, base: &Path) -> anyhow::Result<()> {
    let ctx = Context::new();
    let rendered = tera.render("package.json.tera", &ctx)?;
    fs::write(base.join("package.json"), rendered)?;
    Ok(())
}

/// Renders `README.md`.
fn render_readme(tera: &Tera, base: &Path) -> anyhow::Result<()> {
    let ctx = Context::new();
    let rendered = tera.render("README.md.tera", &ctx)?;
    fs::write(base.join("README.md"), rendered)?;
    Ok(())
}

/// Renders `tsconfig.json`.
fn render_tsconfig(tera: &Tera, base: &Path) -> anyhow::Result<()> {
    let ctx = Context::new();
    let rendered = tera.render("tsconfig.json.tera", &ctx)?;
    fs::write(base.join("tsconfig.json"), rendered)?;
    Ok(())
}

/// Renders `src/index.ts`.
fn render_index(tera: &Tera, src_dir: &Path) -> anyhow::Result<()> {
    let ctx = Context::new();
    let rendered = tera.render("src/index.ts.tera", &ctx)?;
    fs::write(src_dir.join("index.ts"), rendered)?;
    Ok(())
}

/// Renders `src/types.ts` with TypeScript interface definitions for all endpoints.
fn render_types(tera: &Tera, spec: &Spec, src_dir: &Path) -> anyhow::Result<()> {
    let mut type_defs: Vec<TypeDef> = Vec::new();

    for category in CATEGORIES {
        if let Some(eps) = spec.endpoints.get(*category) {
            let mut endpoint_names: Vec<&String> = eps.keys().collect();
            endpoint_names.sort();

            for ep_name in endpoint_names {
                let ep = &eps[ep_name];
                let mapping_key = format!("{}.{}", category, ep_name);
                let ts_method = spec
                    .method_mapping
                    .get(&mapping_key)
                    .map(|m| m.typescript.as_str())
                    .unwrap_or(ep_name);

                let class_name = method_name_to_type_name(ts_method);
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
    let rendered = tera.render("src/types.ts.tera", &ctx)?;
    fs::write(src_dir.join("types.ts"), rendered)?;
    Ok(())
}

/// Renders `src/client.ts` with KrxClient class using category mixins.
fn render_client(tera: &Tera, spec: &Spec, src_dir: &Path) -> anyhow::Result<()> {
    let categories: Vec<&str> = CATEGORIES
        .iter()
        .filter(|c| spec.endpoints.contains_key(**c))
        .copied()
        .collect();

    let mut ctx = Context::new();
    ctx.insert("base_url", &spec.base_url);
    ctx.insert("categories", &categories);
    ctx.insert("notes", &spec.notes);
    let rendered = tera.render("src/client.ts.tera", &ctx)?;
    fs::write(src_dir.join("client.ts"), rendered)?;
    Ok(())
}

/// Renders a single endpoint module (e.g. `src/endpoints/index.ts`).
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
        let ts_method = spec
            .method_mapping
            .get(&mapping_key)
            .map(|m| m.typescript.clone())
            .unwrap_or_else(|| ep_name.clone());

        let response_type = method_name_to_type_name(&ts_method);
        let params: Vec<ParamInfo> = ep
            .params
            .iter()
            .map(|p| ParamInfo {
                name: p.name.clone(),
                description: p.description.clone(),
            })
            .collect();

        methods.push(MethodInfo {
            ts_name: ts_method,
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

    let rendered = tera.render("src/endpoints/category.ts.tera", &ctx)?;
    fs::write(endpoints_dir.join(format!("{}.ts", category)), rendered)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_name_to_type_name() {
        assert_eq!(
            method_name_to_type_name("getKrxIndexDaily"),
            "KrxIndexDailyRecord"
        );
        assert_eq!(method_name_to_type_name("getEtfDaily"), "EtfDailyRecord");
        assert_eq!(
            method_name_to_type_name("getKospiStockDaily"),
            "KospiStockDailyRecord"
        );
    }

    #[test]
    fn test_generate_creates_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_dir = temp_dir.path();

        generate(out_dir.to_str().unwrap()).unwrap();

        // Config files.
        assert!(out_dir.join("krx/package.json").exists());
        assert!(out_dir.join("krx/README.md").exists());
        assert!(out_dir.join("krx/tsconfig.json").exists());

        // Source files.
        assert!(out_dir.join("krx/src/index.ts").exists());
        assert!(out_dir.join("krx/src/client.ts").exists());
        assert!(out_dir.join("krx/src/types.ts").exists());
        assert!(out_dir.join("krx/src/endpoints/index.ts").exists());
        assert!(out_dir.join("krx/src/endpoints/stock.ts").exists());
        assert!(out_dir.join("krx/src/endpoints/etp.ts").exists());
        assert!(out_dir.join("krx/src/endpoints/derivatives.ts").exists());

        // Verify AUTO-GENERATED header in .ts files.
        let client = std::fs::read_to_string(out_dir.join("krx/src/client.ts")).unwrap();
        assert!(client.starts_with("// AUTO-GENERATED"));

        let types = std::fs::read_to_string(out_dir.join("krx/src/types.ts")).unwrap();
        assert!(types.starts_with("// AUTO-GENERATED"));

        let index_ep = std::fs::read_to_string(out_dir.join("krx/src/endpoints/index.ts")).unwrap();
        assert!(index_ep.starts_with("// AUTO-GENERATED"));
        assert!(index_ep.contains("getKrxIndexDaily"));

        // Verify package.json contains AUTO-GENERATED in description.
        let pkg = std::fs::read_to_string(out_dir.join("krx/package.json")).unwrap();
        assert!(pkg.contains("AUTO-GENERATED"));
        // temp_dir is automatically cleaned up on drop.
    }
}
