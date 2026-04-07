# cuid2gen

A fast command-line tool for generating [CUID2](https://github.com/paralleldrive/cuid2) identifiers.

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/raikusy/cuid2gen/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cuid2gen.svg)](https://crates.io/crates/cuid2gen)
[![Documentation](https://docs.rs/cuid2gen/badge.svg)](https://docs.rs/cuid2gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Why Use It

- Generate one or many collision-resistant IDs from a small CLI.
- Control output shape with line, CSV, or JSON formats.
- Tune generated ID length when you need shorter or longer identifiers.
- Install with Cargo directly or through the npm wrapper package.

## Installation

### Cargo

```bash
cargo install cuid2gen
```

### npx

```bash
npx cuid2gen
```

### npm

```bash
npm install -g cuid2gen
cuid2gen
```

## Usage

Generate one ID:

```bash
cuid2gen
```

Example output:

```text
tz4a98xxat96iws9zmbrgj3a
```

Generate multiple IDs:

```bash
cuid2gen --count 3
```

Example output:

```text
f2q7k1h3hzeg5zfr71z2jnbv
u5nscv4mg3zib6kju1omq0vl
u4t8g9xvdw7w3np7am9r2h6u
```

Generate IDs with a custom length:

```bash
cuid2gen --length 10
```

Example output:

```text
e6r0k7n3xq
```

Generate CSV output:

```bash
cuid2gen --count 3 --format csv
```

Example output:

```text
fx0m9z5a8j2v5q8r8p1w4n7c,m1b4d7g1t4y0n8s7x5h2p6kj,x5v8m2p3j9z4r2f1c6b7n0qw
```

Generate JSON output:

```bash
cuid2gen --count 2 --format json
```

Example output:

```json
{"ids":["d3f4m6r8s1z7x0c2v9n5k1jh","n0q8t2v4m7p6r1c9x3z5b2wd"]}
```

## CLI Reference

```text
Usage: cuid2gen [OPTIONS]

Options:
  -c, --count <COUNT>
          Number of IDs to generate

          [default: 1]

  -l, --length <LENGTH>
          Length of each ID (minimum: 2, default: 24)

  -f, --format <FORMAT>
          Output format: line (default), csv, json

          [default: line]
          [possible values: line, csv, json]

  -q, --quiet
          Only output errors

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Validation Rules

- `--count` must not exceed `1000000`.
- `--length` must be at least `2`.
- `--format json` returns an object with an `ids` array, not a bare JSON array.
- `--quiet` suppresses normal output and only leaves error output.

## Development

Prerequisites:

- Rust `1.74+`
- Node.js `22+` only if you are working on the npm wrapper in [`npm/cuid2gen`](npm/cuid2gen)

Common commands:

```bash
cargo build
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

If you are changing the npm wrapper:

```bash
cd npm/cuid2gen
npm install
npm run build
```

## Release and Distribution

- The primary implementation is the Rust crate published as `cuid2gen`.
- The npm package is a thin Node.js launcher that resolves a platform-specific binary package and executes it.
- Node.js version requirements apply to the npm installation path, not to direct Rust or release-binary usage.

## Project Docs

- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security Policy](SECURITY.md)
- [License](LICENSE)

## Acknowledgments

- [CUID2](https://github.com/paralleldrive/cuid2) for the identifier specification.
- [cuid2-rs](https://github.com/mplanchard/cuid2-rs) for the Rust implementation used by this CLI.
