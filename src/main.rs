// src/main.rs
/*
 * Main executable for EventBus
 */

use clap::Parser;
use eventbus::{Result, run};

#[derive(Parser)]
#[command(version, about = "EventBus - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
