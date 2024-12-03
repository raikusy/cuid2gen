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

/// Generates `count` number of CUID2 identifiers.
///
/// # Arguments
///
/// * `count` - The number of IDs to generate
///
/// # Returns
///
/// A vector containing the generated CUID2 identifiers
fn generate_ids(count: u32) -> Vec<String> {
    (0..count).map(|_| create_id()).collect()
}

/// Output CUID2 identifiers in the specified format.
///
/// # Arguments
///
/// * `ids` - A vector of CUID2 identifiers to output
/// * `format` - The output format (Line, CSV, or JSON)
/// * `quiet` - Whether to suppress output
///
/// # Returns
///
/// Result indicating success or an error if writing fails
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

/// A command-line tool for generating CUID2 identifiers
///
/// # Usage
///
/// ```text
/// cuid2 [OPTIONS]
///
/// Options:
///   -c, --count <COUNT>    Number of IDs to generate [default: 1]
///   -l, --length <LENGTH>  Length of each ID (not currently supported)
///   -f, --format <FORMAT>  Output format: line (default), csv, json [default: line]
///   -q, --quiet           Only output errors
///   -h, --help           Print help
///   -V, --version        Print version
/// ```
///
/// # Examples
///
/// Generate a single CUID2:
/// ```text
/// $ cuid2
/// ckxyv07pg000001l53uw5cs8n
/// ```
///
/// Generate 3 CUIDs in CSV format:
/// ```text
/// $ cuid2 -c 3 -f csv
/// ckxyv07pg000001l53uw5cs8n,ckxyv07pg000002l53uw5cs8n,ckxyv07pg000003l53uw5cs8n
/// ```
///
/// Generate 2 CUIDs in JSON format:
/// ```text
/// $ cuid2 -c 2 -f json
/// {"ids":["ckxyv07pg000001l53uw5cs8n","ckxyv07pg000002l53uw5cs8n"]}
/// ```
///
fn main() -> Result<()> {
    let args = Args::parse();

    if args.count > MAX_COUNT {
        return Err(anyhow!("Count exceeds maximum allowed value"));
    }

    if args.length.is_some() {
        eprintln!("Warning: Custom length is not supported in the current version of cuid2");
    }

    let ids = generate_ids(args.count);
    output_ids(ids, &args.format, args.quiet)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn test_basic_generation() {
        let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
        cmd.assert()
            .success()
            .stdout(predicate::str::is_match(r"^[a-z0-9]+\n$").unwrap());
    }

    #[test]
    fn test_multiple_ids() {
        let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
        cmd.arg("--count").arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::is_match(r"^[a-z0-9]+\n[a-z0-9]+\n[a-z0-9]+\n$").unwrap());
    }

    #[test]
    fn test_length_warning() {
        let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
        cmd.arg("--length").arg("10");
        cmd.assert()
            .success()
            .stderr(predicate::str::contains("not supported"));
    }

    #[test]
    fn test_json_format() {
        let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
        cmd.arg("--format").arg("json");
        cmd.assert()
            .success()
            .stdout(predicate::str::is_match(r#"^\{"ids":\["[a-z0-9]+"\]\}\n$"#).unwrap());
    }

    #[test]
    fn test_csv_format() {
        let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
        cmd.arg("--count").arg("2").arg("--format").arg("csv");
        cmd.assert()
            .success()
            .stdout(predicate::str::is_match(r"^[a-z0-9]+,[a-z0-9]+\n$").unwrap());
    }

    #[test]
    fn test_quiet_mode() {
        let mut cmd = Command::cargo_bin("cuid2gen").unwrap();
        cmd.arg("--quiet");
        cmd.assert().success().stdout(predicate::str::is_empty());
    }
}
