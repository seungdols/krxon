# KRX API Endpoint Reference

## 공통 사항

| 항목 | 값 |
|-----|---|
| Base URL | `https://data-dbg.krx.co.kr/svc/apis` |
| HTTP Method | POST |
| Content-Type | application/json |
| 인증 | `AUTH_KEY` 헤더에 API 키 전송 |
| 응답 루트 키 | `OutBlock_1` (배열) |
| 날짜 형식 | `YYYYMMDD` (영업일만 유효) |
| 호출 제한 | AUTH_KEY당 일일 10,000건 |
| 데이터 갱신 | 전일 데이터는 익영업일 08:00 KST |
| 필드 타입 | 모든 응답 필드는 string (숫자 포함, 콤마 포함 가능) |
| 서비스 신청 | 각 엔드포인트별 `openapi.krx.co.kr`에서 별도 신청 필요 |

## 엔드포인트 목록 (16개)

### 지수 (Index) — 4개

| CLI 명령 | API 경로 | 설명 |
|---------|---------|------|
| `fetch index krx` | `/idx/krx_dd_trd` | KRX 지수 일별 시세 |
| `fetch index kospi` | `/idx/kospi_dd_trd` | KOSPI 지수 일별 시세 |
| `fetch index kosdaq` | `/idx/kosdaq_dd_trd` | KOSDAQ 지수 일별 시세 |
| `fetch index derivatives` | `/idx/drvprod_dd_trd` | 파생상품 지수 일별 시세 |

**응답 필드**: `BAS_DD`, `IDX_CLSS`, `IDX_NM`, `CLSPRC_IDX`, `CMPPREVDD_IDX`, `FLUC_RT`, `OPNPRC_IDX`, `HGPRC_IDX`, `LWPRC_IDX`, `ACC_TRDVOL`, `ACC_TRDVAL`, `MKTCAP`

> 파생상품 지수는 `ACC_TRDVOL`, `ACC_TRDVAL`, `MKTCAP` 필드가 없습니다.

### 주식 (Stock) — 4개

| CLI 명령 | API 경로 | 설명 |
|---------|---------|------|
| `fetch stock kospi` | `/sto/stk_bydd_trd` | KOSPI 주식 일별 시세 |
| `fetch stock kosdaq` | `/sto/ksq_bydd_trd` | KOSDAQ 주식 일별 시세 |
| `fetch stock kospi-info` | `/sto/stk_isu_base_info` | KOSPI 종목 기본 정보 |
| `fetch stock kosdaq-info` | `/sto/ksq_isu_base_info` | KOSDAQ 종목 기본 정보 |

**일별 시세 응답 필드**: `BAS_DD`, `ISU_CD`, `ISU_NM`, `MKT_NM`, `SECT_TP_NM`, `TDD_CLSPRC`, `CMPPREVDD_PRC`, `FLUC_RT`, `TDD_OPNPRC`, `TDD_HGPRC`, `TDD_LWPRC`, `ACC_TRDVOL`, `ACC_TRDVAL`, `MKTCAP`, `LIST_SHRS`

**기본 정보 응답 필드**: `ISU_CD`, `ISU_SRT_CD`, `ISU_NM`, `ISU_ABBRV`, `ISU_ENG_NM`, `LIST_DD`, `MKT_TP_NM`, `SECUGRP_NM`, `SECT_TP_NM`, `KIND_STKCERT_TP_NM`, `PARVAL`, `LIST_SHRS`

**추가 옵션**: `--isin <ISIN_CODE>` — 특정 종목 필터링 (생략 시 전체 종목 반환)

### ETP (ETF/ETN) — 2개

| CLI 명령 | API 경로 | 설명 |
|---------|---------|------|
| `fetch etp etf` | `/etp/etf_bydd_trd` | ETF 일별 시세 |
| `fetch etp etn` | `/etp/etn_bydd_trd` | ETN 일별 시세 |

**ETF 응답 필드**: `BAS_DD`, `ISU_CD`, `ISU_NM`, `TDD_CLSPRC`, `CMPPREVDD_PRC`, `FLUC_RT`, `TDD_OPNPRC`, `TDD_HGPRC`, `TDD_LWPRC`, `ACC_TRDVOL`, `ACC_TRDVAL`, `MKTCAP`, `LIST_SHRS`, `NAV`, `IDX_IND_NM`, `OBJ_STKPRC_IDX`, `CMPPREVDD_IDX`, `FLUC_RT_IDX`, `INVSTASST_NETASST_TOTAMT`

**ETN 응답 필드**: `BAS_DD`, `ISU_CD`, `ISU_NM`, `TDD_CLSPRC`, `CMPPREVDD_PRC`, `FLUC_RT`, `TDD_OPNPRC`, `TDD_HGPRC`, `TDD_LWPRC`, `ACC_TRDVOL`, `ACC_TRDVAL`, `MKTCAP`, `LIST_SHRS`, `IDX_IND_NM`, `OBJ_STKPRC_IDX`, `CMPPREVDD_IDX`, `FLUC_RT_IDX`, `INDIC_VAL_AMT`, `PER1SECU_INDIC_VAL`

**추가 옵션**: `--isin <ISIN_CODE>` — 특정 종목 필터링

> ETN 데이터는 2014-11-17부터 제공됩니다. 이전 날짜 조회 시 경고 메시지가 출력됩니다.

### 파생상품 (Derivatives) — 6개

| CLI 명령 | API 경로 | 설명 |
|---------|---------|------|
| `fetch derivatives futures` | `/drv/fut_bydd_trd` | 선물 일별 시세 |
| `fetch derivatives stock-futures-kospi` | `/drv/eqsfu_stk_bydd_trd` | KOSPI 주식선물 일별 시세 |
| `fetch derivatives stock-futures-kosdaq` | `/drv/eqkfu_ksq_bydd_trd` | KOSDAQ 주식선물 일별 시세 |
| `fetch derivatives options` | `/drv/opt_bydd_trd` | 옵션 일별 시세 |
| `fetch derivatives stock-options-kospi` | `/drv/eqsop_bydd_trd` | KOSPI 주식옵션 일별 시세 |
| `fetch derivatives stock-options-kosdaq` | `/drv/eqkop_bydd_trd` | KOSDAQ 주식옵션 일별 시세 |

**선물 응답 필드**: `BAS_DD`, `ISU_CD`, `ISU_NM`, `PROD_NM`, `MKT_NM`, `TDD_CLSPRC`, `TDD_OPNPRC`, `TDD_HGPRC`, `TDD_LWPRC`, `SETL_PRC`, `SPOT_PRC`, `CMPPREVDD_PRC`, `ACC_TRDVOL`, `ACC_TRDVAL`, `ACC_OPNINT_QTY`

**옵션 응답 필드**: `BAS_DD`, `ISU_CD`, `ISU_NM`, `PROD_NM`, `RGHT_TP_NM`, `TDD_CLSPRC`, `TDD_OPNPRC`, `TDD_HGPRC`, `TDD_LWPRC`, `CMPPREVDD_PRC`, `ACC_TRDVOL`, `ACC_TRDVAL`, `ACC_OPNINT_QTY`, `IMP_VOLT`, `NXTDD_BAS_PRC`

> `isinCd` 생략 시 전체 종목 데이터가 반환됩니다. 파생상품의 경우 대용량 페이로드에 주의하세요.

## CLI 공통 옵션

| 옵션 | 설명 | 기본값 |
|-----|------|-------|
| `--date YYYYMMDD` | 기준일자 (필수) | — |
| `--key <API_KEY>` | API 키 (KRX_API_KEY 환경변수 대체) | — |
| `--output json\|table` | 출력 형식 | `json` |
| `--isin <ISIN_CODE>` | 종목 필터 (주식/ETP만) | — |

## 사용 예시

```bash
# KOSPI 지수 조회
krxon fetch index kospi --date 20250301

# 삼성전자 주식 조회
krxon fetch stock kospi --date 20250301 --isin KR7005930003

# ETF 테이블 출력
krxon fetch etp etf --date 20250301 --output table

# 선물 시세 조회
krxon fetch derivatives futures --date 20250301
```
