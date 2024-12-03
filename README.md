# cuid2gen

A fast and secure command-line tool for generating [CUID2](https://github.com/paralleldrive/cuid2) identifiers - Collision-resistant Unique IDs.

[![Crates.io](https://img.shields.io/crates/v/cuid2gen.svg)](https://crates.io/crates/cuid2gen)
[![Documentation](https://docs.rs/cuid2gen/badge.svg)](https://docs.rs/cuid2gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Generate one or multiple CUID2 identifiers
- Multiple output formats (newline-separated, comma-separated, JSON array)
- Fast and memory efficient
- Zero configuration required
- Cross-platform support

## Installation

### Using Cargo

```bash
cargo install cuid2gen
```

### Using Homebrew‰

```bash
brew install cuid2gen
```

### Using Nix

```bash
nix-env -i cuid2gen
```

## Usage

Generate a single CUID2:

```bash
cuid2gen
```

Generate multiple CUIDs:

```bash
cuid2gen -c 5
```

Generate as JSON array:

```bash
cuid2gen -c 3 --format json
```

Generate comma-separated values:

```bash
cuid2gen -c 3 --format csv
```

## Options

```
USAGE:
    cuid2gen [OPTIONS]

OPTIONS:
    -c, --count <COUNT>      Number of IDs to generate [default: 1]
    -l, --length <LENGTH>    Length of each ID (not supported in current version)
    -f, --format <FORMAT>    Output format: line (default), csv, json
    -q, --quiet             Only output errors
    -h, --help             Print help
    -V, --version          Print version
```

## Why CUID2?

CUID2s are designed to be:

- Secure: resistant to prediction and scanning attacks
- Collision-resistant: extremely low probability of duplicates
- Horizontally scalable: safe for distributed systems
- URL-safe and ASCII-safe
- Sorted by time of creation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/raikusy/cuid2gen.git
cd cuid2gen

# Build
cargo build

# Run tests
cargo test

# Run formatter
cargo fmt

# Run linter
cargo clippy
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [CUID2](https://github.com/paralleldrive/cuid2) - The original CUID2 specification
- [cuid2-rs](https://github.com/mplanchard/cuid2-rs) - Rust implementation of CUID2
