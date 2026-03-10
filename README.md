# krxon

CLI tool for the KRX (Korea Exchange) Open API.

## Features

- **fetch** - Query KRX market data from the command line
- **generate** - Generate Python and TypeScript SDK clients from the API spec

## Installation

### Cargo (Rust)

```bash
cargo install krxon
```

### Python SDK (PyPI)

```bash
pip install krxon
```

### TypeScript SDK (npm)

```bash
npm install @krxon/krx
```

### From source

```bash
git clone https://github.com/seungdols/krxon.git
cd krxon
cargo build --release
```

## Getting Started

### Prerequisites

- A KRX Open API key from [openapi.krx.co.kr](https://openapi.krx.co.kr)

### Setup

```bash
# Store your API key
krxon init

# Or set via environment variable
export KRX_API_KEY=your_key_here
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

## SDK Usage

### Python

```python
from krx import KrxClient

client = KrxClient(api_key="your_key")
records = client.get_kospi_stock_daily(base_date="20250301")
for r in records:
    print(r["ISU_NM"], r["TDD_CLSPRC"])
```

### TypeScript

```typescript
import { KrxClient } from "@krxon/krx";

const client = new KrxClient({ apiKey: "your_key" });
const records = await client.getKospiStockDaily({ basDate: "20250301" });
records.forEach((r) => console.log(r.ISU_NM, r.TDD_CLSPRC));
```

## Project Structure

See [AGENT.md](AGENT.md) for the full project structure and conventions.

## License

MIT
