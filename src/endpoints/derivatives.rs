//! Derivatives endpoints for futures and options.
//!
//! Covers 6 endpoints:
//! - Futures daily (`/drv/fut_bydd_trd`)
//! - KOSPI stock futures daily (`/drv/stk_fut_bydd_trd`)
//! - KOSDAQ stock futures daily (`/drv/ksq_fut_bydd_trd`)
//! - Options daily (`/drv/opt_bydd_trd`)
//! - KOSPI stock options daily (`/drv/stk_opt_bydd_trd`)
//! - KOSDAQ stock options daily (`/drv/ksq_opt_bydd_trd`)

use serde::{Deserialize, Serialize};

use crate::client::KrxClient;
use crate::error::KrxError;

/// Derivatives daily trading record returned by KRX API.
///
/// Used by all 6 derivatives endpoints (futures and options).
/// All fields are strings as returned by the KRX API.
#[derive(Debug, Deserialize, Serialize)]
pub struct DerivativesRecord {
    /// Base date (YYYYMMDD).
    #[serde(rename = "BAS_DD")]
    pub bas_dd: String,

    /// Issue code.
    #[serde(rename = "ISU_CD")]
    pub isu_cd: String,

    /// Issue name.
    #[serde(rename = "ISU_NM")]
    pub isu_nm: String,

    /// Closing price.
    #[serde(rename = "TDD_CLSPRC")]
    pub tdd_clsprc: String,

    /// Change compared to previous day.
    #[serde(rename = "CMPPREVDD_PRC")]
    pub cmpprevdd_prc: String,

    /// Fluctuation rate (%).
    #[serde(rename = "FLUC_RT", default)]
    pub fluc_rt: Option<String>,

    /// Opening price.
    #[serde(rename = "TDD_OPNPRC", default)]
    pub tdd_opnprc: Option<String>,

    /// High price.
    #[serde(rename = "TDD_HGPRC", default)]
    pub tdd_hgprc: Option<String>,

    /// Low price.
    #[serde(rename = "TDD_LWPRC", default)]
    pub tdd_lwprc: Option<String>,

    /// Accumulated trading volume.
    #[serde(rename = "ACC_TRDVOL")]
    pub acc_trdvol: String,

    /// Accumulated trading value.
    #[serde(rename = "ACC_TRDVAL")]
    pub acc_trdval: String,

    /// Accumulated open interest quantity.
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub acc_opnint_qty: String,
}

/// Fetches futures daily trading data.
pub async fn fetch_futures(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/fut_bydd_trd", date).await
}

/// Fetches KOSPI stock futures daily trading data.
pub async fn fetch_stock_futures_kospi(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/stk_fut_bydd_trd", date).await
}

/// Fetches KOSDAQ stock futures daily trading data.
pub async fn fetch_stock_futures_kosdaq(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/ksq_fut_bydd_trd", date).await
}

/// Fetches options daily trading data.
pub async fn fetch_options(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/opt_bydd_trd", date).await
}

/// Fetches KOSPI stock options daily trading data.
pub async fn fetch_stock_options_kospi(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/stk_opt_bydd_trd", date).await
}

/// Fetches KOSDAQ stock options daily trading data.
pub async fn fetch_stock_options_kosdaq(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/ksq_opt_bydd_trd", date).await
}

/// Internal helper: calls the given derivatives endpoint path with the given date.
async fn fetch_derivatives(
    client: &KrxClient,
    path: &str,
    date: &str,
) -> Result<Vec<DerivativesRecord>, KrxError> {
    let params = serde_json::json!({ "basDd": date });
    let raw = client.post(path, params).await?;

    let records: Vec<DerivativesRecord> = raw
        .into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| KrxError::ParseError(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_derivatives_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4101V30009",
            "ISU_NM": "KOSPI200 F 202503",
            "TDD_CLSPRC": "350.00",
            "CMPPREVDD_PRC": "2.50",
            "FLUC_RT": "0.72",
            "TDD_OPNPRC": "348.00",
            "TDD_HGPRC": "351.00",
            "TDD_LWPRC": "347.50",
            "ACC_TRDVOL": "250,000",
            "ACC_TRDVAL": "87,500,000,000",
            "ACC_OPNINT_QTY": "150,000"
        });

        let record: DerivativesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR4101V30009");
        assert_eq!(record.isu_nm, "KOSPI200 F 202503");
        assert_eq!(record.tdd_clsprc, "350.00");
        assert_eq!(record.cmpprevdd_prc, "2.50");
        assert_eq!(record.fluc_rt, Some("0.72".to_string()));
        assert_eq!(record.tdd_opnprc, Some("348.00".to_string()));
        assert_eq!(record.acc_trdvol, "250,000");
        assert_eq!(record.acc_opnint_qty, "150,000");
    }

    #[test]
    fn test_deserialize_without_optional_fields() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4101V30009",
            "ISU_NM": "삼성전자 선물 202503",
            "TDD_CLSPRC": "80,000",
            "CMPPREVDD_PRC": "1,000",
            "ACC_TRDVOL": "5,000",
            "ACC_TRDVAL": "400,000,000",
            "ACC_OPNINT_QTY": "3,000"
        });

        let record: DerivativesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert!(record.fluc_rt.is_none());
        assert!(record.tdd_opnprc.is_none());
        assert!(record.tdd_hgprc.is_none());
        assert!(record.tdd_lwprc.is_none());
        assert_eq!(record.acc_opnint_qty, "3,000");
    }

    #[test]
    fn test_deserialize_options_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4201V3A272",
            "ISU_NM": "KOSPI200 C 202503 350",
            "TDD_CLSPRC": "5.50",
            "CMPPREVDD_PRC": "-0.30",
            "FLUC_RT": "-5.17",
            "TDD_OPNPRC": "5.80",
            "TDD_HGPRC": "6.00",
            "TDD_LWPRC": "5.20",
            "ACC_TRDVOL": "10,000",
            "ACC_TRDVAL": "550,000,000",
            "ACC_OPNINT_QTY": "25,000"
        });

        let record: DerivativesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_nm, "KOSPI200 C 202503 350");
        assert_eq!(record.tdd_clsprc, "5.50");
        assert_eq!(record.acc_opnint_qty, "25,000");
    }
}
