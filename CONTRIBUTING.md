# Contributing to VEAC

Thank you for your interest in contributing to VEAC (Video Editing as Code)! This guide will help you get started.

## Development Environment Setup

### Prerequisites

- **Rust toolchain** (1.70 or later) — install via [rustup](https://rustup.rs/)
- **FFmpeg** — install via your system package manager (e.g., `brew install ffmpeg` on macOS, `apt install ffmpeg` on Debian/Ubuntu)

### Getting Started

```bash
git clone https://github.com/yishuiliunian/VEAC.git
cd VEAC
cargo build
```

## Project Architecture

VEAC is organized as a Cargo workspace with four crates:

| Crate | Responsibility |
|---|---|
| **veac-lang** | Lexer → Parser → Semantic Analyzer → IR |
| **veac-codegen** | IR → FFmpeg CLI commands and filter graphs |
| **veac-runtime** | FFmpeg execution, media probing, progress tracking |
| **veac-cli** | CLI entry point (`build`, `check`, `plan`, `fmt`, `probe`) |

## Code Style

All code must pass formatting and lint checks before merging:

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

Please run both commands before submitting a pull request.

## Testing

Run the full test suite with:

```bash
cargo test --all
```

Make sure all tests pass before opening a pull request. When adding new functionality, include corresponding tests in the relevant `tests/` directory.

## Pull Request Workflow

1. **Fork** the repository and clone your fork locally.
2. **Create a branch** from `main` for your change:
   ```bash
   git checkout -b my-feature
   ```
3. **Make your changes** in small, focused commits with clear messages.
4. **Run checks** before pushing:
   ```bash
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   cargo test --all
   ```
5. **Push** your branch and open a **Pull Request** against `main`.
6. Respond to any **review feedback** and update your PR as needed.

## Reporting Issues

When opening an issue, please include:

- A clear and descriptive title.
- Steps to reproduce the problem (including a minimal `.veac` file if applicable).
- Expected behavior vs. actual behavior.
- Your environment details (OS, Rust version, FFmpeg version).
- Any relevant error messages or log output.

## License

By contributing to VEAC, you agree that your contributions will be licensed under the [MIT License](LICENSE).
