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
    #[arg(long, default_value = "json")]
    pub output: String,
}
