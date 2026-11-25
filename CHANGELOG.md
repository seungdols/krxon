# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4] - 2026-04-01

### Changed

- Upgraded transitive TLS-related dependencies to patched versions to address RustSec advisories:
  - `aws-lc-sys` `0.38.0` -> `0.39.1`
  - `rustls-webpki` `0.103.9` -> `0.103.10`
  - `quinn-proto` `0.11.13` -> `0.11.14`
- Replaced unmaintained/unsound YAML stack `serde_yml` + `libyml` with `serde_norway`.
- Bumped version to `0.1.4`.

## [0.1.3] - 2026-03-14

### Changed

- Included README files in generated/published npm and PyPI packages.
- Added crate packaging verification step to ensure README is included before crates.io publish.
- Bumped version to `0.1.3`.

## [0.1.2] - 2026-03-13

### Changed

- Bumped version to `0.1.2`.

## [0.1.1] - 2026-03-11

### Changed

- Hardened release workflow security by pinning GitHub Actions to commit SHAs and reducing default token permissions.
- Reduced API key exposure risk by redacting secrets in `Debug` output and removing `Debug` derive on CLI argument types.
- Added security guidance for `--key` usage and platform warning for non-Unix config permission handling.

## [0.1.0] - 2026-03-10

### Added

- CLI tool for KRX (Korea Exchange) Open API
- `fetch` command for querying market data (index, stock, ETP, derivatives)
- `generate python` command for Python SDK code generation
- `generate typescript` command for TypeScript SDK code generation
- `init` / `clean` commands for API key configuration
- Support for table and JSON output formats
- ISIN code filtering for stock and ETP endpoints
- Python SDK with httpx-based KrxClient
- TypeScript SDK with fetch-based KrxClient (ES module)
- Endpoint specification as single source of truth (`spec/endpoints.yaml`)

[Unreleased]: https://github.com/seungdols/krxon/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/seungdols/krxon/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/seungdols/krxon/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/seungdols/krxon/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/seungdols/krxon/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/seungdols/krxon/releases/tag/v0.1.0
