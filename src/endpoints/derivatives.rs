//! Derivatives endpoints for futures and options.
//!
//! Covers 6 endpoints:
//! - Futures daily (`/drv/fut_bydd_trd`)
//! - KOSPI stock futures daily (`/drv/stk_fut_bydd_trd`)
//! - KOSDAQ stock futures daily (`/drv/ksq_fut_bydd_trd`)
//! - Options daily (`/drv/opt_bydd_trd`)
//! - KOSPI stock options daily (`/drv/stk_opt_bydd_trd`)
//! - KOSDAQ stock options daily (`/drv/ksq_opt_bydd_trd`)

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::client::KrxClient;
use crate::error::KrxError;

/// Futures daily trading record returned by KRX API.
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

    /// Opening price.
    #[serde(rename = "TDD_OPNPRC")]
    pub tdd_opnprc: String,

    /// High price.
    #[serde(rename = "TDD_HGPRC")]
    pub tdd_hgprc: String,

    /// Low price.
    #[serde(rename = "TDD_LWPRC")]
    pub tdd_lwprc: String,

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

    /// Accumulated open interest quantity.
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub acc_opnint_qty: String,
}

/// Stock futures daily trading record (KOSPI/KOSDAQ).
#[derive(Debug, Deserialize, Serialize)]
pub struct StockFuturesRecord {
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

    /// Accumulated open interest quantity.
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub acc_opnint_qty: String,
}

/// Options daily trading record (options, stock options KOSPI/KOSDAQ).
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

    /// Accumulated open interest quantity.
    #[serde(rename = "ACC_OPNINT_QTY")]
    pub acc_opnint_qty: String,

    /// Implied volatility.
    #[serde(rename = "IMP_VOLT")]
    pub imp_volt: String,

    /// Next day base price.
    #[serde(rename = "NXTDD_BAS_PRC")]
    pub nxtdd_bas_prc: String,
}

// ---------------------------------------------------------------------------
// Public fetch functions
// ---------------------------------------------------------------------------

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
) -> Result<Vec<StockFuturesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/stk_fut_bydd_trd", date).await
}

/// Fetches KOSDAQ stock futures daily trading data.
pub async fn fetch_stock_futures_kosdaq_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<StockFuturesRecord>, KrxError> {
    fetch_derivatives(client, "/drv/ksq_fut_bydd_trd", date).await
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
    fetch_derivatives(client, "/drv/stk_opt_bydd_trd", date).await
}

/// Fetches KOSDAQ stock options daily trading data.
pub async fn fetch_stock_options_kosdaq_daily(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<OptionsRecord>, KrxError> {
    fetch_derivatives(client, "/drv/ksq_opt_bydd_trd", date).await
}

// ---------------------------------------------------------------------------
// Internal helper
// ---------------------------------------------------------------------------

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
            "TDD_CLSPRC": "350.25",
            "TDD_OPNPRC": "348.00",
            "TDD_HGPRC": "351.50",
            "TDD_LWPRC": "347.80",
            "SETL_PRC": "350.20",
            "SPOT_PRC": "350.00",
            "CMPPREVDD_PRC": "2.25",
            "ACC_TRDVOL": "150,000",
            "ACC_TRDVAL": "52,500,000,000",
            "ACC_OPNINT_QTY": "280,000"
        });

        let record: FuturesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_nm, "KOSPI200 F 2503");
        assert_eq!(record.tdd_clsprc, "350.25");
        assert_eq!(record.tdd_opnprc, "348.00");
        assert_eq!(record.setl_prc, "350.20");
        assert_eq!(record.acc_opnint_qty, "280,000");
    }

    #[test]
    fn test_deserialize_stock_futures_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4005930009",
            "ISU_NM": "삼성전자 F 2503",
            "PROD_NM": "주식선물",
            "MKT_NM": "파생",
            "TDD_CLSPRC": "72,000",
            "SETL_PRC": "71,950",
            "SPOT_PRC": "72,100",
            "CMPPREVDD_PRC": "-500",
            "ACC_TRDVOL": "5,000",
            "ACC_TRDVAL": "360,000,000",
            "ACC_OPNINT_QTY": "12,000"
        });

        let record: StockFuturesRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_nm, "삼성전자 F 2503");
        assert_eq!(record.tdd_clsprc, "72,000");
        assert_eq!(record.setl_prc, "71,950");
    }

    #[test]
    fn test_deserialize_options_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR4201V3B002",
            "ISU_NM": "KOSPI200 C 2503 350",
            "PROD_NM": "코스피200옵션",
            "RGHT_TP_NM": "콜",
            "TDD_CLSPRC": "5.50",
            "TDD_OPNPRC": "5.00",
            "TDD_HGPRC": "6.00",
            "TDD_LWPRC": "4.80",
            "CMPPREVDD_PRC": "0.50",
            "ACC_TRDVOL": "20,000",
            "ACC_TRDVAL": "1,100,000,000",
            "ACC_OPNINT_QTY": "50,000",
            "IMP_VOLT": "18.50",
            "NXTDD_BAS_PRC": "5.55"
        });

        let record: OptionsRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_nm, "KOSPI200 C 2503 350");
        assert_eq!(record.rght_tp_nm, "콜");
        assert_eq!(record.tdd_clsprc, "5.50");
        assert_eq!(record.imp_volt, "18.50");
        assert_eq!(record.nxtdd_bas_prc, "5.55");
    }
}
