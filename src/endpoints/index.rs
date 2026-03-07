//! Index endpoints (KRX/KOSPI/KOSDAQ/Derivatives indices).
//!
//! Covers 4 endpoints:
//! - KRX index daily (`/idx/krx_dd_trd`)
//! - KOSPI index daily (`/idx/kospi_dd_trd`)
//! - KOSDAQ index daily (`/idx/ksdaq_dd_trd`)
//! - Derivatives index daily (`/idx/drv_dd_trd`)

use serde_json::{json, Value};

use crate::client::KrxClient;
use crate::error::KrxError;

/// Fetches KRX index daily data (`/idx/krx_dd_trd`).
pub async fn fetch_krx_index(client: &KrxClient, date: &str) -> Result<Vec<Value>, KrxError> {
    client.post("/idx/krx_dd_trd", json!({"basDd": date})).await
}

/// Fetches KOSPI index daily data (`/idx/kospi_dd_trd`).
pub async fn fetch_kospi_index(client: &KrxClient, date: &str) -> Result<Vec<Value>, KrxError> {
    client
        .post("/idx/kospi_dd_trd", json!({"basDd": date}))
        .await
}

/// Fetches KOSDAQ index daily data (`/idx/ksdaq_dd_trd`).
pub async fn fetch_kosdaq_index(client: &KrxClient, date: &str) -> Result<Vec<Value>, KrxError> {
    client
        .post("/idx/ksdaq_dd_trd", json!({"basDd": date}))
        .await
}

/// Fetches derivatives index daily data (`/idx/drv_dd_trd`).
pub async fn fetch_derivatives_index(
    client: &KrxClient,
    date: &str,
) -> Result<Vec<Value>, KrxError> {
    client
        .post("/idx/drv_dd_trd", json!({"basDd": date}))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    fn test_client(server: &Server, api_key: &str) -> KrxClient {
        KrxClient::with_base_url(api_key, &server.url()).expect("failed to create test client")
    }

    #[tokio::test]
    async fn test_fetch_krx_index_sends_correct_request() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/krx_dd_trd")
            .match_header("AUTH_KEY", "test_key")
            .match_body(mockito::Matcher::Json(json!({"basDd": "20250301"})))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "OutBlock_1": [
                        {"BAS_DD": "20250301", "IDX_NM": "KRX 300"}
                    ]
                })
                .to_string(),
            )
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = fetch_krx_index(&client, "20250301").await;

        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0]["IDX_NM"], "KRX 300");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fetch_kospi_index_sends_correct_path() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/kospi_dd_trd")
            .match_body(mockito::Matcher::Json(json!({"basDd": "20250301"})))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"OutBlock_1": []}).to_string())
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = fetch_kospi_index(&client, "20250301").await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fetch_kosdaq_index_sends_correct_path() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/ksdaq_dd_trd")
            .match_body(mockito::Matcher::Json(json!({"basDd": "20250301"})))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"OutBlock_1": []}).to_string())
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = fetch_kosdaq_index(&client, "20250301").await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fetch_derivatives_index_sends_correct_path() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/idx/drv_dd_trd")
            .match_body(mockito::Matcher::Json(json!({"basDd": "20250301"})))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({"OutBlock_1": []}).to_string())
            .create_async()
            .await;

        let client = test_client(&server, "test_key");
        let result = fetch_derivatives_index(&client, "20250301").await;

        assert!(result.is_ok());
        mock.assert_async().await;
    }
}
