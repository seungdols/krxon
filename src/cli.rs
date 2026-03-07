//! CLI entrypoint using clap derive macros.

use clap::{Parser, Subcommand};

/// krxon - CLI tool for KRX Open API.
#[derive(Parser, Debug)]
#[command(name = "krxon")]
#[command(about = "CLI tool for KRX (Korea Exchange) Open API")]
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
    Generate {
        #[command(subcommand)]
        language: GenerateLanguage,
    },
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
    /// Fetch ETP (ETF/ETN) data.
    Etp {
        #[command(subcommand)]
        subcommand: EtpSubcommand,
    },
    /// Fetch derivatives (futures/options) data.
    Derivatives {
        #[command(subcommand)]
        subcommand: DerivativesSubcommand,
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

/// Derivatives subcommands.
#[derive(Subcommand, Debug)]
pub enum DerivativesSubcommand {
    /// Futures daily trading data.
    Futures(FetchArgs),
    /// KOSPI stock futures daily trading data.
    #[command(name = "stock-futures-kospi")]
    StockFuturesKospi(FetchArgs),
    /// KOSDAQ stock futures daily trading data.
    #[command(name = "stock-futures-kosdaq")]
    StockFuturesKosdaq(FetchArgs),
    /// Options daily trading data.
    Options(FetchArgs),
    /// KOSPI stock options daily trading data.
    #[command(name = "stock-options-kospi")]
    StockOptionsKospi(FetchArgs),
    /// KOSDAQ stock options daily trading data.
    #[command(name = "stock-options-kosdaq")]
    StockOptionsKosdaq(FetchArgs),
}

/// ETP subcommands.
#[derive(Subcommand, Debug)]
pub enum EtpSubcommand {
    /// ETF daily trading data.
    Etf(EtpFetchArgs),
    /// ETN daily trading data.
    Etn(EtpFetchArgs),
}

/// Arguments for ETP fetch subcommands.
#[derive(clap::Args, Debug)]
pub struct EtpFetchArgs {
    /// Base date in YYYYMMDD format.
    #[arg(long)]
    pub date: String,

    /// Filter by ISIN code (e.g. KR7069500007).
    #[arg(long)]
    pub isin: Option<String>,

    /// API key (overrides KRX_API_KEY env var).
    #[arg(long)]
    pub key: Option<String>,

    /// Output format: json or table.
    #[arg(long, default_value = "json", value_parser = ["json", "table"])]
    pub output: String,
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

/// Generate language targets.
#[derive(Subcommand, Debug)]
pub enum GenerateLanguage {
    /// Generate Python SDK client.
    Python(GenerateArgs),
    /// Generate TypeScript SDK client.
    Typescript(GenerateArgs),
}

/// Arguments for the generate subcommand.
#[derive(clap::Args, Debug)]
pub struct GenerateArgs {
    /// Output directory for generated SDK files.
    #[arg(long)]
    pub out: String,
}
