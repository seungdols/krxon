# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/seungdols/krxon/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/seungdols/krxon/releases/tag/v0.1.0
