// src/main.rs
/*
 * Main executable for NftDigitalFingerprinterTechUltra
 */

use clap::Parser;
use nftdigitalfingerprintertechultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftDigitalFingerprinterTechUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
