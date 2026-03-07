# Contributing Guide

## 개발 환경 설정

### 필수 요구사항

- Rust 1.75+ (`rustup` 권장)
- KRX API 키 ([openapi.krx.co.kr](https://openapi.krx.co.kr) 발급)

### 환경 변수 설정

```bash
cp .env.example .env
# .env 파일에 KRX_API_KEY 설정
export KRX_API_KEY=your_api_key_here
```

## 빌드 & 테스트

```bash
# 빌드
cargo build

# 전체 테스트
cargo test

# 린트
cargo clippy -- -D warnings

# 포맷팅
cargo fmt

# 포맷 검사 (CI용)
cargo fmt -- --check
```

## 코드 스타일

- `cargo fmt`로 자동 포맷팅
- `cargo clippy -- -D warnings`로 모든 경고 해결
- 공개 함수와 구조체에 doc comment (`///`) 필수
- 테스트는 같은 파일에 `#[cfg(test)]` 모듈로 작성
- 프로덕션 코드에서 `unwrap()`, `expect()` 사용 금지

## 에러 처리 규칙

- **도메인 에러**: `KrxError` (thiserror) 사용 — API 관련 에러
- **내부 에러**: `anyhow::Result` 사용 — 예상치 못한 에러
- 에러 메시지는 한국어로 작성 (사용자 대상)

## 새 엔드포인트 추가 절차

1. `spec/endpoints.yaml`에 엔드포인트 정의 추가
2. `method_mapping`에 Python/TypeScript 메서드명 추가
3. `src/endpoints/`에 해당 카테고리 모듈에 fetch 함수 구현
4. `src/cli.rs`에 서브커맨드 추가
5. `src/main.rs`에 핸들러 함수 추가
6. 테스트 작성 및 통과 확인
7. 관련 문서 업데이트 (`docs/endpoints.md`, `AGENT.md`)

## 커밋 컨벤션

```
<type>: <description>

# 타입 예시
feat:     새 기능 추가
fix:      버그 수정
refactor: 리팩토링
docs:     문서 변경
test:     테스트 추가/수정
chore:    빌드, 설정 등 기타
```

## GitHub 이슈 라벨

| 라벨 | 용도 |
|-----|------|
| `epic` | 대규모 기능 묶음 |
| `spec` | 스펙/설계 |
| `setup` | 프로젝트 초기 설정 |
| `core` | 공통 모듈 |
| `feature` | 기능 구현 |
| `fetch` | fetch 명령 관련 |
| `codegen` | 코드 생성 관련 |
| `mcp` | MCP 서버 관련 |
| `release` | 배포/릴리스 |

## 프로젝트 원칙

- **Single Source of Truth**: `spec/endpoints.yaml`이 모든 엔드포인트의 단일 진실 소스
- **문서 동기화**: 코드 변경 시 관련 문서(`AGENT.md`, `README.md`, `docs/`) 함께 업데이트
- **테스트 필수**: 새 기능은 반드시 테스트와 함께 커밋
