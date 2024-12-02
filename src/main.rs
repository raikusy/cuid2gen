use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use cuid2::create_id;
use serde::Serialize;
use std::io::{self, Write};

// Define a reasonable maximum count to prevent excessive memory usage
const MAX_COUNT: u32 = 1_000_000;

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
    format: OutputFormat,

    /// Only output errors
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Line,
    Csv,
    Json,
}

#[derive(Serialize)]
struct Output {
    ids: Vec<String>,
}

fn generate_ids(count: u32) -> Vec<String> {
    (0..count).map(|_| create_id()).collect()
}

fn output_ids(ids: Vec<String>, format: &OutputFormat, quiet: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match format {
        OutputFormat::Line => {
            for id in ids {
                if !quiet {
                    writeln!(handle, "{}", id).context("Failed to write to stdout")?;
                }
            }
        }
        OutputFormat::Csv => {
            let output = ids.join(",");
            if !quiet {
                writeln!(handle, "{}", output).context("Failed to write to stdout")?;
            }
        }
        OutputFormat::Json => {
            let output = Output { ids };
            if !quiet {
                serde_json::to_writer(&mut handle, &output)
                    .context("Failed to serialize to JSON")?;
                writeln!(handle).context("Failed to write newline")?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.count > MAX_COUNT {
        return Err(anyhow!("Count exceeds maximum allowed value"));
    }

    if let Some(_) = args.length {
        eprintln!("Warning: Custom length is not supported in the current version of cuid2");
    }

    let ids = generate_ids(args.count);
    output_ids(ids, &args.format, args.quiet)?;

    Ok(())
}
