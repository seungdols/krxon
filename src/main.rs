//! krxon - CLI tool and MCP server for KRX (Korea Exchange) Open API.

#![allow(dead_code)]

mod cli;
mod client;
mod codegen;
mod endpoints;
mod error;
mod mcp;
mod output;
mod utils;

use clap::Parser;

use cli::{Cli, Commands, FetchCategory, IndexMarket, OutputFormat};
use client::KrxClient;

/// Extracts `FetchOptions` from any `IndexMarket` variant.
fn index_market_opts(market: &IndexMarket) -> &cli::FetchOptions {
    match market {
        IndexMarket::Krx(opts)
        | IndexMarket::Kospi(opts)
        | IndexMarket::Kosdaq(opts)
        | IndexMarket::Derivatives(opts) => opts,
    }
}

/// Application entry point.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch(fetch_args) => match fetch_args.category {
            FetchCategory::Index(ref market) => {
                let opts = index_market_opts(market);

                // 1. Validate date format
                if !utils::is_valid_date_format(&opts.date) {
                    return Err(error::KrxError::InvalidDate(opts.date.clone()).into());
                }

                // 2. Resolve API key
                let api_key = client::resolve_api_key(opts.key.as_deref())?;
                let krx_client = KrxClient::new(&api_key)?;

                // 3. Call the appropriate endpoint
                let data = match market {
                    IndexMarket::Krx(_) => {
                        endpoints::index::fetch_krx_index(&krx_client, &opts.date).await?
                    }
                    IndexMarket::Kospi(_) => {
                        endpoints::index::fetch_kospi_index(&krx_client, &opts.date).await?
                    }
                    IndexMarket::Kosdaq(_) => {
                        endpoints::index::fetch_kosdaq_index(&krx_client, &opts.date).await?
                    }
                    IndexMarket::Derivatives(_) => {
                        endpoints::index::fetch_derivatives_index(&krx_client, &opts.date).await?
                    }
                };

                // 4. Format output
                let formatted = match opts.format {
                    OutputFormat::Json => serde_json::to_string_pretty(&data)?,
                    OutputFormat::Csv => output::format_as_csv(&data),
                    OutputFormat::Table => output::format_as_table(&data),
                };

                // 5. Write to stdout or file
                match &opts.output {
                    Some(path) => std::fs::write(path, format!("{}\n", formatted))?,
                    None => println!("{}", formatted),
                }
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
