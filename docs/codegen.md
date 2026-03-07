# Code Generation Guide

## 개요

krxon은 `spec/endpoints.yaml`을 기반으로 Python과 TypeScript SDK 클라이언트를 자동 생성합니다.
Tera 템플릿 엔진을 사용하여 `templates/` 디렉토리의 템플릿으로부터 코드를 렌더링합니다.

## 사용법

```bash
# Python SDK 생성
krxon generate python --out ./sdk/python

# TypeScript SDK 생성
krxon generate typescript --out ./sdk/typescript
```

## 생성 구조

### Python SDK

```
<out_dir>/krx/
├── __init__.py              # 패키지 초기화
├── client.py                # KrxClient 클래스 (카테고리 Mixin 상속)
├── types.py                 # TypedDict 응답 타입 정의
└── endpoints/
    ├── __init__.py
    ├── index.py             # IndexMixin (4개 메서드)
    ├── stock.py             # StockMixin (4개 메서드)
    ├── etp.py               # EtpMixin (2개 메서드)
    └── derivatives.py       # DerivativesMixin (6개 메서드)
```

### TypeScript SDK

```
<out_dir>/krx/
├── package.json             # @krxon/krx 패키지
├── tsconfig.json            # TypeScript 설정
└── src/
    ├── index.ts             # 공개 API re-export
    ├── client.ts            # KrxClient 클래스
    ├── types.ts             # interface 응답 타입 정의
    └── endpoints/
        ├── index.ts         # IndexMixin (4개 메서드)
        ├── stock.ts         # StockMixin (4개 메서드)
        ├── etp.ts           # EtpMixin (2개 메서드)
        └── derivatives.ts   # DerivativesMixin (6개 메서드)
```

## 메서드 네이밍 규칙

`spec/endpoints.yaml`의 `method_mapping`에서 정의됩니다:

| 엔드포인트 | Python (snake_case) | TypeScript (camelCase) |
|-----------|--------------------|-----------------------|
| index.krx_daily | `get_krx_index_daily` | `getKrxIndexDaily` |
| stock.kospi_daily | `get_kospi_stock_daily` | `getKospiStockDaily` |
| etp.etf_daily | `get_etf_daily` | `getEtfDaily` |
| derivatives.futures_daily | `get_futures_daily` | `getFuturesDaily` |

## 타입 생성 규칙

메서드명으로부터 타입명을 자동 파생합니다:

- **Python**: `get_krx_index_daily` → `KrxIndexDailyRecord` (TypedDict)
- **TypeScript**: `getKrxIndexDaily` → `KrxIndexDailyRecord` (interface)

변환 로직:
1. `get` / `get_` 접두사 제거
2. PascalCase 변환
3. `Record` 접미사 추가

## 템플릿 구조

### Python 템플릿 (`templates/python/`)

| 파일 | 설명 |
|-----|------|
| `__init__.py.tera` | 패키지 초기화 |
| `client.py.tera` | KrxClient 클래스 (base_url, Mixin 상속) |
| `types.py.tera` | TypedDict 정의 (응답 필드 기반) |
| `endpoints/__init__.py.tera` | 엔드포인트 패키지 초기화 |
| `endpoints/category.py.tera` | 카테고리별 Mixin 클래스 |

### TypeScript 템플릿 (`templates/typescript/`)

| 파일 | 설명 |
|-----|------|
| `package.json.tera` | npm 패키지 설정 |
| `tsconfig.json.tera` | TypeScript 컴파일러 설정 |
| `src/index.ts.tera` | 공개 API re-export |
| `src/client.ts.tera` | KrxClient 클래스 |
| `src/types.ts.tera` | interface 타입 정의 |
| `src/endpoints/category.ts.tera` | 카테고리별 Mixin 클래스 |

## 생성 흐름

```
1. spec/endpoints.yaml 로드 (codegen/spec.rs)
2. Tera 템플릿 초기화
3. 출력 디렉토리 생성
4. 공통 파일 렌더링 (client, types, init 등)
5. 카테고리별 엔드포인트 모듈 렌더링 (index, stock, etp, derivatives)
6. 파일 시스템에 기록
```

## 새 엔드포인트 추가 시

1. `spec/endpoints.yaml`에 엔드포인트 정의 추가
2. `method_mapping`에 Python/TypeScript 메서드명 추가
3. `krxon generate python/typescript` 재실행
4. 생성된 코드에 `# AUTO-GENERATED` / `// AUTO-GENERATED` 헤더가 포함됨

## 테스트

```bash
# 코드 생성 스모크 테스트
cargo test --test python_codegen_smoke
cargo test --test typescript_codegen_smoke

# 단위 테스트 (타입 변환, 파일 생성 검증)
cargo test codegen
```
