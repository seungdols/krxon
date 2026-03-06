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
    Fetch,
    /// Generate SDK clients (Python, TypeScript).
    Generate,
    /// Run as an MCP server.
    Serve,
}
