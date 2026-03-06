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

use cli::{Cli, Commands};

/// Application entry point.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch => {
            eprintln!("fetch command not yet implemented");
        }
        Commands::Generate => {
            eprintln!("generate command not yet implemented");
        }
        Commands::Serve => {
            eprintln!("serve command not yet implemented");
        }
    }

    Ok(())
}
