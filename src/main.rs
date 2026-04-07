use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use std::io::{self, Write};

// Define a reasonable maximum count to prevent excessive memory usage
const MAX_COUNT: u32 = 1_000_000;
const MAX_LENGTH: u16 = 128;

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

    /// Length of each ID (minimum: 2, maximum: 128, default: 24)
    #[arg(short, long)]
    length: Option<u16>,

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

fn create_id(length: Option<u16>, constructor: Option<&cuid2::CuidConstructor>) -> String {
    match (length, constructor) {
        (Some(_), Some(constructor)) => constructor.create_id(),
        _ => cuid2::create_id(),
    }
}

fn output_ids(count: u32, length: Option<u16>, format: &OutputFormat, quiet: bool) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let constructor = length.map(|len| cuid2::CuidConstructor::new().with_length(len));

    match format {
        OutputFormat::Line => {
            for _ in 0..count {
                if !quiet {
                    let id = create_id(length, constructor.as_ref());
                    writeln!(handle, "{}", id).context("Failed to write to stdout")?;
                }
            }
        }
        OutputFormat::Csv => {
            if !quiet {
                for index in 0..count {
                    if index > 0 {
                        write!(handle, ",").context("Failed to write CSV delimiter")?;
                    }
                    let id = create_id(length, constructor.as_ref());
                    write!(handle, "{}", id).context("Failed to write to stdout")?;
                }
                if count > 0 {
                    writeln!(handle).context("Failed to write newline")?;
                }
            }
        }
        OutputFormat::Json => {
            if !quiet {
                write!(handle, "{{\"ids\":[").context("Failed to write JSON prefix")?;
                for index in 0..count {
                    if index > 0 {
                        write!(handle, ",").context("Failed to write JSON delimiter")?;
                    }
                    let id = create_id(length, constructor.as_ref());
                    serde_json::to_writer(&mut handle, &id)
                        .context("Failed to serialize ID to JSON")?;
                }
                write!(handle, "]}}").context("Failed to write JSON suffix")?;
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

    if let Some(len) = args.length {
        if len < 2 {
            return Err(anyhow!("Length must be at least 2"));
        }
        if len > MAX_LENGTH {
            return Err(anyhow!(
                "Length exceeds maximum allowed value of {}",
                MAX_LENGTH
            ));
        }
    }

    output_ids(args.count, args.length, &args.format, args.quiet)?;

    Ok(())
}
