//! Integration test: generates TypeScript SDK and verifies structure and tsc compilation.

use std::process::Command;

#[test]
fn test_typescript_sdk_generates_and_compiles() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let out_dir = temp_dir.path();

    // Generate TypeScript SDK.
    let status = Command::new(env!("CARGO_BIN_EXE_krxon"))
        .args(["generate", "typescript", "--out"])
        .arg(out_dir.to_str().unwrap())
        .status()
        .expect("Failed to run krxon generate");
    assert!(status.success(), "krxon generate typescript failed");

    // Verify all expected files exist.
    let expected_files = [
        "krx/package.json",
        "krx/README.md",
        "krx/tsconfig.json",
        "krx/src/index.ts",
        "krx/src/client.ts",
        "krx/src/types.ts",
        "krx/src/endpoints/index.ts",
        "krx/src/endpoints/stock.ts",
        "krx/src/endpoints/etp.ts",
        "krx/src/endpoints/derivatives.ts",
    ];
    for file in &expected_files {
        assert!(
            out_dir.join(file).exists(),
            "Expected file not found: {}",
            file
        );
    }

    // Verify AUTO-GENERATED header in .ts files.
    let ts_files = [
        "krx/src/index.ts",
        "krx/src/client.ts",
        "krx/src/types.ts",
        "krx/src/endpoints/index.ts",
        "krx/src/endpoints/stock.ts",
        "krx/src/endpoints/etp.ts",
        "krx/src/endpoints/derivatives.ts",
    ];
    for file in &ts_files {
        let content = std::fs::read_to_string(out_dir.join(file)).unwrap();
        assert!(
            content.starts_with("// AUTO-GENERATED"),
            "File {} missing AUTO-GENERATED header",
            file
        );
    }

    // Verify package.json contains AUTO-GENERATED in description.
    let pkg = std::fs::read_to_string(out_dir.join("krx/package.json")).unwrap();
    assert!(
        pkg.contains("AUTO-GENERATED"),
        "package.json missing AUTO-GENERATED marker"
    );

    // Run tsc type-check (requires npm/npx with typescript installed).
    let krx_dir = out_dir.join("krx");

    // Install typescript locally.
    let npm_install = Command::new("npm")
        .args(["install", "--save-dev", "typescript"])
        .current_dir(&krx_dir)
        .output();

    match npm_install {
        Ok(output) if output.status.success() => {
            // Run tsc --noEmit to verify the generated code compiles.
            let tsc_check = Command::new("npx")
                .args(["tsc", "--noEmit"])
                .current_dir(&krx_dir)
                .output()
                .expect("Failed to run npx tsc");

            assert!(
                tsc_check.status.success(),
                "tsc --noEmit failed.\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&tsc_check.stdout),
                String::from_utf8_lossy(&tsc_check.stderr),
            );
        }
        Ok(output) => {
            eprintln!(
                "TypeScript compile test skipped: npm install failed.\nstderr: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(_) => {
            eprintln!("TypeScript compile test skipped: npm not found");
        }
    }
    // temp_dir is automatically cleaned up on drop.
}
