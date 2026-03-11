//! Index endpoints (KRX/KOSPI/KOSDAQ/Derivatives indices).
//!
//! Covers 4 endpoints:
//! - KRX index daily (`/idx/krx_dd_trd`)
//! - KOSPI index daily (`/idx/kospi_dd_trd`)
//! - KOSDAQ index daily (`/idx/kosdaq_dd_trd`)
//! - Derivatives index daily (`/idx/drvprod_dd_trd`)

use serde::{Deserialize, Serialize};

use crate::client::KrxClient;
use crate::error::KrxError;

/// Index daily trading record returned by KRX API.
///
/// Fields `acc_trdvol`, `acc_trdval`, and `mktcap` are `Option` because
/// the derivatives index endpoint does not include them.
#[derive(Debug, Deserialize, Serialize)]
pub struct IndexRecord {
    /// Base date (YYYYMMDD).
    #[serde(rename = "BAS_DD")]
    pub bas_dd: String,

    /// Index classification.
    #[serde(rename = "IDX_CLSS")]
    pub idx_clss: String,

    /// Index name.
    #[serde(rename = "IDX_NM")]
    pub idx_nm: String,

    /// Closing price index.
    #[serde(rename = "CLSPRC_IDX")]
    pub clsprc_idx: String,

    /// Change compared to previous day.
    #[serde(rename = "CMPPREVDD_IDX")]
    pub cmpprevdd_idx: String,

    /// Fluctuation rate (%).
    #[serde(rename = "FLUC_RT")]
    pub fluc_rt: String,

    /// Opening price index.
    #[serde(rename = "OPNPRC_IDX")]
    pub opnprc_idx: String,

    /// High price index.
    #[serde(rename = "HGPRC_IDX")]
    pub hgprc_idx: String,

    /// Low price index.
    #[serde(rename = "LWPRC_IDX")]
    pub lwprc_idx: String,

    /// Accumulated trading volume.
    #[serde(rename = "ACC_TRDVOL", default)]
    pub acc_trdvol: Option<String>,

    /// Accumulated trading value.
    #[serde(rename = "ACC_TRDVAL", default)]
    pub acc_trdval: Option<String>,

    /// Market capitalization.
    #[serde(rename = "MKTCAP", default)]
    pub mktcap: Option<String>,
}

/// Fetches KRX composite index daily data.
pub async fn fetch_krx_index(client: &KrxClient, date: &str) -> Result<Vec<IndexRecord>, KrxError> {
    fetch_index(client, "/idx/krx_dd_trd", date).await
}

/// Fetches KOSPI index daily data.
pub async fn fetch_kospi_index(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<IndexRecord>, KrxError> {
    fetch_index(client, "/idx/kospi_dd_trd", date).await
}

/// Fetches KOSDAQ index daily data.
pub async fn fetch_kosdaq_index(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<IndexRecord>, KrxError> {
    fetch_index(client, "/idx/kosdaq_dd_trd", date).await
}

/// Fetches derivatives index daily data.
pub async fn fetch_derivatives_index(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<IndexRecord>, KrxError> {
    fetch_index(client, "/idx/drvprod_dd_trd", date).await
}

/// Internal helper: calls the given index endpoint path with the given date.
async fn fetch_index(
    client: &KrxClient,
    path: &str,
    date: &str,
) -> Result<Vec<IndexRecord>, KrxError> {
    let params = serde_json::json!({ "basDd": date });
    let raw = client.post(path, params).await?;

    let records: Vec<IndexRecord> = raw
        .into_iter()
        .map(|v| serde_json::from_value(v).map_err(|e| KrxError::ParseError(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_full_index_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "IDX_CLSS": "KRX",
            "IDX_NM": "KRX 300",
            "CLSPRC_IDX": "1,234.56",
            "CMPPREVDD_IDX": "12.34",
            "FLUC_RT": "1.01",
            "OPNPRC_IDX": "1,222.00",
            "HGPRC_IDX": "1,240.00",
            "LWPRC_IDX": "1,220.00",
            "ACC_TRDVOL": "1,000,000",
            "ACC_TRDVAL": "500,000,000",
            "MKTCAP": "1,000,000,000,000"
        });

        let record: IndexRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.idx_nm, "KRX 300");
        assert_eq!(record.acc_trdvol, Some("1,000,000".to_string()));
        assert_eq!(record.mktcap, Some("1,000,000,000,000".to_string()));
    }

    #[test]
    fn test_deserialize_derivatives_index_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "IDX_CLSS": "파생",
            "IDX_NM": "KOSPI 200 선물지수",
            "CLSPRC_IDX": "350.00",
            "CMPPREVDD_IDX": "-2.50",
            "FLUC_RT": "-0.71",
            "OPNPRC_IDX": "352.00",
            "HGPRC_IDX": "353.00",
            "LWPRC_IDX": "349.00"
        });

        let record: IndexRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.idx_nm, "KOSPI 200 선물지수");
        assert!(record.acc_trdvol.is_none());
        assert!(record.acc_trdval.is_none());
        assert!(record.mktcap.is_none());
    }
}
