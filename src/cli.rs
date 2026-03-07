//! CLI entrypoint using clap derive macros.

use clap::{Parser, Subcommand};

/// krxon - CLI tool and MCP server for KRX Open API.
#[derive(Parser, Debug)]
#[command(name = "krxon")]
#[command(about = "CLI tool and MCP server for KRX (Korea Exchange) Open API")]
#[command(version)]
pub struct Cli {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch market data from the KRX API.
    Fetch {
        #[command(subcommand)]
        resource: FetchResource,
    },
    /// Generate SDK clients (Python, TypeScript).
    Generate,
    /// Run as an MCP server.
    Serve,
}

/// Fetch resource categories.
#[derive(Subcommand, Debug)]
pub enum FetchResource {
    /// Fetch index (KRX/KOSPI/KOSDAQ/Derivatives) data.
    Index {
        #[command(subcommand)]
        subcommand: IndexSubcommand,
    },
    /// Fetch stock (KOSPI/KOSDAQ daily trading and info) data.
    Stock {
        #[command(subcommand)]
        subcommand: StockSubcommand,
    },
}

/// Index subcommands.
#[derive(Subcommand, Debug)]
pub enum IndexSubcommand {
    /// KRX composite index daily data.
    Krx(FetchArgs),
    /// KOSPI index daily data.
    Kospi(FetchArgs),
    /// KOSDAQ index daily data.
    Kosdaq(FetchArgs),
    /// Derivatives index daily data.
    Derivatives(FetchArgs),
}

/// Stock subcommands.
#[derive(Subcommand, Debug)]
pub enum StockSubcommand {
    /// KOSPI stock daily trading data.
    Kospi(StockFetchArgs),
    /// KOSDAQ stock daily trading data.
    Kosdaq(StockFetchArgs),
    /// KOSPI stock base info.
    #[command(name = "kospi-info")]
    KospiInfo(StockFetchArgs),
    /// KOSDAQ stock base info.
    #[command(name = "kosdaq-info")]
    KosdaqInfo(StockFetchArgs),
}

/// Common arguments for all fetch subcommands.
#[derive(clap::Args, Debug)]
pub struct FetchArgs {
    /// Base date in YYYYMMDD format.
    #[arg(long)]
    pub date: String,

    /// API key (overrides KRX_API_KEY env var).
    #[arg(long)]
    pub key: Option<String>,

    /// Output format: json or table.
    #[arg(long, default_value = "json", value_parser = ["json", "table"])]
    pub output: String,
}

/// Arguments for stock fetch subcommands (adds --isin option).
#[derive(clap::Args, Debug)]
pub struct StockFetchArgs {
    /// Common fetch arguments (date, key, output).
    #[command(flatten)]
    pub common: FetchArgs,

    /// ISIN code to filter a specific stock (e.g. KR7005930003).
    #[arg(long)]
    pub isin: Option<String>,
}
