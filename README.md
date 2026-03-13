# krxon

[![Crates.io](https://img.shields.io/crates/v/krxon)](https://crates.io/crates/krxon)
[![PyPI](https://img.shields.io/pypi/v/krxon)](https://pypi.org/project/krxon/)
[![npm](https://img.shields.io/npm/v/@krxon/krx)](https://www.npmjs.com/package/@krxon/krx)
[![CI](https://github.com/seungdols/krxon/actions/workflows/ci.yml/badge.svg)](https://github.com/seungdols/krxon/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

KRX(한국거래소) Open API를 위한 CLI 도구 및 다국어 SDK 생성기입니다.

> **API 문서**: [seungdols.github.io/krxon](https://seungdols.github.io/krxon/)

## Features

- **index / stock / etp / derivatives** — CLI에서 KRX 시장 데이터 조회 (지수, 주식, ETP, 파생상품)
- **generate** — API 스펙으로부터 Python / TypeScript SDK 자동 생성
- **init / clean** — API 키 설정 및 초기화
- JSON, Table 두 가지 출력 포맷 지원
- ISIN 코드 필터링 지원 (주식, ETP)

## Supported Endpoints

| 카테고리 | 엔드포인트 | 설명 |
|---------|-----------|------|
| **지수** | `index krx` | KRX 일별 지수 |
| | `index kospi` | KOSPI 일별 지수 |
| | `index kosdaq` | KOSDAQ 일별 지수 |
| | `index derivatives` | 파생상품 일별 지수 |
| **주식** | `stock kospi` | KOSPI 일별 시세 |
| | `stock kosdaq` | KOSDAQ 일별 시세 |
| | `stock kospi-info` | KOSPI 종목 기본 정보 |
| | `stock kosdaq-info` | KOSDAQ 종목 기본 정보 |
| **ETP** | `etp etf` | ETF 일별 시세 |
| | `etp etn` | ETN 일별 시세 |
| **파생상품** | `derivatives futures` | 선물 일별 시세 |
| | `derivatives options` | 옵션 일별 시세 |
| | `derivatives stock-futures-kospi` | KOSPI 주식선물 일별 시세 |
| | `derivatives stock-futures-kosdaq` | KOSDAQ 주식선물 일별 시세 |
| | `derivatives stock-options-kospi` | KOSPI 주식옵션 일별 시세 |
| | `derivatives stock-options-kosdaq` | KOSDAQ 주식옵션 일별 시세 |

## Installation

### Cargo (Rust)

```bash
cargo install krxon
```

### Homebrew (macOS/Linux)

```bash
brew tap seungdols/krxon
brew install krxon
```

### Windows (winget)

```powershell
winget install seungdols.krxon
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

- KRX Open API 키 발급: [openapi.krx.co.kr](https://openapi.krx.co.kr)

### Setup

```bash
# API 키 저장 (대화형)
krxon init

# 또는 환경변수로 설정
export KRX_API_KEY=your_key_here
```

### CLI Usage

`fetch` 접두어는 선택 사항입니다.

- `krxon index ...` 와 `krxon fetch index ...` 는 동일하게 동작합니다.
- `krxon stock ...` 와 `krxon fetch stock ...` 는 동일하게 동작합니다.

#### 지수 데이터 조회

```bash
krxon index krx --date 20250301
krxon index kospi --date 20250301
krxon index kosdaq --date 20250301
krxon index derivatives --date 20250301
```

#### 주식 데이터 조회

```bash
krxon stock kospi --date 20250301
krxon stock kosdaq --date 20250301
krxon stock kospi-info --date 20250301
krxon stock kosdaq-info --date 20250301

# ISIN 코드로 특정 종목 필터링
krxon stock kospi --date 20250301 --isin KR7005930003
```

#### ETP 데이터 조회

```bash
krxon etp etf --date 20250301
krxon etp etn --date 20250301

# ISIN 코드로 필터링
krxon etp etf --date 20250301 --isin KR7069500007
```

#### 파생상품 데이터 조회

```bash
# 선물
krxon derivatives futures --date 20250301
krxon derivatives stock-futures-kospi --date 20250301
krxon derivatives stock-futures-kosdaq --date 20250301

# 옵션
krxon derivatives options --date 20250301
krxon derivatives stock-options-kospi --date 20250301
krxon derivatives stock-options-kosdaq --date 20250301

# 테이블 출력
krxon derivatives futures --date 20250301 --output table
```

#### SDK 코드 생성

```bash
# Python SDK 생성
krxon generate python --out ./sdk/python

# TypeScript SDK 생성
krxon generate typescript --out ./sdk/typescript
```

### Common Options

| 옵션 | 설명 | 기본값 |
|------|------|--------|
| `--date` | 기준일 (YYYYMMDD 형식, 필수) | — |
| `--key` | API 키 (`KRX_API_KEY` 환경변수 대체, 공유 환경 사용 주의) | — |
| `--output` | 출력 형식 | `json` |
| `--isin` | ISIN 코드 필터 (주식, ETP) | — |

보안 권장:
- 공유/다중 사용자 환경에서는 `--key` 대신 `KRX_API_KEY` 또는 `krxon init` 사용을 권장합니다.

## SDK Usage

### Python

```python
from krx import KrxClient

client = KrxClient(api_key="your_key")
records = client.get_kospi_stock_daily(basDd="20250301")
for r in records:
    print(r["ISU_NM"], r["TDD_CLSPRC"])
```

### TypeScript

```typescript
import { KrxClient } from "@krxon/krx";

const client = new KrxClient({ apiKey: "your_key" });
const records = await client.getKospiStockDaily("20250301");
records.forEach((r) => console.log(r.ISU_NM, r.TDD_CLSPRC));
```

## Project Structure

```
krxon/
├── src/              # Rust CLI 소스코드
├── spec/             # API 엔드포인트 스펙 (endpoints.yaml)
├── templates/        # SDK 코드 생성 템플릿 (Tera)
│   ├── python/       # Python SDK 템플릿
│   └── typescript/   # TypeScript SDK 템플릿
├── sdk/              # 생성된 SDK 코드
├── docs/             # API 문서 (GitHub Pages)
└── .github/          # CI/CD 워크플로우
```

자세한 내용은 [AGENT.md](AGENT.md)를 참고하세요.

배포 채널 운영 가이드는 [docs/distribution.md](docs/distribution.md)를 참고하세요.

## Changelog

변경 이력은 [CHANGELOG.md](CHANGELOG.md)를 참고하세요.

## License

MIT
