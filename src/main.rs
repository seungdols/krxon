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

/// Application entry point.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch { resource } => match resource {
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

    // Validate date format.
    if !utils::is_valid_date_format(&args.date) {
        anyhow::bail!(
            "Invalid date format: '{}'. Expected YYYYMMDD (e.g. 20250301)",
            args.date
        );
    }

    // Resolve API key.
    let api_key = resolve_api_key(args.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    // Call the appropriate endpoint.
    let records = match &subcommand {
        IndexSubcommand::Krx(_) => fetch_krx_index(&client, &args.date).await?,
        IndexSubcommand::Kospi(_) => fetch_kospi_index(&client, &args.date).await?,
        IndexSubcommand::Kosdaq(_) => fetch_kosdaq_index(&client, &args.date).await?,
        IndexSubcommand::Derivatives(_) => fetch_derivatives_index(&client, &args.date).await?,
    };

    // Output results (clap value_parser guarantees "json" or "table").
    match args.output.as_str() {
        "table" => print_index_table(&records),
        _ => {
            let json = serde_json::to_string_pretty(&records)?;
            println!("{}", json);
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

    // Validate date format.
    if !utils::is_valid_date_format(&args.common.date) {
        anyhow::bail!(
            "Invalid date format: '{}'. Expected YYYYMMDD (e.g. 20250301)",
            args.common.date
        );
    }

    // Resolve API key.
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
        "table" => print_stock_table(records),
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
        "table" => print_stock_info_table(records),
        _ => println!("{}", serde_json::to_string_pretty(&records)?),
    }
    Ok(())
}

/// Prints index records in a simple text table format.
fn print_index_table(records: &[endpoints::index::IndexRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<12} {:<10} {:<30} {:>12} {:>10} {:>8}",
        "Date", "Class", "Name", "Close", "Change", "Rate(%)"
    );
    println!("{}", "-".repeat(90));

    for r in records {
        println!(
            "{:<12} {:<10} {:<30} {:>12} {:>10} {:>8}",
            r.bas_dd, r.idx_clss, r.idx_nm, r.clsprc_idx, r.cmpprevdd_idx, r.fluc_rt
        );
    }
}

/// Prints stock daily records in a simple text table format.
fn print_stock_table(records: &[endpoints::stock::StockRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<12} {:<14} {:<20} {:>10} {:>10} {:>8} {:>14}",
        "Date", "ISIN", "Name", "Close", "Change", "Rate(%)", "Volume"
    );
    println!("{}", "-".repeat(96));

    for r in records {
        println!(
            "{:<12} {:<14} {:<20} {:>10} {:>10} {:>8} {:>14}",
            r.bas_dd, r.isu_cd, r.isu_nm, r.tdd_clsprc, r.cmpprevdd_prc, r.fluc_rt, r.acc_trdvol
        );
    }
}

/// Prints stock info records in a simple text table format.
fn print_stock_info_table(records: &[endpoints::stock::StockInfoRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<14} {:<8} {:<20} {:<20} {:<10} {:>10}",
        "ISIN", "Code", "Name", "English Name", "Market", "Par Value"
    );
    println!("{}", "-".repeat(90));

    for r in records {
        println!(
            "{:<14} {:<8} {:<20} {:<20} {:<10} {:>10}",
            r.isu_cd, r.isu_srt_cd, r.isu_nm, r.isu_eng_nm, r.mkt_tp_nm, r.parval
        );
    }
}

/// Minimum date for ETN data availability.
const ETN_MIN_DATE: &str = "20141117";

/// Handles `fetch etp <subcommand>`.
async fn handle_fetch_etp(subcommand: EtpSubcommand) -> anyhow::Result<()> {
    let args = match &subcommand {
        EtpSubcommand::Etf(args) | EtpSubcommand::Etn(args) => args,
    };

    // Validate date format.
    if !utils::is_valid_date_format(&args.date) {
        anyhow::bail!(
            "Invalid date format: '{}'. Expected YYYYMMDD (e.g. 20250301)",
            args.date
        );
    }

    // Warn if ETN date is before data availability.
    if matches!(&subcommand, EtpSubcommand::Etn(_)) && args.date.as_str() < ETN_MIN_DATE {
        eprintln!("Warning: ETN data is available from 2014-11-17. Results may be empty.");
    }

    // Resolve API key.
    let api_key = resolve_api_key(args.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    // Call the appropriate endpoint and output results.
    match &subcommand {
        EtpSubcommand::Etf(_) => {
            let mut records = fetch_etf_daily(&client, &args.date).await?;
            if let Some(isin) = &args.isin {
                records.retain(|r| r.isu_cd == *isin);
            }
            match args.output.as_str() {
                "table" => print_etf_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        EtpSubcommand::Etn(_) => {
            let mut records = fetch_etn_daily(&client, &args.date).await?;
            if let Some(isin) = &args.isin {
                records.retain(|r| r.isu_cd == *isin);
            }
            match args.output.as_str() {
                "table" => print_etn_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
    }

    Ok(())
}

/// Prints ETF records in a simple text table format.
fn print_etf_table(records: &[endpoints::etp::EtfRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<12} {:<14} {:<24} {:>10} {:>10} {:>8} {:>14}",
        "Date", "Code", "Name", "Close", "Change", "Rate(%)", "NAV"
    );
    println!("{}", "-".repeat(100));

    for r in records {
        println!(
            "{:<12} {:<14} {:<24} {:>10} {:>10} {:>8} {:>14}",
            r.bas_dd, r.isu_cd, r.isu_nm, r.tdd_clsprc, r.cmpprevdd_prc, r.fluc_rt, r.nav
        );
    }
}

/// Prints ETN records in a simple text table format.
fn print_etn_table(records: &[endpoints::etp::EtnRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<12} {:<14} {:<24} {:>10} {:>10} {:>8} {:>14}",
        "Date", "Code", "Name", "Close", "Change", "Rate(%)", "IndicVal"
    );
    println!("{}", "-".repeat(100));

    for r in records {
        println!(
            "{:<12} {:<14} {:<24} {:>10} {:>10} {:>8} {:>14}",
            r.bas_dd, r.isu_cd, r.isu_nm, r.tdd_clsprc, r.cmpprevdd_prc, r.fluc_rt, r.indic_val_amt
        );
    }
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

    // Validate date format.
    if !utils::is_valid_date_format(&args.date) {
        anyhow::bail!(
            "Invalid date format: '{}'. Expected YYYYMMDD (e.g. 20250301)",
            args.date
        );
    }

    // Resolve API key.
    let api_key = resolve_api_key(args.key.as_deref())?;
    let client = KrxClient::new(&api_key)?;

    match &subcommand {
        DerivativesSubcommand::Futures(_) => {
            let records = fetch_futures_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            match args.output.as_str() {
                "table" => print_futures_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        DerivativesSubcommand::StockFuturesKospi(_) => {
            let records = fetch_stock_futures_kospi_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            match args.output.as_str() {
                "table" => print_futures_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        DerivativesSubcommand::StockFuturesKosdaq(_) => {
            let records = fetch_stock_futures_kosdaq_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            match args.output.as_str() {
                "table" => print_futures_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        DerivativesSubcommand::Options(_) => {
            let records = fetch_options_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            match args.output.as_str() {
                "table" => print_options_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        DerivativesSubcommand::StockOptionsKospi(_) => {
            let records = fetch_stock_options_kospi_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            match args.output.as_str() {
                "table" => print_options_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
        DerivativesSubcommand::StockOptionsKosdaq(_) => {
            let records = fetch_stock_options_kosdaq_daily(&client, &args.date).await?;
            eprintln!("Fetched {} records", records.len());
            match args.output.as_str() {
                "table" => print_options_table(&records),
                _ => println!("{}", serde_json::to_string_pretty(&records)?),
            }
        }
    }

    Ok(())
}

/// Prints futures records in a simple text table format.
fn print_futures_table(records: &[endpoints::derivatives::FuturesRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<12} {:<20} {:<20} {:>10} {:>10} {:>10} {:>14} {:>14}",
        "Date", "Code", "Name", "Close", "Settle", "Change", "Volume", "OpenInt"
    );
    println!("{}", "-".repeat(118));

    for r in records {
        println!(
            "{:<12} {:<20} {:<20} {:>10} {:>10} {:>10} {:>14} {:>14}",
            r.bas_dd,
            r.isu_cd,
            r.isu_nm,
            r.tdd_clsprc,
            r.setl_prc,
            r.cmpprevdd_prc,
            r.acc_trdvol,
            r.acc_opnint_qty
        );
    }
}

/// Prints options records in a simple text table format.
fn print_options_table(records: &[endpoints::derivatives::OptionsRecord]) {
    if records.is_empty() {
        println!("No data found.");
        return;
    }

    println!(
        "{:<12} {:<20} {:<24} {:<6} {:>10} {:>10} {:>14} {:>10}",
        "Date", "Code", "Name", "Type", "Close", "Change", "Volume", "IV"
    );
    println!("{}", "-".repeat(114));

    for r in records {
        println!(
            "{:<12} {:<20} {:<24} {:<6} {:>10} {:>10} {:>14} {:>10}",
            r.bas_dd,
            r.isu_cd,
            r.isu_nm,
            r.rght_tp_nm,
            r.tdd_clsprc,
            r.cmpprevdd_prc,
            r.acc_trdvol,
            r.imp_volt
        );
    }
}
