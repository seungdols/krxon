# AGENT.md

This file provides guidance for AI agents (Claude, OpenClaw, Copilot, etc.) working on the `krxon` codebase.

---

## Project Overview

`krxon` is a Rust-based CLI tool for the KRX (Korea Exchange) Open API.
It provides two core capabilities:

- **`fetch`** — Query KRX market data directly from the command line
- **`generate`** — Generate Python and TypeScript SDK clients from the spec

> **Planned**: `serve` — MCP server for AI agent integration (향후 구현 예정)

---

## Repository Structure

```
krxon/
├── AGENT.md                  ← You are here
├── README.md
├── Cargo.toml
├── .env.example              ← Environment variable template
├── spec/
│   └── endpoints.yaml        ← Source of Truth for all endpoints
├── src/
│   ├── main.rs               ← Async entrypoint (tokio)
│   ├── cli.rs                ← CLI argument parsing (clap derive)
│   ├── client.rs             ← KRX HTTP client (reqwest)
│   ├── error.rs              ← Domain error types (thiserror)
│   ├── output.rs             ← Table formatting (comfy-table)
│   ├── utils.rs              ← Date validation, formatting
│   ├── endpoints/
│   │   ├── mod.rs
│   │   ├── index.rs          ← Index endpoints (KRX/KOSPI/KOSDAQ/Derivatives)
│   │   ├── stock.rs          ← Stock endpoints (KOSPI/KOSDAQ daily + info)
│   │   ├── etp.rs            ← ETF/ETN endpoints
│   │   └── derivatives.rs    ← Futures/Options endpoints
│   └── codegen/
│       ├── mod.rs
│       ├── spec.rs           ← YAML spec loader
│       ├── python.rs         ← Python SDK generator
│       └── typescript.rs     ← TypeScript SDK generator
├── templates/
│   ├── python/
│   │   ├── __init__.py.tera
│   │   ├── client.py.tera
│   │   ├── types.py.tera
│   │   └── endpoints/
│   │       ├── __init__.py.tera
│   │       └── category.py.tera
│   └── typescript/
│       ├── package.json.tera
│       ├── tsconfig.json.tera
│       └── src/
│           ├── index.ts.tera
│           ├── client.ts.tera
│           ├── types.ts.tera
│           └── endpoints/
│               └── category.ts.tera
├── tests/
│   ├── python_codegen_smoke.rs
│   └── typescript_codegen_smoke.rs
└── docs/
    ├── architecture.md       ← 시스템 설계 및 모듈 구조
    ├── endpoints.md          ← KRX API 엔드포인트 레퍼런스
    ├── codegen.md            ← 코드 생성 가이드
    └── contributing.md       ← 기여 가이드
```

---

## Documentation Policy

> **코드 변경 시 관련 문서를 함께 업데이트합니다.**

### 변경 시 업데이트할 문서

| 변경 내용 | 업데이트 대상 |
|---------|------------|
| 엔드포인트 추가/수정 | `docs/endpoints.md` |
| 아키텍처/모듈 구조 변경 | `docs/architecture.md` |
| 코드 생성 템플릿/로직 변경 | `docs/codegen.md` |
| 빌드, 테스트, 기여 프로세스 변경 | `docs/contributing.md` |

코드 변경 없이 문서만 변경하는 것도 허용됩니다.

---

## Source of Truth

**`spec/endpoints.yaml` is the single source of truth** for all KRX API endpoints.

- All `fetch` subcommands are derived from this file.
- All generated SDK code (Python, TypeScript) is derived from this file.

When adding a new endpoint:
1. Add it to `spec/endpoints.yaml` first.
2. Implement the corresponding fetch subcommand.
3. Regenerate SDK outputs if needed (`krxon generate python/typescript`).

---

## Key Conventions

### API Communication

- **Base URL**: `https://data-dbg.krx.co.kr/svc/apis`
- **Method**: POST with JSON body
- **Auth**: HTTP header `AUTH_KEY: <api_key>`
- **Response root**: `OutBlock_1` array
- **Date format**: `YYYYMMDD` (営業日 only — no data on market holidays)
- **Rate limit**: 10,000 calls/day

### API Key Resolution Order

1. `--key` CLI flag
2. `KRX_API_KEY` environment variable
3. `~/.krxon/config.toml`

### Error Handling

Use `KrxError` (defined in `src/error.rs`) for all domain-level errors.
Use `anyhow::Result` for internal/unexpected errors.
Never use `unwrap()` or `expect()` in production code paths.

### Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all warnings.
- All public functions and structs must have doc comments (`///`).
- Tests live in the same file as the code under `#[cfg(test)]`.

---

## Build & Test

```bash
# Build
cargo build

# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Run CLI locally
cargo run -- fetch index kospi --date 20250301 --key $KRX_API_KEY

# Generate SDKs
cargo run -- generate python --out ./sdk/python
cargo run -- generate typescript --out ./sdk/typescript
```

---

## MCP Integration (향후 구현 예정)

`serve` 명령은 아직 구현되지 않았습니다. 구현 시 MCP 서버(stdio transport)로 AI 에이전트 통합을 지원할 예정입니다.

---

## GitHub Issues

All work is tracked via GitHub Issues. Labels used:

| Label | Meaning |
|---|---|
| `epic` | Large feature umbrella |
| `spec` | Specification or design |
| `setup` | Project initialization |
| `core` | Core shared modules |
| `feature` | Feature implementation |
| `fetch` | fetch command work |
| `codegen` | Code generation work |
| `mcp` | MCP server work |
| `release` | Release and deployment |

Before starting work on an issue, check whether `AGENT.md` or `README.md` needs to be updated.

---

## Important Notes

- All numeric fields returned by the KRX API are `string` type — do not assume numeric types.
- `basDd` must be a valid business day. Requests on holidays return empty data or an error.
- Each KRX API endpoint requires a **separate service subscription** at `openapi.krx.co.kr` in addition to having an API key.
- When `isinCd` is omitted, the API returns data for all listed securities — this can be a large payload, especially for derivatives.
