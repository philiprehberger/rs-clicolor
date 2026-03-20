# rs-clicolor

[![CI](https://github.com/philiprehberger/rs-clicolor/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-clicolor/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-clicolor.svg)](https://crates.io/crates/philiprehberger-clicolor)
[![License](https://img.shields.io/github/license/philiprehberger/rs-clicolor)](LICENSE)

Cross-platform terminal color and style output with automatic capability detection

## Installation

```toml
[dependencies]
philiprehberger-clicolor = "0.1"
```

## Usage

```rust
use philiprehberger_clicolor::Colorize;

println!("{}", "Error: something failed".red().bold());
println!("{}", "Warning: check this".yellow());
println!("{}", "Success!".green().on_black());
```

## API

| Function / Type | Description |
|----------------|-------------|
| `Colorize` trait | Adds `.red()`, `.bold()`, etc. to `&str` |
| `Style` | Reusable style builder |
| `Color` | Color enum (16 colors + 256 + RGB) |
| `strip_ansi(s)` | Remove ANSI escape codes from a string |

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## License

MIT
