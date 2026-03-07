//! ETP (Exchange Traded Products) endpoints for ETF and ETN.
//!
//! Covers 2 endpoints:
//! - ETF daily (`/etp/etf_bydd_trd`)
//! - ETN daily (`/etp/etn_bydd_trd`)

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::client::KrxClient;
use crate::error::KrxError;

/// ETF daily trading record returned by KRX API.
#[derive(Debug, Deserialize, Serialize)]
pub struct EtfRecord {
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

    /// Net Asset Value.
    #[serde(rename = "NAV")]
    pub nav: String,

    /// Underlying index name.
    #[serde(rename = "IDX_IND_NM")]
    pub idx_ind_nm: String,

    /// Objective stock price index.
    #[serde(rename = "OBJ_STKPRC_IDX")]
    pub obj_stkprc_idx: String,

    /// Index change compared to previous day.
    #[serde(rename = "CMPPREVDD_IDX")]
    pub cmpprevdd_idx: String,

    /// Index fluctuation rate.
    #[serde(rename = "FLUC_RT_IDX")]
    pub fluc_rt_idx: String,

    /// Total net asset amount of investment assets.
    #[serde(rename = "INVSTASST_NETASST_TOTAMT")]
    pub invstasst_netasst_totamt: String,
}

/// ETN daily trading record returned by KRX API.
#[derive(Debug, Deserialize, Serialize)]
pub struct EtnRecord {
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

    /// Underlying index name.
    #[serde(rename = "IDX_IND_NM")]
    pub idx_ind_nm: String,

    /// Objective stock price index.
    #[serde(rename = "OBJ_STKPRC_IDX")]
    pub obj_stkprc_idx: String,

    /// Index change compared to previous day.
    #[serde(rename = "CMPPREVDD_IDX")]
    pub cmpprevdd_idx: String,

    /// Index fluctuation rate.
    #[serde(rename = "FLUC_RT_IDX")]
    pub fluc_rt_idx: String,

    /// Indicative value amount.
    #[serde(rename = "INDIC_VAL_AMT")]
    pub indic_val_amt: String,

    /// Indicative value per 1 security.
    #[serde(rename = "PER1SECU_INDIC_VAL")]
    pub per1secu_indic_val: String,
}

/// Fetches ETF daily trading data.
pub async fn fetch_etf_daily(client: &KrxClient, date: &str) -> Result<Vec<EtfRecord>, KrxError> {
    fetch_etp(client, "/etp/etf_bydd_trd", date).await
}

/// Fetches ETN daily trading data.
pub async fn fetch_etn_daily(client: &KrxClient, date: &str) -> Result<Vec<EtnRecord>, KrxError> {
    fetch_etp(client, "/etp/etn_bydd_trd", date).await
}

/// Internal helper: calls the given ETP endpoint path with the given date.
async fn fetch_etp<T: DeserializeOwned>(
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
    fn test_deserialize_etf_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR7069500007",
            "ISU_NM": "KODEX 200",
            "TDD_CLSPRC": "35,000",
            "CMPPREVDD_PRC": "500",
            "FLUC_RT": "1.45",
            "TDD_OPNPRC": "34,600",
            "TDD_HGPRC": "35,200",
            "TDD_LWPRC": "34,500",
            "ACC_TRDVOL": "5,000,000",
            "ACC_TRDVAL": "175,000,000,000",
            "MKTCAP": "10,000,000,000,000",
            "LIST_SHRS": "285,710,000",
            "NAV": "35,012.34",
            "IDX_IND_NM": "KOSPI 200",
            "OBJ_STKPRC_IDX": "350.12",
            "CMPPREVDD_IDX": "5.00",
            "FLUC_RT_IDX": "1.45",
            "INVSTASST_NETASST_TOTAMT": "10,003,525,000,000"
        });

        let record: EtfRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR7069500007");
        assert_eq!(record.isu_nm, "KODEX 200");
        assert_eq!(record.nav, "35,012.34");
        assert_eq!(record.invstasst_netasst_totamt, "10,003,525,000,000");
    }

    #[test]
    fn test_deserialize_etn_record() {
        let json = serde_json::json!({
            "BAS_DD": "20250301",
            "ISU_CD": "KR6500003AC5",
            "ISU_NM": "TRUE ETN 레버리지",
            "TDD_CLSPRC": "12,500",
            "CMPPREVDD_PRC": "-200",
            "FLUC_RT": "-1.57",
            "TDD_OPNPRC": "12,700",
            "TDD_HGPRC": "12,800",
            "TDD_LWPRC": "12,400",
            "ACC_TRDVOL": "100,000",
            "ACC_TRDVAL": "1,250,000,000",
            "MKTCAP": "500,000,000,000",
            "LIST_SHRS": "40,000,000",
            "IDX_IND_NM": "KOSPI 200",
            "OBJ_STKPRC_IDX": "350.12",
            "CMPPREVDD_IDX": "-5.00",
            "FLUC_RT_IDX": "-1.41",
            "INDIC_VAL_AMT": "12,520",
            "PER1SECU_INDIC_VAL": "12,520"
        });

        let record: EtnRecord = serde_json::from_value(json).unwrap();
        assert_eq!(record.bas_dd, "20250301");
        assert_eq!(record.isu_cd, "KR6500003AC5");
        assert_eq!(record.isu_nm, "TRUE ETN 레버리지");
        assert_eq!(record.indic_val_amt, "12,520");
        assert_eq!(record.per1secu_indic_val, "12,520");
    }
}
