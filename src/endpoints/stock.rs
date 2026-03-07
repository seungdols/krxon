//! Stock endpoints (KOSPI/KOSDAQ daily trading and stock info).
//!
//! Covers 4 endpoints:
//! - KOSPI stock daily (`/sto/stk_bydd_trd`)
//! - KOSDAQ stock daily (`/sto/ksq_bydd_trd`)
//! - KOSPI stock info (`/sto/stk_isu_base_info`)
//! - KOSDAQ stock info (`/sto/ksq_isu_base_info`)

use serde::{Deserialize, Serialize};

use crate::client::KrxClient;
use crate::error::KrxError;

/// Stock daily trading record returned by KRX API.
///
/// Used by both KOSPI (`/sto/stk_bydd_trd`) and KOSDAQ (`/sto/ksq_bydd_trd`)
/// endpoints. All fields are strings as returned by the KRX API.
#[derive(Debug, Deserialize, Serialize)]
pub struct StockRecord {
    /// Base date (YYYYMMDD).
    #[serde(rename = "BAS_DD")]
    pub bas_dd: String,

    /// ISIN code.
    #[serde(rename = "ISU_CD")]
    pub isu_cd: String,

    /// Stock name.
    #[serde(rename = "ISU_NM")]
    pub isu_nm: String,

    /// Market name.
    #[serde(rename = "MKT_NM")]
    pub mkt_nm: String,

    /// Sector name.
    #[serde(rename = "SECT_TP_NM")]
    pub sect_tp_nm: String,

    /// Closing price.
    #[serde(rename = "TDD_CLSPRC")]
    pub tdd_clsprc: String,

    /// Change compared to previous day.
    #[serde(rename = "CMPPREVDD_PRC")]
    pub cmpprevdd_prc: String,

    /// Fluctuation rate (%).
    #[serde(rename = "FLUC_RT")]
    pub fluc_rt: String,

    /// Opening price.
    #[serde(rename = "TDD_OPNPRC")]
    pub tdd_opnprc: String,

    /// High price.
    #[serde(rename = "TDD_HGPRC")]
    pub tdd_hgprc: String,

    /// Low price.
    #[serde(rename = "TDD_LWPRC")]
    pub tdd_lwprc: String,

    /// Accumulated trading volume.
    #[serde(rename = "ACC_TRDVOL")]
    pub acc_trdvol: String,

    /// Accumulated trading value.
    #[serde(rename = "ACC_TRDVAL")]
    pub acc_trdval: String,

    /// Market capitalization.
    #[serde(rename = "MKTCAP")]
    pub mktcap: String,

    /// Listed shares.
    #[serde(rename = "LIST_SHRS")]
    pub list_shrs: String,
}

/// Stock base info record returned by KRX API.
///
/// Used by both KOSPI (`/sto/stk_isu_base_info`) and KOSDAQ
/// (`/sto/ksq_isu_base_info`) endpoints.
#[derive(Debug, Deserialize, Serialize)]
pub struct StockInfoRecord {
    /// ISIN code.
    #[serde(rename = "ISU_CD")]
    pub isu_cd: String,

    /// Short code.
    #[serde(rename = "ISU_SRT_CD")]
    pub isu_srt_cd: String,

    /// Stock name (Korean).
    #[serde(rename = "ISU_NM")]
    pub isu_nm: String,

    /// Stock abbreviation (Korean).
    #[serde(rename = "ISU_ABBRV")]
    pub isu_abbrv: String,

    /// Stock name (English).
    #[serde(rename = "ISU_ENG_NM")]
    pub isu_eng_nm: String,

    /// Listing date.
    #[serde(rename = "LIST_DD")]
    pub list_dd: String,

    /// Market type name.
    #[serde(rename = "MKT_TP_NM")]
    pub mkt_tp_nm: String,

    /// Security group name.
    #[serde(rename = "SECUGRP_NM")]
    pub secugrp_nm: String,

    /// Sector name.
    #[serde(rename = "SECT_TP_NM")]
    pub sect_tp_nm: String,

    /// Stock certificate type.
    #[serde(rename = "KIND_STKCERT_TP_NM")]
    pub kind_stkcert_tp_nm: String,

    /// Par value.
    #[serde(rename = "PARVAL")]
    pub parval: String,

    /// Listed shares.
    #[serde(rename = "LIST_SHRS")]
    pub list_shrs: String,
}

/// Fetches KOSPI stock daily trading data.
pub async fn fetch_kospi_stock(
    client: &KrxClient,
    date: &str,
    isin: Option<&str>,
) -> Result<Vec<StockRecord>, KrxError> {
    fetch_stock(client, "/sto/stk_bydd_trd", date, isin).await
}

/// Fetches KOSDAQ stock daily trading data.
pub async fn fetch_kosdaq_stock(
    client: &KrxClient,
    date: &str,
    isin: Option<&str>,
) -> Result<Vec<StockRecord>, KrxError> {
    fetch_stock(client, "/sto/ksq_bydd_trd", date, isin).await
}

/// Fetches KOSPI stock base info.
pub async fn fetch_kospi_stock_info(
    client: &KrxClient,
    date: &str,
    isin: Option<&str>,
) -> Result<Vec<StockInfoRecord>, KrxError> {
    fetch_stock_info(client, "/sto/stk_isu_base_info", date, isin).await
}

/// Fetches KOSDAQ stock base info.
pub async fn fetch_kosdaq_stock_info(
    client: &KrxClient,
    date: &str,
    isin: Option<&str>,
) -> Result<Vec<StockInfoRecord>, KrxError> {
    fetch_stock_info(client, "/sto/ksq_isu_base_info", date, isin).await
}

/// Internal helper: calls a stock daily endpoint with date and optional ISIN.
async fn fetch_stock(
    client: &KrxClient,
    path: &str,
    date: &str,
    isin: Option<&str>,
) -> Result<Vec<StockRecord>, KrxError> {
    let params = build_params(date, isin);
    let raw = client.post(path, params).await?;

    let records: Vec<StockRecord> = raw
        .into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| KrxError::ParseError(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

/// Internal helper: calls a stock info endpoint with date and optional ISIN.
async fn fetch_stock_info(
    client: &KrxClient,
    path: &str,
    date: &str,
    isin: Option<&str>,
) -> Result<Vec<StockInfoRecord>, KrxError> {
    let params = build_params(date, isin);
    let raw = client.post(path, params).await?;

    let records: Vec<StockInfoRecord> = raw
        .into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| KrxError::ParseError(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

/// Builds JSON params with `basDd` and optional `isinCd`.
fn build_params(date: &str, isin: Option<&str>) -> serde_json::Value {
    match isin {
        Some(code) => serde_json::json!({ "basDd": date, "isinCd": code }),
        None => serde_json::json!({ "basDd": date }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_stock_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR7005930003",
            "ISU_NM": "삼성전자보통주",
            "MKT_NM": "KOSPI",
            "SECT_TP_NM": "전기전자",
            "TDD_CLSPRC": "80,000",
            "CMPPREVDD_PRC": "1,000",
            "FLUC_RT": "1.27",
            "TDD_OPNPRC": "79,500",
            "TDD_HGPRC": "80,500",
            "TDD_LWPRC": "79,000",
            "ACC_TRDVOL": "10,000,000",
            "ACC_TRDVAL": "800,000,000,000",
            "MKTCAP": "477,000,000,000,000",
            "LIST_SHRS": "5,969,782,550"
        });

        let record: StockRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR7005930003");
        assert_eq!(record.isu_nm, "삼성전자보통주");
        assert_eq!(record.tdd_clsprc, "80,000");
        assert_eq!(record.mktcap, "477,000,000,000,000");
    }

    #[test]
    fn test_deserialize_stock_info_record() {
        let json = serde_json::json!({
            "ISU_CD": "KR7005930003",
            "ISU_SRT_CD": "005930",
            "ISU_NM": "삼성전자보통주",
            "ISU_ABBRV": "삼성전자",
            "ISU_ENG_NM": "SamsungElectronics",
            "LIST_DD": "19750611",
            "MKT_TP_NM": "KOSPI",
            "SECUGRP_NM": "주권",
            "SECT_TP_NM": "전기전자",
            "KIND_STKCERT_TP_NM": "보통주",
            "PARVAL": "100",
            "LIST_SHRS": "5,969,782,550"
        });

        let record: StockInfoRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.isu_cd, "KR7005930003");
        assert_eq!(record.isu_srt_cd, "005930");
        assert_eq!(record.isu_nm, "삼성전자보통주");
        assert_eq!(record.isu_eng_nm, "SamsungElectronics");
        assert_eq!(record.list_dd, "19750611");
        assert_eq!(record.parval, "100");
    }

    #[test]
    fn test_build_params_without_isin() {
        let params = build_params("20250301", None);
        assert_eq!(params, serde_json::json!({ "basDd": "20250301" }));
    }

    #[test]
    fn test_build_params_with_isin() {
        let params = build_params("20250301", Some("KR7005930003"));
        assert_eq!(
            params,
            serde_json::json!({ "basDd": "20250301", "isinCd": "KR7005930003" })
        );
    }
}
