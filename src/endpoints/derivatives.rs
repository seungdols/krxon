//! Derivatives endpoints for futures and options.
//!
//! Covers 6 endpoints:
//! - Futures daily (`/drv/fut_bydd_trd`)
//! - KOSPI stock futures daily (`/drv/eqsfu_stk_bydd_trd`)
//! - KOSDAQ stock futures daily (`/drv/eqkfu_ksq_bydd_trd`)
//! - Options daily (`/drv/opt_bydd_trd`)
//! - KOSPI stock options daily (`/drv/eqsop_bydd_trd`)
//! - KOSDAQ stock options daily (`/drv/eqkop_bydd_trd`)

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::client::KrxClient;
use crate::error::KrxError;

/// Futures daily trading record returned by KRX API.
///
/// Used for `futures`, `stock-futures-kospi`, and `stock-futures-kosdaq` endpoints.
/// Fields `tdd_opnprc`, `tdd_hgprc`, and `tdd_lwprc` are `Option` because
/// the stock futures endpoints do not include them.
#[derive(Debug, Deserialize, Serialize)]
pub struct FuturesRecord {
    /// Base date (YYYYMMDD).
    #[serde(rename = "BAS_DD")]
    pub bas_dd: String,

    /// Issue code.
    #[serde(rename = "ISU_CD")]
    pub isu_cd: String,

    /// Issue name.
    #[serde(rename = "ISU_NM")]
    pub isu_nm: String,

    /// Product name.
    #[serde(rename = "PROD_NM")]
    pub prod_nm: String,

    /// Market name.
    #[serde(rename = "MKT_NM")]
    pub mkt_nm: String,

    /// Closing price.
    #[serde(rename = "TDD_CLSPRC")]
    pub tdd_clsprc: String,

    /// Opening price (futures daily only).
    #[serde(rename = "TDD_OPNPRC", default)]
    pub tdd_opnprc: Option<String>,

    /// High price (futures daily only).
    #[serde(rename = "TDD_HGPRC", default)]
    pub tdd_hgprc: Option<String>,

    /// Low price (futures daily only).
    #[serde(rename = "TDD_LWPRC", default)]
    pub tdd_lwprc: Option<String>,

    /// Settlement price.
    #[serde(rename = "SETL_PRC")]
    pub setl_prc: String,

    /// Spot price.
    #[serde(rename = "SPOT_PRC")]
    pub spot_prc: String,

    /// Change compared to previous day.
    #[serde(rename = "CMPPREVDD_PRC")]
    pub cmpprevdd_prc: String,

    /// Accumulated trading volume.
    #[serde(rename = "ACC_TRDVOL")]
    pub acc_trdvol: String,

    /// Accumulated trading value.
    #[serde(rename = "ACC_TRDVAL")]
    pub acc_trdval: String,

    /// Open interest quantity.
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub acc_opnint_qty: String,
}

/// Options daily trading record returned by KRX API.
///
/// Used for `options`, `stock-options-kospi`, and `stock-options-kosdaq` endpoints.
/// All three endpoints share the same response field structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionsRecord {
    /// Base date (YYYYMMDD).
    #[serde(rename = "BAS_DD")]
    pub bas_dd: String,

    /// Issue code.
    #[serde(rename = "ISU_CD")]
    pub isu_cd: String,

    /// Issue name.
    #[serde(rename = "ISU_NM")]
    pub isu_nm: String,

    /// Product name.
    #[serde(rename = "PROD_NM")]
    pub prod_nm: String,

    /// Right type (Call/Put).
    #[serde(rename = "RGHT_TP_NM")]
    pub rght_tp_nm: String,

    /// Closing price.
    #[serde(rename = "TDD_CLSPRC")]
    pub tdd_clsprc: String,

    /// Opening price.
    #[serde(rename = "TDD_OPNPRC")]
    pub tdd_opnprc: String,

    /// High price.
    #[serde(rename = "TDD_HGPRC")]
    pub tdd_hgprc: String,

    /// Low price.
    #[serde(rename = "TDD_LWPRC")]
    pub tdd_lwprc: String,

    /// Change compared to previous day.
    #[serde(rename = "CMPPREVDD_PRC")]
    pub cmpprevdd_prc: String,

    /// Accumulated trading volume.
    #[serde(rename = "ACC_TRDVOL")]
    pub acc_trdvol: String,

    /// Accumulated trading value.
    #[serde(rename = "ACC_TRDVAL")]
    pub acc_trdval: String,

    /// Open interest quantity.
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub acc_opnint_qty: String,

    /// Implied volatility.
    #[serde(rename = "IMP_VOLT")]
    pub imp_volt: String,

    /// Next day base price.
    #[serde(rename = "NXTDD_BAS_PRC")]
    pub nxtdd_bas_prc: String,
}

/// Fetches futures daily trading data.
pub async fn fetch_futures_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<FuturesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/fut_bydd_trd", date).await
}

/// Fetches KOSPI stock futures daily trading data.
pub async fn fetch_stock_futures_kospi_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<FuturesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/eqsfu_stk_bydd_trd", date).await
}

/// Fetches KOSDAQ stock futures daily trading data.
pub async fn fetch_stock_futures_kosdaq_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<FuturesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/eqkfu_ksq_bydd_trd", date).await
}

/// Fetches options daily trading data.
pub async fn fetch_options_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<OptionsRecord>, KrxError> {
    fetch_derivatives(client, "/drv/opt_bydd_trd", date).await
}

/// Fetches KOSPI stock options daily trading data.
pub async fn fetch_stock_options_kospi_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<OptionsRecord>, KrxError> {
    fetch_derivatives(client, "/drv/eqsop_bydd_trd", date).await
}

/// Fetches KOSDAQ stock options daily trading data.
pub async fn fetch_stock_options_kosdaq_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<OptionsRecord>, KrxError> {
    fetch_derivatives(client, "/drv/eqkop_bydd_trd", date).await
}

/// Internal helper: calls the given derivatives endpoint path with the given date.
async fn fetch_derivatives<T: DeserializeOwned>(
    client: &KrxClient,
    path: &str,
    date: &str,
) -> Result<Vec<T>, KrxError> {
    let params = serde_json::json!({ "basDd": date });
    let raw = client.post(path, params).await?;

    let records: Vec<T> = raw
        .into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| KrxError::ParseError(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_futures_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4101V30009",
            "ISU_NM": "KOSPI200 F 2503",
            "PROD_NM": "코스피200선물",
            "MKT_NM": "파생",
            "TDD_CLSPRC": "350.00",
            "TDD_OPNPRC": "348.50",
            "TDD_HGPRC": "351.00",
            "TDD_LWPRC": "347.00",
            "SETL_PRC": "350.00",
            "SPOT_PRC": "349.80",
            "CMPPREVDD_PRC": "2.50",
            "ACC_TRDVOL": "150,000",
            "ACC_TRDVAL": "52,500,000,000",
            "ACC_OPNINT_QTY": "250,000"
        });

        let record: FuturesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR4101V30009");
        assert_eq!(record.isu_nm, "KOSPI200 F 2503");
        assert_eq!(record.prod_nm, "코스피200선물");
        assert_eq!(record.tdd_clsprc, "350.00");
        assert_eq!(record.tdd_opnprc, Some("348.50".to_string()));
        assert_eq!(record.tdd_hgprc, Some("351.00".to_string()));
        assert_eq!(record.tdd_lwprc, Some("347.00".to_string()));
        assert_eq!(record.setl_prc, "350.00");
        assert_eq!(record.spot_prc, "349.80");
        assert_eq!(record.acc_opnint_qty, "250,000");
    }

    #[test]
    fn test_deserialize_stock_futures_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4005930008",
            "ISU_NM": "삼성전자 F 2503",
            "PROD_NM": "삼성전자선물",
            "MKT_NM": "주식선물",
            "TDD_CLSPRC": "55,000",
            "SETL_PRC": "55,000",
            "SPOT_PRC": "54,800",
            "CMPPREVDD_PRC": "500",
            "ACC_TRDVOL": "10,000",
            "ACC_TRDVAL": "550,000,000",
            "ACC_OPNINT_QTY": "20,000"
        });

        let record: FuturesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR4005930008");
        assert_eq!(record.isu_nm, "삼성전자 F 2503");
        assert!(record.tdd_opnprc.is_none());
        assert!(record.tdd_hgprc.is_none());
        assert!(record.tdd_lwprc.is_none());
        assert_eq!(record.setl_prc, "55,000");
        assert_eq!(record.acc_opnint_qty, "20,000");
    }

    #[test]
    fn test_deserialize_options_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4201V3A002",
            "ISU_NM": "KOSPI200 콜옵션 2503 350",
            "PROD_NM": "코스피200옵션",
            "RGHT_TP_NM": "콜",
            "TDD_CLSPRC": "5.50",
            "TDD_OPNPRC": "5.00",
            "TDD_HGPRC": "6.00",
            "TDD_LWPRC": "4.80",
            "CMPPREVDD_PRC": "0.50",
            "ACC_TRDVOL": "50,000",
            "ACC_TRDVAL": "2,750,000,000",
            "ACC_OPNINT_QTY": "100,000",
            "IMP_VOLT": "15.50",
            "NXTDD_BAS_PRC": "5.60"
        });

        let record: OptionsRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR4201V3A002");
        assert_eq!(record.isu_nm, "KOSPI200 콜옵션 2503 350");
        assert_eq!(record.rght_tp_nm, "콜");
        assert_eq!(record.tdd_clsprc, "5.50");
        assert_eq!(record.imp_volt, "15.50");
        assert_eq!(record.nxtdd_bas_prc, "5.60");
        assert_eq!(record.acc_opnint_qty, "100,000");
    }
}
