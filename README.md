# rs-clicolor

[![CI](https://github.com/philiprehberger/rs-clicolor/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-clicolor/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-clicolor.svg)](https://crates.io/crates/philiprehberger-clicolor)
[![GitHub release](https://img.shields.io/github/v/release/philiprehberger/rs-clicolor)](https://github.com/philiprehberger/rs-clicolor/releases)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-clicolor)](https://github.com/philiprehberger/rs-clicolor/commits/main)
[![License](https://img.shields.io/github/license/philiprehberger/rs-clicolor)](LICENSE)
[![Bug Reports](https://img.shields.io/github/issues/philiprehberger/rs-clicolor/bug)](https://github.com/philiprehberger/rs-clicolor/issues?q=is%3Aissue+is%3Aopen+label%3Abug)
[![Feature Requests](https://img.shields.io/github/issues/philiprehberger/rs-clicolor/enhancement)](https://github.com/philiprehberger/rs-clicolor/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
[![Sponsor](https://img.shields.io/badge/sponsor-GitHub%20Sponsors-ec6cb9)](https://github.com/sponsors/philiprehberger)

Cross-platform terminal color and style output with automatic capability detection

## Installation

```toml
[dependencies]
philiprehberger-clicolor = "0.1.1"
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

## Support

If you find this package useful, consider giving it a star on GitHub — it helps motivate continued maintenance and development.

[![LinkedIn](https://img.shields.io/badge/Philip%20Rehberger-LinkedIn-0A66C2?logo=linkedin)](https://www.linkedin.com/in/philiprehberger)
[![More packages](https://img.shields.io/badge/more-open%20source%20packages-blue)](https://philiprehberger.com/open-source-packages)

## License

[MIT](LICENSE)
