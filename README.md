# krxon

CLI tool and MCP server for the KRX (Korea Exchange) Open API.

## Features

- **fetch** - Query KRX market data from the command line
- **generate** - Generate Python and TypeScript SDK clients from the API spec
- **serve** - Run as an MCP server for AI agent integration

## Getting Started

### Prerequisites

- Rust 1.75+ (2021 edition)
- A KRX Open API key from [openapi.krx.co.kr](https://openapi.krx.co.kr)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/seungdols/krxon.git
   cd krxon
   ```

2. Copy the environment file:
   ```bash
   cp .env.example .env
   ```

3. Edit `.env` and add your API key.

4. Build the project:
   ```bash
   cargo build
   ```

### Usage

```bash
# Fetch KRX index data
cargo run -- fetch

# Generate SDK clients
cargo run -- generate

# Run MCP server
cargo run -- serve
```

## Project Structure

See [AGENT.md](AGENT.md) for the full project structure and conventions.

## License

MIT
