// src/main.rs
/*
 * Main executable for BlockchainSmartContractAuditor
 */

use clap::Parser;
use blockchainsmartcontractauditor::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainSmartContractAuditor - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
