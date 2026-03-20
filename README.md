# rs-clicolor

Cross-platform terminal color and style output with automatic capability detection.

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

### Reusable styles

```rust
use philiprehberger_clicolor::Style;

let error = Style::new().red().bold();
let info = Style::new().cyan();

println!("{}", error.paint("Something failed"));
println!("{}", info.paint("FYI"));
```

### Environment support

Automatically respects:
- `NO_COLOR` — disables all colors
- `CLICOLOR=0` — disables colors
- `CLICOLOR_FORCE` — forces colors even when not a TTY
- Piped output — colors disabled when stdout is not a terminal

## API

| Function / Type | Description |
|----------------|-------------|
| `Colorize` trait | Adds `.red()`, `.bold()`, etc. to `&str` |
| `Style` | Reusable style builder |
| `Color` | Color enum (16 colors + 256 + RGB) |
| `strip_ansi(s)` | Remove ANSI escape codes from a string |

## License

MIT
