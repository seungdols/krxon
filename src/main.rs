//! krxon - CLI tool and MCP server for KRX (Korea Exchange) Open API.

#![allow(dead_code)]

mod cli;
mod client;
mod codegen;
mod endpoints;
mod error;
mod mcp;
mod utils;

use clap::Parser;

use cli::{Cli, Commands, FetchResource, IndexSubcommand, StockSubcommand};
use client::{resolve_api_key, KrxClient};
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
        },
        Commands::Generate => {
            eprintln!("generate command not yet implemented");
        }
        Commands::Serve => {
            eprintln!("serve command not yet implemented");
        }
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
