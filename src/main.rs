use std::io::{self, Write};

use anyhow::{Context, Result};
use clap::Parser;
use cuid2::create_id;
use serde::Serialize;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Generate CUID2s - Collision-resistant Unique IDs",
    long_about = "A command-line tool for generating CUID2 identifiers - secure, collision-resistant IDs suitable for distributed systems"
)]
struct Args {
    /// Number of IDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    /// Length of each ID (not supported in current cuid2 version)
    #[arg(short, long)]
    length: Option<u32>,

    /// Output format: line (default), csv, json
    #[arg(short, long, default_value = "line")]
    format: String,

    /// Only output errors
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Serialize)]
struct Output {
    ids: Vec<String>,
}

fn generate_ids(count: u32) -> Vec<String> {
    (0..count).map(|_| create_id()).collect()
}

fn output_ids(ids: Vec<String>, format: &str, quiet: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match format.to_lowercase().as_str() {
        "line" => {
            for id in ids {
                writeln!(handle, "{}", id).context("Failed to write to stdout")?;
            }
        }
        "csv" => {
            writeln!(handle, "{}", ids.join(",")).context("Failed to write to stdout")?;
        }
        "json" => {
            let output = Output { ids };
            serde_json::to_writer_pretty(&mut handle, &output).context("Failed to serialize to JSON")?;
            writeln!(handle).context("Failed to write newline")?;
        }
        _ => {
            eprintln!("Invalid format specified. Using default 'line' format.");
            if !quiet {
                for id in ids {
                    writeln!(handle, "{}", id).context("Failed to write to stdout")?;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if let Some(_) = args.length {
        eprintln!("Warning: Custom length is not supported in the current version of cuid2");
    }

    let ids = generate_ids(args.count);
    output_ids(ids, &args.format, args.quiet)?;

    Ok(())
}
