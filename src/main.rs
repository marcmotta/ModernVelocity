// src/main.rs
/*
 * Main executable for ModernVelocity
 */

use clap::Parser;
use modernvelocity::{Result, run};

#[derive(Parser)]
#[command(version, about = "ModernVelocity - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
