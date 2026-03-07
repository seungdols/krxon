# krxon

CLI tool for the KRX (Korea Exchange) Open API.

## Features

- **fetch** - Query KRX market data from the command line
- **generate** - Generate Python and TypeScript SDK clients from the API spec

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

#### Fetch Index Data

```bash
krxon fetch index krx --date 20250301
krxon fetch index kospi --date 20250301
krxon fetch index kosdaq --date 20250301
krxon fetch index derivatives --date 20250301
```

#### Fetch Stock Data

```bash
krxon fetch stock kospi --date 20250301
krxon fetch stock kosdaq --date 20250301
krxon fetch stock kospi-info --date 20250301
krxon fetch stock kosdaq-info --date 20250301

# Filter by ISIN code
krxon fetch stock kospi --date 20250301 --isin KR7005930003
```

#### Fetch ETP Data

```bash
krxon fetch etp etf --date 20250301
krxon fetch etp etn --date 20250301

# Filter by ISIN code
krxon fetch etp etf --date 20250301 --isin KR7069500007
```

#### Fetch Derivatives Data

```bash
# Futures
krxon fetch derivatives futures --date 20250301
krxon fetch derivatives stock-futures-kospi --date 20250301
krxon fetch derivatives stock-futures-kosdaq --date 20250301

# Options
krxon fetch derivatives options --date 20250301
krxon fetch derivatives stock-options-kospi --date 20250301
krxon fetch derivatives stock-options-kosdaq --date 20250301

# Table output
krxon fetch derivatives futures --date 20250301 --output table
```

#### Generate SDK Clients

```bash
# Generate Python SDK
krxon generate python --out ./sdk/python

# Generate TypeScript SDK
krxon generate typescript --out ./sdk/typescript
```

### Common Options

- `--date` (required): Base date in YYYYMMDD format
- `--key`: API key (overrides `KRX_API_KEY` env var)
- `--output`: `json` (default) or `table`

## Project Structure

See [AGENT.md](AGENT.md) for the full project structure and conventions.

## License

MIT
