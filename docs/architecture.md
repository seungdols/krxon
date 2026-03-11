# Architecture

## 개요

krxon은 Rust 기반 CLI 도구로, KRX(한국거래소) Open API를 통해 시장 데이터를 조회하고 Python/TypeScript SDK를 생성합니다.

## 모듈 구조

```
src/
├── main.rs          # 진입점, 명령 라우팅
├── cli.rs           # clap derive 기반 CLI 인자 정의
├── client.rs        # KRX HTTP 클라이언트 (reqwest)
├── error.rs         # 도메인 에러 타입 (thiserror)
├── output.rs        # 테이블/CSV 포맷팅 (comfy-table)
├── utils.rs         # 날짜 검증, 유틸리티 함수
├── endpoints/       # API 엔드포인트 구현
│   ├── index.rs     # 지수 (4개)
│   ├── stock.rs     # 주식 (4개)
│   ├── etp.rs       # ETF/ETN (2개)
│   └── derivatives.rs # 파생상품 (6개)
└── codegen/         # SDK 코드 생성
    ├── spec.rs      # YAML 스펙 파서
    ├── python.rs    # Python SDK 생성기
    └── typescript.rs # TypeScript SDK 생성기
```

## 데이터 흐름

### fetch 명령

```
CLI 입력 → cli.rs (clap 파싱)
         → main.rs (명령 라우팅)
         → client.rs (HTTP POST 요청)
         → KRX API 서버
         → OutBlock_1 응답 파싱
         → endpoints/*.rs (레코드 역직렬화)
         → output.rs (JSON 또는 테이블 포맷팅)
         → stdout 출력
```

### generate 명령

```
CLI 입력 → cli.rs (clap 파싱)
         → codegen/spec.rs (endpoints.yaml 로드)
         → codegen/python.rs 또는 typescript.rs
         → templates/*.tera (Tera 템플릿 렌더링)
         → 파일 시스템에 SDK 출력
```

## 핵심 설계 결정

### Single Source of Truth

`spec/endpoints.yaml`이 모든 엔드포인트의 단일 진실 소스입니다:
- CLI fetch 명령의 레코드 구조체 정의 기반
- 생성되는 Python/TypeScript SDK의 메서드와 타입 정의
- 메서드명 매핑 (snake_case ↔ camelCase)

### 에러 처리 전략

- **도메인 에러**: `KrxError` (thiserror) — API 인증, 구독, 파싱 등
- **내부 에러**: `anyhow::Result` — 예상치 못한 오류
- **금지**: `unwrap()`, `expect()` — 프로덕션 코드에서 사용 금지

### HTTP 클라이언트

- `reqwest` 비동기 클라이언트 사용
- 30초 타임아웃 기본 설정
- `AUTH_KEY` 헤더로 인증
- 응답에서 `OutBlock_1` 배열 자동 추출

### 출력 포맷

- **JSON** (기본): `serde_json::to_string_pretty` — 필드 순서 보존
- **테이블**: `comfy-table` — 터미널 너비에 동적 맞춤

## 의존성

| 크레이트 | 용도 |
|---------|------|
| clap 4 | CLI 인자 파싱 (derive 매크로) |
| reqwest 0.12 | 비동기 HTTP 클라이언트 |
| serde / serde_json | JSON 직렬화/역직렬화 |
| serde_yaml 0.9 | YAML 스펙 파싱 |
| tokio 1 | 비동기 런타임 |
| anyhow 1 | 유연한 에러 처리 |
| thiserror 1 | 구조화된 에러 타입 |
| tera 1 | 코드 생성 템플릿 엔진 |
| comfy-table 7 | 테이블 포맷팅 |
| chrono 0.4 | 날짜/시간 유틸리티 |
