# krxon CLI Skill Guide

krxon is a CLI tool for querying KRX (Korea Exchange) market data.

## Prerequisites

API key is resolved in the following order:

1. `--key` flag passed directly to the command
2. `KRX_API_KEY` shell environment variable (`export KRX_API_KEY=your_key`)
3. Config file at `~/.krxon/config.json`:
   ```json
   { "api_key": "your_api_key" }
   ```

`--date` must be a business day in `YYYYMMDD` format. Holidays return empty data.

## Command Overview

`fetch` can be omitted. Both forms are equivalent:

```
krxon fetch index kospi --date 20250301
krxon index kospi --date 20250301
```

## Common Options

| Option     | Description                              | Required |
|------------|------------------------------------------|----------|
| `--date`   | Base date in YYYYMMDD format             | Yes      |
| `--key`    | API key (overrides `KRX_API_KEY` env)    | No       |
| `--output` | Output format: `json` (default), `table` | No       |
| `--isin`   | Filter by ISIN code (stock/etp only)     | No       |

## Commands

### Index Data

```bash
krxon index krx --date 20250301
krxon index kospi --date 20250301
krxon index kosdaq --date 20250301
krxon index derivatives --date 20250301
```

### Stock Data

```bash
krxon stock kospi --date 20250301
krxon stock kosdaq --date 20250301
krxon stock kospi-info --date 20250301
krxon stock kosdaq-info --date 20250301

# Filter by ISIN
krxon stock kospi --date 20250301 --isin KR7005930003
```

### ETP Data (ETF/ETN)

```bash
krxon etp etf --date 20250301
krxon etp etn --date 20250301

# Filter by ISIN
krxon etp etf --date 20250301 --isin KR7069500007
```

Note: ETN data is available from 2014-11-17 onwards.

### Derivatives Data

```bash
# Futures
krxon derivatives futures --date 20250301
krxon derivatives stock-futures-kospi --date 20250301
krxon derivatives stock-futures-kosdaq --date 20250301

# Options
krxon derivatives options --date 20250301
krxon derivatives stock-options-kospi --date 20250301
krxon derivatives stock-options-kosdaq --date 20250301
```

### Generate SDK Clients

```bash
krxon generate python --out ./sdk/python
krxon generate typescript --out ./sdk/typescript
```

## Output Formats

- `json` (default): Pretty-printed JSON to stdout.
- `table`: Formatted table to stdout. Use `--output table`.

## Examples

```bash
# KOSPI index as table
krxon index kospi --date 20250303 --output table

# Samsung Electronics stock by ISIN
krxon stock kospi --date 20250303 --isin KR7005930003

# All KOSDAQ stocks as JSON
krxon stock kosdaq --date 20250303

# Futures with explicit API key
krxon derivatives futures --date 20250303 --key YOUR_KEY
```
