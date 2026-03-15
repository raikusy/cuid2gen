# Contributing to cuid2gen

Thanks for contributing to `cuid2gen`.

## Before You Start

- Rust `1.74+` is required for core development.
- Node.js `22+` is only needed if you are changing the npm wrapper under `npm/cuid2gen`.
- For large changes, open an issue first so the behavior change can be discussed before implementation.

## Local Setup

```bash
git clone https://github.com/raikusy/cuid2gen.git
cd cuid2gen
cargo build
cargo test
```

If your change touches the npm wrapper:

```bash
cd npm/cuid2gen
npm install
npm run build
```

## Required Checks

Run these before opening a pull request:

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

Run the npm build as well if you changed files under `npm/`.

## Pull Request Expectations

- Keep changes focused and explain the user-facing impact.
- Add or update tests when behavior changes.
- Update documentation when CLI flags, output, installation, or release behavior changes.
- Link related issues in the pull request description when applicable.

## Branches and Commits

- Use a descriptive branch name.
- Write commit messages that explain what changed.
- Squash or clean up noisy work-in-progress commits before merge if needed.

## Reporting Bugs and Proposing Features

- Use GitHub issues for bugs, regressions, feature requests, and documentation gaps.
- Do not use public issues for undisclosed security vulnerabilities. Follow the process in [SECURITY.md](SECURITY.md).
