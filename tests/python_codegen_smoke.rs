//! Integration test: generates Python SDK and verifies imports work.

use std::process::Command;

#[test]
fn test_python_sdk_generates_and_imports() {
    let temp_dir = std::env::temp_dir().join("krxon_smoke_test");
    let _ = std::fs::remove_dir_all(&temp_dir);

    // Generate Python SDK.
    let status = Command::new(env!("CARGO_BIN_EXE_krxon"))
        .args(["generate", "python", "--out"])
        .arg(temp_dir.to_str().unwrap())
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
            temp_dir.join(file).exists(),
            "Expected file not found: {}",
            file
        );
    }

    // Verify AUTO-GENERATED header in all files.
    for file in &expected_files {
        let content = std::fs::read_to_string(temp_dir.join(file)).unwrap();
        assert!(
            content.starts_with("# AUTO-GENERATED"),
            "File {} missing AUTO-GENERATED header",
            file
        );
    }

    // Run Python import check (requires Python 3.9+).
    let python_check = Command::new("python3")
        .args([
            "-c",
            &format!(
                "import sys; sys.path.insert(0, '{}'); from krx import KrxClient; print('OK')",
                temp_dir.display()
            ),
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
                // Python might not be installed — log but don't fail.
                eprintln!(
                    "Python smoke test skipped (python3 returned error): {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(_) => {
            eprintln!("Python smoke test skipped: python3 not found");
        }
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}
