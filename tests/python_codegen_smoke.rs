//! Integration test: generates Python SDK and verifies imports work.

use std::process::Command;

#[test]
fn test_python_sdk_generates_and_imports() {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let out_dir = temp_dir.path();

    // Generate Python SDK.
    let status = Command::new(env!("CARGO_BIN_EXE_krxon"))
        .args(["generate", "python", "--out"])
        .arg(out_dir.to_str().unwrap())
        .status()
        .expect("Failed to run krxon generate");
    assert!(status.success(), "krxon generate python failed");

    // Verify all expected files exist.
    let expected_files = [
        "krx/__init__.py",
        "krx/client.py",
        "krx/types.py",
        "krx/endpoints/__init__.py",
        "krx/endpoints/index.py",
        "krx/endpoints/stock.py",
        "krx/endpoints/etp.py",
        "krx/endpoints/derivatives.py",
    ];
    for file in &expected_files {
        assert!(
            out_dir.join(file).exists(),
            "Expected file not found: {}",
            file
        );
    }

    // Verify AUTO-GENERATED header in all files.
    for file in &expected_files {
        let content = std::fs::read_to_string(out_dir.join(file)).unwrap();
        assert!(
            content.starts_with("# AUTO-GENERATED"),
            "File {} missing AUTO-GENERATED header",
            file
        );
    }

    // Run Python import check (requires Python 3.9+ with httpx installed).
    // Pass path via env var to avoid quoting/escaping issues across platforms.
    let python_check = Command::new("python3")
        .env("KRXON_SDK_PATH", out_dir)
        .args([
            "-c",
            "import os, sys; sys.path.insert(0, os.environ['KRXON_SDK_PATH']); from krx import KrxClient; print('OK')",
        ])
        .output();

    match python_check {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                assert!(
                    stdout.contains("OK"),
                    "Python import check failed: {}",
                    stdout
                );
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // Only skip if httpx is not installed (dependency not available).
                if stderr.contains("No module named") && stderr.contains("httpx") {
                    eprintln!("Python smoke test skipped: httpx not installed");
                } else {
                    panic!(
                        "Python import check failed with status {}.\nstdout: {}\nstderr: {}",
                        output.status,
                        String::from_utf8_lossy(&output.stdout),
                        stderr,
                    );
                }
            }
        }
        Err(_) => {
            eprintln!("Python smoke test skipped: python3 not found");
        }
    }
    // temp_dir is automatically cleaned up on drop.
}
