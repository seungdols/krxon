//! CLI entrypoint using clap derive macros.

use clap::{Parser, Subcommand, ValueEnum};

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
    Fetch(FetchArgs),
    /// Generate SDK clients (Python, TypeScript).
    Generate,
    /// Run as an MCP server.
    Serve,
}

/// Arguments for the `fetch` command.
#[derive(Parser, Debug)]
pub struct FetchArgs {
    /// Market data category to fetch.
    #[command(subcommand)]
    pub category: FetchCategory,
}

/// Available fetch categories.
#[derive(Subcommand, Debug)]
pub enum FetchCategory {
    /// Fetch index (지수) data.
    #[command(subcommand)]
    Index(IndexMarket),
}

/// Index market subcommands.
///
/// Each variant carries shared fetch options (`--date`, `--key`, etc.).
#[derive(Subcommand, Debug)]
pub enum IndexMarket {
    /// KRX 지수 일별 시세.
    Krx(FetchOptions),
    /// KOSPI 지수 일별 시세.
    Kospi(FetchOptions),
    /// KOSDAQ 지수 일별 시세.
    Kosdaq(FetchOptions),
    /// 파생상품 지수 일별 시세.
    Derivatives(FetchOptions),
}

/// Shared options for fetch commands.
#[derive(Parser, Debug)]
pub struct FetchOptions {
    /// 기준일자 (YYYYMMDD).
    #[arg(long)]
    pub date: String,

    /// API key (overrides KRX_API_KEY env var).
    #[arg(long)]
    pub key: Option<String>,

    /// Output format.
    #[arg(long, default_value = "json")]
    pub format: OutputFormat,

    /// Write output to file instead of stdout.
    #[arg(long)]
    pub output: Option<String>,
}

/// Output format for fetch results.
#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// JSON (default).
    Json,
    /// Comma-separated values.
    Csv,
    /// Human-readable text table.
    Table,
}
