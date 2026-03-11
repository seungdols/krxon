//! krxon - CLI tool for KRX (Korea Exchange) Open API.

#![allow(dead_code)]

mod cli;
mod client;
mod codegen;
mod endpoints;
mod error;
mod output;
mod utils;

use clap::Parser;

use cli::{
    Cli, Commands, DerivativesSubcommand, EtpSubcommand, FetchResource, GenerateLanguage,
    IndexSubcommand, StockSubcommand,
};
use client::{resolve_api_key, KrxClient};
use endpoints::derivatives::{
    fetch_futures_daily, fetch_options_daily, fetch_stock_futures_kosdaq_daily,
    fetch_stock_futures_kospi_daily, fetch_stock_options_kosdaq_daily,
    fetch_stock_options_kospi_daily,
};
use endpoints::etp::{fetch_etf_daily, fetch_etn_daily};
use endpoints::index::{
    fetch_derivatives_index, fetch_kosdaq_index, fetch_kospi_index, fetch_krx_index,
};
use endpoints::stock::{
    fetch_kosdaq_stock, fetch_kosdaq_stock_info, fetch_kospi_stock, fetch_kospi_stock_info,
};
use output::format_records_table;

/// Prints a formatted table, or `"No data found."` if the table is empty.
fn print_table<T, F>(headers: &[&str], records: &[T], to_row: F)
where
    F: Fn(&T) -> Vec<String>,
{
    let table = format_records_table(headers, records, to_row);
    if table.is_empty() {
        println!("No data found.");
    } else {
        println!("{}", table);
    }
}

/// Application entry point.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            handle_init(&args.key)?;
        }
        Commands::Clean => {
            handle_clean()?;
        }
        Commands::Fetch { resource } | Commands::FetchShortcut(resource) => match resource {
            FetchResource::Index { subcommand } => {
                handle_fetch_index(subcommand).await?;
            }
            FetchResource::Stock { subcommand } => {
                handle_fetch_stock(subcommand).await?;
            }
            FetchResource::Etp { subcommand } => {
                handle_fetch_etp(subcommand).await?;
            }
            FetchResource::Derivatives { subcommand } => {
                handle_fetch_derivatives(subcommand).await?;
            }
        },
        Commands::Generate { language } => match language {
            GenerateLanguage::Python(args) => {
                codegen::python::generate(&args.out)?;
            }
            GenerateLanguage::Typescript(args) => {
                codegen::typescript::generate(&args.out)?;
            }
        },
    }

    Ok(())
}

/// Handles `fetch index <subcommand>`.
async fn handle_fetch_index(subcommand: IndexSubcommand) -> anyhow::Result<()> {
    let args = match &subcommand {
        IndexSubcommand::Krx(args)
        | IndexSubcommand::Kospi(args)
        | IndexSubcommand::Kosdaq(args)
        | IndexSubcommand::Derivatives(args) => args,
    };

    utils::validate_date(&args.date)?;

    let api_key = resolve_api_key(args.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    let records = match &subcommand {
        IndexSubcommand::Krx(_) => fetch_krx_index(&client, &args.date).await?,
        IndexSubcommand::Kospi(_) => fetch_kospi_index(&client, &args.date).await?,
        IndexSubcommand::Kosdaq(_) => fetch_kosdaq_index(&client, &args.date).await?,
        IndexSubcommand::Derivatives(_) => fetch_derivatives_index(&client, &args.date).await?,
    };

    match args.output.as_str() {
        "table" => {
            print_table(
                &["Date", "Class", "Name", "Close", "Change", "Rate(%)"],
                &records,
                |r| {
                    vec![
                        r.bas_dd.clone(),
                        r.idx_clss.clone(),
                        r.idx_nm.clone(),
                        r.clsprc_idx.clone(),
                        r.cmpprevdd_idx.clone(),
                        r.fluc_rt.clone(),
                    ]
                },
            );
        }
        _ => {
            println!("{}", serde_json::to_string_pretty(&records)?);
        }
    }

    Ok(())
}

/// Handles `fetch stock <subcommand>`.
async fn handle_fetch_stock(subcommand: StockSubcommand) -> anyhow::Result<()> {
    let args = match &subcommand {
        StockSubcommand::Kospi(args)
        | StockSubcommand::Kosdaq(args)
        | StockSubcommand::KospiInfo(args)
        | StockSubcommand::KosdaqInfo(args) => args,
    };

    utils::validate_date(&args.common.date)?;

    let api_key = resolve_api_key(args.common.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    let isin = args.isin.as_deref();
    let date = &args.common.date;

    match &subcommand {
        StockSubcommand::Kospi(_) => {
            let records = fetch_kospi_stock(&client, date, isin).await?;
            output_stock_records(&records, isin, &args.common.output)?;
        }
        StockSubcommand::Kosdaq(_) => {
            let records = fetch_kosdaq_stock(&client, date, isin).await?;
            output_stock_records(&records, isin, &args.common.output)?;
        }
        StockSubcommand::KospiInfo(_) => {
            let records = fetch_kospi_stock_info(&client, date, isin).await?;
            output_stock_info_records(&records, isin, &args.common.output)?;
        }
        StockSubcommand::KosdaqInfo(_) => {
            let records = fetch_kosdaq_stock_info(&client, date, isin).await?;
            output_stock_info_records(&records, isin, &args.common.output)?;
        }
    }

    Ok(())
}

/// Outputs stock daily records with optional record count logging.
fn output_stock_records(
    records: &[endpoints::stock::StockRecord],
    isin: Option<&str>,
    output: &str,
) -> anyhow::Result<()> {
    if isin.is_none() {
        eprintln!("Fetched {} records", records.len());
    }
    match output {
        "table" => {
            print_table(
                &[
                    "Date", "ISIN", "Name", "Close", "Change", "Rate(%)", "Volume",
                ],
                records,
                |r| {
                    vec![
                        r.bas_dd.clone(),
                        r.isu_cd.clone(),
                        r.isu_nm.clone(),
                        r.tdd_clsprc.clone(),
                        r.cmpprevdd_prc.clone(),
                        r.fluc_rt.clone(),
                        r.acc_trdvol.clone(),
                    ]
                },
            );
        }
        _ => println!("{}", serde_json::to_string_pretty(&records)?),
    }
    Ok(())
}

/// Outputs stock info records with optional record count logging.
fn output_stock_info_records(
    records: &[endpoints::stock::StockInfoRecord],
    isin: Option<&str>,
    output: &str,
) -> anyhow::Result<()> {
    if isin.is_none() {
        eprintln!("Fetched {} records", records.len());
    }
    match output {
        "table" => {
            print_table(
                &[
                    "ISIN",
                    "Code",
                    "Name",
                    "English Name",
                    "Market",
                    "Par Value",
                ],
                records,
                |r| {
                    vec![
                        r.isu_cd.clone(),
                        r.isu_srt_cd.clone(),
                        r.isu_nm.clone(),
                        r.isu_eng_nm.clone(),
                        r.mkt_tp_nm.clone(),
                        r.parval.clone(),
                    ]
                },
            );
        }
        _ => println!("{}", serde_json::to_string_pretty(&records)?),
    }
    Ok(())
}

/// Minimum date for ETN data availability.
const ETN_MIN_DATE: &str = "20141117";

/// Handles `fetch etp <subcommand>`.
async fn handle_fetch_etp(subcommand: EtpSubcommand) -> anyhow::Result<()> {
    let args = match &subcommand {
        EtpSubcommand::Etf(args) | EtpSubcommand::Etn(args) => args,
    };

    utils::validate_date(&args.date)?;

    // Warn if ETN date is before data availability.
    if matches!(&subcommand, EtpSubcommand::Etn(_)) && args.date.as_str() < ETN_MIN_DATE {
        eprintln!("Warning: ETN data is available from 2014-11-17. Results may be empty.");
    }

    let api_key = resolve_api_key(args.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    match &subcommand {
        EtpSubcommand::Etf(_) => {
            let mut records = fetch_etf_daily(&client, &args.date).await?;
            if let Some(isin) = &args.isin {
                records.retain(|r| r.isu_cd == *isin);
            }
            match args.output.as_str() {
                "table" => {
                    print_table(
                        &["Date", "Code", "Name", "Close", "Change", "Rate(%)", "NAV"],
                        &records,
                        |r| {
                            vec![
                                r.bas_dd.clone(),
                                r.isu_cd.clone(),
                                r.isu_nm.clone(),
                                r.tdd_clsprc.clone(),
                                r.cmpprevdd_prc.clone(),
                                r.fluc_rt.clone(),
                                r.nav.clone(),
                            ]
                        },
                    );
                }
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        EtpSubcommand::Etn(_) => {
            let mut records = fetch_etn_daily(&client, &args.date).await?;
            if let Some(isin) = &args.isin {
                records.retain(|r| r.isu_cd == *isin);
            }
            match args.output.as_str() {
                "table" => {
                    print_table(
                        &[
                            "Date", "Code", "Name", "Close", "Change", "Rate(%)", "IndicVal",
                        ],
                        &records,
                        |r| {
                            vec![
                                r.bas_dd.clone(),
                                r.isu_cd.clone(),
                                r.isu_nm.clone(),
                                r.tdd_clsprc.clone(),
                                r.cmpprevdd_prc.clone(),
                                r.fluc_rt.clone(),
                                r.indic_val_amt.clone(),
                            ]
                        },
                    );
                }
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
    }

    Ok(())
}

/// Handles `fetch derivatives <subcommand>`.
async fn handle_fetch_derivatives(subcommand: DerivativesSubcommand) -> anyhow::Result<()> {
    let args = match &subcommand {
        DerivativesSubcommand::Futures(args)
        | DerivativesSubcommand::StockFuturesKospi(args)
        | DerivativesSubcommand::StockFuturesKosdaq(args)
        | DerivativesSubcommand::Options(args)
        | DerivativesSubcommand::StockOptionsKospi(args)
        | DerivativesSubcommand::StockOptionsKosdaq(args) => args,
    };

    utils::validate_date(&args.date)?;

    let api_key = resolve_api_key(args.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    match &subcommand {
        DerivativesSubcommand::Futures(_) => {
            let records = fetch_futures_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            output_futures(&records, &args.output)?;
        }
        DerivativesSubcommand::StockFuturesKospi(_) => {
            let records = fetch_stock_futures_kospi_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            output_futures(&records, &args.output)?;
        }
        DerivativesSubcommand::StockFuturesKosdaq(_) => {
            let records = fetch_stock_futures_kosdaq_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            output_futures(&records, &args.output)?;
        }
        DerivativesSubcommand::Options(_) => {
            let records = fetch_options_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            output_options(&records, &args.output)?;
        }
        DerivativesSubcommand::StockOptionsKospi(_) => {
            let records = fetch_stock_options_kospi_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            output_options(&records, &args.output)?;
        }
        DerivativesSubcommand::StockOptionsKosdaq(_) => {
            let records = fetch_stock_options_kosdaq_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            output_options(&records, &args.output)?;
        }
    }

    Ok(())
}

/// Outputs futures records in table or JSON format.
fn output_futures(
    records: &[endpoints::derivatives::FuturesRecord],
    output: &str,
) -> anyhow::Result<()> {
    match output {
        "table" => {
            print_table(
                &[
                    "Date", "Code", "Name", "Close", "Settle", "Change", "Volume", "OpenInt",
                ],
                records,
                |r| {
                    vec![
                        r.bas_dd.clone(),
                        r.isu_cd.clone(),
                        r.isu_nm.clone(),
                        r.tdd_clsprc.clone(),
                        r.setl_prc.clone(),
                        r.cmpprevdd_prc.clone(),
                        r.acc_trdvol.clone(),
                        r.acc_opnint_qty.clone(),
                    ]
                },
            );
        }
        _ => println!("{}", serde_json::to_string_pretty(&records)?),
    }
    Ok(())
}

/// Outputs options records in table or JSON format.
fn output_options(
    records: &[endpoints::derivatives::OptionsRecord],
    output: &str,
) -> anyhow::Result<()> {
    match output {
        "table" => {
            print_table(
                &[
                    "Date", "Code", "Name", "Type", "Close", "Change", "Volume", "IV",
                ],
                records,
                |r| {
                    vec![
                        r.bas_dd.clone(),
                        r.isu_cd.clone(),
                        r.isu_nm.clone(),
                        r.rght_tp_nm.clone(),
                        r.tdd_clsprc.clone(),
                        r.cmpprevdd_prc.clone(),
                        r.acc_trdvol.clone(),
                        r.imp_volt.clone(),
                    ]
                },
            );
        }
        _ => println!("{}", serde_json::to_string_pretty(&records)?),
    }
    Ok(())
}

/// Handles `init --key <API_KEY>`.
///
/// Creates `~/.krxon/config.json` with the given API key.
/// Skips if the config file already exists.
fn handle_init(api_key: &str) -> anyhow::Result<()> {
    let home =
        std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME 환경 변수를 찾을 수 없습니다"))?;
    let config_dir = std::path::Path::new(&home).join(".krxon");
    let config_path = config_dir.join("config.json");

    if config_path.exists() {
        println!("설정 파일이 이미 존재합니다: {}", config_path.display());
        println!("기존 설정을 덮어쓰려면 파일을 삭제 후 다시 실행하세요.");
        return Ok(());
    }

    std::fs::create_dir_all(&config_dir)?;

    let config = serde_json::json!({ "api_key": api_key });
    let config_bytes = serde_json::to_string_pretty(&config)?;

    // Write with restrictive permissions (0o600) since the file contains a secret.
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut opts = std::fs::OpenOptions::new();
        opts.write(true).create_new(true).mode(0o600);
        std::io::Write::write_all(&mut opts.open(&config_path)?, config_bytes.as_bytes())?;
    }
    #[cfg(not(unix))]
    {
        std::fs::write(&config_path, config_bytes)?;
    }

    println!("설정 파일 생성 완료: {}", config_path.display());
    Ok(())
}

/// Handles `clean`.
///
/// Removes the `~/.krxon` config directory.
/// Skips if it doesn't exist.
fn handle_clean() -> anyhow::Result<()> {
    let home =
        std::env::var("HOME").map_err(|_| anyhow::anyhow!("HOME 환경 변수를 찾을 수 없습니다"))?;
    let config_dir = std::path::Path::new(&home).join(".krxon");

    if !config_dir.exists() {
        println!(
            "설정 디렉토리가 존재하지 않습니다: {}",
            config_dir.display()
        );
        return Ok(());
    }

    std::fs::remove_dir_all(&config_dir)?;
    println!("설정 디렉토리 삭제 완료: {}", config_dir.display());
    Ok(())
}
