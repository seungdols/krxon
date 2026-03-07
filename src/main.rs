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

use cli::{Cli, Commands, FetchResource, IndexSubcommand};
use client::{resolve_api_key, KrxClient};
use endpoints::index::{
    fetch_derivatives_index, fetch_kosdaq_index, fetch_kospi_index, fetch_krx_index,
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
    let client = KrxClient::new(api_key);

    // Call the appropriate endpoint.
    let records = match &subcommand {
        IndexSubcommand::Krx(_) => fetch_krx_index(&client, &args.date).await?,
        IndexSubcommand::Kospi(_) => fetch_kospi_index(&client, &args.date).await?,
        IndexSubcommand::Kosdaq(_) => fetch_kosdaq_index(&client, &args.date).await?,
        IndexSubcommand::Derivatives(_) => fetch_derivatives_index(&client, &args.date).await?,
    };

    // Output results.
    match args.output.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&records)?;
            println!("{}", json);
        }
        "table" => {
            print_index_table(&records);
        }
        other => {
            anyhow::bail!("Unknown output format: '{}'. Use 'json' or 'table'", other);
        }
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
