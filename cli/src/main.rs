//! Vantis CLI - Main Entry Point
//!
//! Command-line interface for Vantis Media Player.

use anyhow::Result;
use vantis_cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}