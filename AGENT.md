# AGENT.md

This file provides guidance for AI agents (Claude, OpenClaw, Copilot, etc.) working on the `krxon` codebase.

---

## Project Overview

`krxon` is a Rust-based CLI tool and MCP server for the KRX (Korea Exchange) Open API.
It provides three core capabilities:

- **`fetch`** — Query KRX market data directly from the command line
- **`generate`** — Generate Python and TypeScript SDK clients from the spec
- **`serve`** — Run as an MCP server for AI agent integration (OpenClaw, Claude Desktop, etc.)

---

## Repository Structure

```
krxon/
├── AGENT.md                  ← You are here
├── Cargo.toml
├── spec/
│   └── endpoints.yaml        ← Source of Truth for all endpoints
├── src/
│   ├── main.rs
│   ├── cli.rs                ← CLI entrypoint (clap)
│   ├── client.rs             ← KRX HTTP client
│   ├── error.rs              ← Error types (thiserror)
│   ├── utils.rs              ← Date validation, formatting
│   ├── endpoints/
│   │   ├── mod.rs
│   │   ├── index.rs          ← Index endpoints (KRX/KOSPI/KOSDAQ/Derivatives)
│   │   ├── stock.rs          ← Stock endpoints (KOSPI/KOSDAQ daily + info)
│   │   ├── etp.rs            ← ETF/ETN endpoints
│   │   └── derivatives.rs    ← Futures/Options endpoints
│   ├── codegen/
│   │   ├── mod.rs
│   │   ├── python.rs         ← Python SDK generator
│   │   └── typescript.rs     ← TypeScript SDK generator
│   └── mcp/
│       ├── mod.rs
│       └── server.rs         ← MCP server (stdio transport)
├── templates/
│   ├── python/               ← Tera templates for Python codegen
│   └── typescript/           ← Tera templates for TypeScript codegen
└── docs/
    ├── architecture.md       ← System design and decisions
    ├── endpoints.md          ← KRX API endpoint reference
    ├── codegen.md            ← Code generation guide
    ├── mcp.md                ← MCP server setup and tool reference
    └── contributing.md       ← Contribution guidelines
```

---

## Documentation Policy

> **Always read and keep the `docs/` directory up to date.**

### Before making changes

1. Read the relevant document in `docs/` before modifying any module.
2. If a document does not exist yet, create it before or alongside your implementation.

### After making changes

Update the corresponding `docs/` file whenever you:

| Change | Document to update |
|---|---|
| Add or modify an endpoint | `docs/endpoints.md` |
| Change architecture or module structure | `docs/architecture.md` |
| Modify codegen templates or logic | `docs/codegen.md` |
| Change MCP tool definitions or server behavior | `docs/mcp.md` |
| Update build, test, or contribution process | `docs/contributing.md` |

Documentation must stay in sync with the code. A PR that changes behavior without updating the relevant `docs/` file is considered incomplete.

---

## Source of Truth

**`spec/endpoints.yaml` is the single source of truth** for all KRX API endpoints.

- All `fetch` subcommands are derived from this file.
- All generated SDK code (Python, TypeScript) is derived from this file.
- All MCP tool definitions are derived from this file.

When adding a new endpoint:
1. Add it to `spec/endpoints.yaml` first.
2. Implement the corresponding fetch subcommand.
3. Regenerate SDK outputs if needed (`krxon generate python/typescript`).
4. Update `docs/endpoints.md`.

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

# Run MCP server (stdio mode)
cargo run -- serve --stdio
```

---

## MCP Integration (OpenClaw / Claude Desktop)

Add the following to your MCP client configuration:

```json
{
  "mcpServers": {
    "krxon": {
      "command": "krxon",
      "args": ["serve", "--stdio"],
      "env": {
        "KRX_API_KEY": "your_api_key_here"
      }
    }
  }
}
```

Available MCP tools are documented in `docs/mcp.md`.

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

Before starting work on an issue, check whether a related `docs/` file needs to be read or created.

---

## Important Notes

- All numeric fields returned by the KRX API are `string` type — do not assume numeric types.
- `basDd` must be a valid business day. Requests on holidays return empty data or an error.
- Each KRX API endpoint requires a **separate service subscription** at `openapi.krx.co.kr` in addition to having an API key.
- When `isinCd` is omitted, the API returns data for all listed securities — this can be a large payload, especially for derivatives.
