//! Cross-platform terminal color and style output with automatic capability detection.
//!
//! # Usage
//!
//! ```rust
//! use philiprehberger_clicolor::Colorize;
//!
//! println!("{}", "Error: something failed".red().bold());
//! println!("{}", "Warning: check this".yellow());
//! println!("{}", "Success!".green().on_black());
//! ```
//!
//! # Reusable styles
//!
//! ```rust
//! use philiprehberger_clicolor::Style;
//!
//! let error = Style::new().red().bold();
//! let info = Style::new().cyan();
//!
//! println!("{}", error.paint("Something failed"));
//! println!("{}", info.paint("FYI"));
//! ```
//!
//! # Environment support
//!
//! Automatically respects:
//! - `NO_COLOR` — disables all colors
//! - `CLICOLOR=0` — disables colors
//! - `CLICOLOR_FORCE` — forces colors even when not a TTY
//! - Piped output — colors disabled when stdout is not a terminal

use std::fmt;
use std::io::IsTerminal;
use std::sync::OnceLock;

/// Terminal color values supporting 16 standard colors, 256-color palette, and RGB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Standard black (ANSI 0).
    Black,
    /// Standard red (ANSI 1).
    Red,
    /// Standard green (ANSI 2).
    Green,
    /// Standard yellow (ANSI 3).
    Yellow,
    /// Standard blue (ANSI 4).
    Blue,
    /// Standard magenta (ANSI 5).
    Magenta,
    /// Standard cyan (ANSI 6).
    Cyan,
    /// Standard white (ANSI 7).
    White,
    /// Bright black (ANSI 8).
    BrightBlack,
    /// Bright red (ANSI 9).
    BrightRed,
    /// Bright green (ANSI 10).
    BrightGreen,
    /// Bright yellow (ANSI 11).
    BrightYellow,
    /// Bright blue (ANSI 12).
    BrightBlue,
    /// Bright magenta (ANSI 13).
    BrightMagenta,
    /// Bright cyan (ANSI 14).
    BrightCyan,
    /// Bright white (ANSI 15).
    BrightWhite,
    /// 256-color palette index (0-255).
    Ansi256(u8),
    /// True color RGB value.
    Rgb(u8, u8, u8),
}

impl Color {
    /// Returns the ANSI escape code for this color as a foreground color.
    pub fn fg_code(&self) -> String {
        match self {
            Color::Black => "30".into(),
            Color::Red => "31".into(),
            Color::Green => "32".into(),
            Color::Yellow => "33".into(),
            Color::Blue => "34".into(),
            Color::Magenta => "35".into(),
            Color::Cyan => "36".into(),
            Color::White => "37".into(),
            Color::BrightBlack => "90".into(),
            Color::BrightRed => "91".into(),
            Color::BrightGreen => "92".into(),
            Color::BrightYellow => "93".into(),
            Color::BrightBlue => "94".into(),
            Color::BrightMagenta => "95".into(),
            Color::BrightCyan => "96".into(),
            Color::BrightWhite => "97".into(),
            Color::Ansi256(n) => format!("38;5;{n}"),
            Color::Rgb(r, g, b) => format!("38;2;{r};{g};{b}"),
        }
    }

    /// Returns the ANSI escape code for this color as a background color.
    pub fn bg_code(&self) -> String {
        match self {
            Color::Black => "40".into(),
            Color::Red => "41".into(),
            Color::Green => "42".into(),
            Color::Yellow => "43".into(),
            Color::Blue => "44".into(),
            Color::Magenta => "45".into(),
            Color::Cyan => "46".into(),
            Color::White => "47".into(),
            Color::BrightBlack => "100".into(),
            Color::BrightRed => "101".into(),
            Color::BrightGreen => "102".into(),
            Color::BrightYellow => "103".into(),
            Color::BrightBlue => "104".into(),
            Color::BrightMagenta => "105".into(),
            Color::BrightCyan => "106".into(),
            Color::BrightWhite => "107".into(),
            Color::Ansi256(n) => format!("48;5;{n}"),
            Color::Rgb(r, g, b) => format!("48;2;{r};{g};{b}"),
        }
    }
}

/// A reusable style definition with foreground color, background color, and text attributes.
///
/// Use the builder pattern to construct styles:
///
/// ```rust
/// use philiprehberger_clicolor::Style;
///
/// let error_style = Style::new().red().bold();
/// println!("{}", error_style.paint("Error!"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
}

impl Style {
    /// Creates a new empty style with no colors or attributes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the foreground color.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Sets the background color.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Sets the foreground to black.
    pub fn black(self) -> Self {
        self.fg(Color::Black)
    }

    /// Sets the foreground to red.
    pub fn red(self) -> Self {
        self.fg(Color::Red)
    }

    /// Sets the foreground to green.
    pub fn green(self) -> Self {
        self.fg(Color::Green)
    }

    /// Sets the foreground to yellow.
    pub fn yellow(self) -> Self {
        self.fg(Color::Yellow)
    }

    /// Sets the foreground to blue.
    pub fn blue(self) -> Self {
        self.fg(Color::Blue)
    }

    /// Sets the foreground to magenta.
    pub fn magenta(self) -> Self {
        self.fg(Color::Magenta)
    }

    /// Sets the foreground to cyan.
    pub fn cyan(self) -> Self {
        self.fg(Color::Cyan)
    }

    /// Sets the foreground to white.
    pub fn white(self) -> Self {
        self.fg(Color::White)
    }

    /// Sets the foreground to bright black.
    pub fn bright_black(self) -> Self {
        self.fg(Color::BrightBlack)
    }

    /// Sets the foreground to bright red.
    pub fn bright_red(self) -> Self {
        self.fg(Color::BrightRed)
    }

    /// Sets the foreground to bright green.
    pub fn bright_green(self) -> Self {
        self.fg(Color::BrightGreen)
    }

    /// Sets the foreground to bright yellow.
    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BrightYellow)
    }

    /// Sets the foreground to bright blue.
    pub fn bright_blue(self) -> Self {
        self.fg(Color::BrightBlue)
    }

    /// Sets the foreground to bright magenta.
    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BrightMagenta)
    }

    /// Sets the foreground to bright cyan.
    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BrightCyan)
    }

    /// Sets the foreground to bright white.
    pub fn bright_white(self) -> Self {
        self.fg(Color::BrightWhite)
    }

    /// Sets the background to black.
    pub fn on_black(self) -> Self {
        self.bg(Color::Black)
    }

    /// Sets the background to red.
    pub fn on_red(self) -> Self {
        self.bg(Color::Red)
    }

    /// Sets the background to green.
    pub fn on_green(self) -> Self {
        self.bg(Color::Green)
    }

    /// Sets the background to yellow.
    pub fn on_yellow(self) -> Self {
        self.bg(Color::Yellow)
    }

    /// Sets the background to blue.
    pub fn on_blue(self) -> Self {
        self.bg(Color::Blue)
    }

    /// Sets the background to magenta.
    pub fn on_magenta(self) -> Self {
        self.bg(Color::Magenta)
    }

    /// Sets the background to cyan.
    pub fn on_cyan(self) -> Self {
        self.bg(Color::Cyan)
    }

    /// Sets the background to white.
    pub fn on_white(self) -> Self {
        self.bg(Color::White)
    }

    /// Sets the background to bright black.
    pub fn on_bright_black(self) -> Self {
        self.bg(Color::BrightBlack)
    }

    /// Sets the background to bright red.
    pub fn on_bright_red(self) -> Self {
        self.bg(Color::BrightRed)
    }

    /// Sets the background to bright green.
    pub fn on_bright_green(self) -> Self {
        self.bg(Color::BrightGreen)
    }

    /// Sets the background to bright yellow.
    pub fn on_bright_yellow(self) -> Self {
        self.bg(Color::BrightYellow)
    }

    /// Sets the background to bright blue.
    pub fn on_bright_blue(self) -> Self {
        self.bg(Color::BrightBlue)
    }

    /// Sets the background to bright magenta.
    pub fn on_bright_magenta(self) -> Self {
        self.bg(Color::BrightMagenta)
    }

    /// Sets the background to bright cyan.
    pub fn on_bright_cyan(self) -> Self {
        self.bg(Color::BrightCyan)
    }

    /// Sets the background to bright white.
    pub fn on_bright_white(self) -> Self {
        self.bg(Color::BrightWhite)
    }

    /// Enables bold text.
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Enables dim (faint) text.
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Enables italic text.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Enables underlined text.
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Enables strikethrough text.
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    /// Applies this style to the given text, returning a styled string.
    ///
    /// Respects `should_colorize()` — returns plain text if colors are disabled.
    pub fn paint(&self, text: &str) -> String {
        if !should_colorize() {
            return text.to_string();
        }
        self.force_paint(text)
    }

    /// Applies this style to the given text unconditionally, ignoring color detection.
    ///
    /// Useful for testing or when you want to force ANSI output.
    pub fn force_paint(&self, text: &str) -> String {
        let codes = self.ansi_codes();
        if codes.is_empty() {
            return text.to_string();
        }
        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), text)
    }

    /// Collects all ANSI SGR codes for this style.
    fn ansi_codes(&self) -> Vec<String> {
        let mut codes = Vec::new();
        if self.bold {
            codes.push("1".into());
        }
        if self.dim {
            codes.push("2".into());
        }
        if self.italic {
            codes.push("3".into());
        }
        if self.underline {
            codes.push("4".into());
        }
        if self.strikethrough {
            codes.push("9".into());
        }
        if let Some(ref fg) = self.fg {
            codes.push(fg.fg_code());
        }
        if let Some(ref bg) = self.bg {
            codes.push(bg.bg_code());
        }
        codes
    }
}

/// A styled string that applies ANSI formatting when displayed.
///
/// Created by the [`Colorize`] trait methods. Implements `Display` and `Colorize`
/// so styles can be chained: `"hello".red().bold().on_white()`.
#[derive(Debug, Clone)]
pub struct StyledString {
    text: String,
    style: Style,
}

impl StyledString {
    /// Returns the styled string with ANSI codes applied unconditionally.
    ///
    /// Ignores `should_colorize()` — useful for testing.
    pub fn force_styled(&self) -> String {
        self.style.force_paint(&self.text)
    }
}

impl fmt::Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if should_colorize() {
            write!(f, "{}", self.style.force_paint(&self.text))
        } else {
            write!(f, "{}", self.text)
        }
    }
}

/// Trait providing color and style methods on string types.
///
/// Implemented for `&str` and [`StyledString`], allowing method chaining:
///
/// ```rust
/// use philiprehberger_clicolor::Colorize;
///
/// let styled = "hello".red().bold().on_white();
/// ```
pub trait Colorize {
    /// Sets the foreground to black.
    fn black(self) -> StyledString;
    /// Sets the foreground to red.
    fn red(self) -> StyledString;
    /// Sets the foreground to green.
    fn green(self) -> StyledString;
    /// Sets the foreground to yellow.
    fn yellow(self) -> StyledString;
    /// Sets the foreground to blue.
    fn blue(self) -> StyledString;
    /// Sets the foreground to magenta.
    fn magenta(self) -> StyledString;
    /// Sets the foreground to cyan.
    fn cyan(self) -> StyledString;
    /// Sets the foreground to white.
    fn white(self) -> StyledString;
    /// Sets the foreground to bright black.
    fn bright_black(self) -> StyledString;
    /// Sets the foreground to bright red.
    fn bright_red(self) -> StyledString;
    /// Sets the foreground to bright green.
    fn bright_green(self) -> StyledString;
    /// Sets the foreground to bright yellow.
    fn bright_yellow(self) -> StyledString;
    /// Sets the foreground to bright blue.
    fn bright_blue(self) -> StyledString;
    /// Sets the foreground to bright magenta.
    fn bright_magenta(self) -> StyledString;
    /// Sets the foreground to bright cyan.
    fn bright_cyan(self) -> StyledString;
    /// Sets the foreground to bright white.
    fn bright_white(self) -> StyledString;
    /// Sets the background to black.
    fn on_black(self) -> StyledString;
    /// Sets the background to red.
    fn on_red(self) -> StyledString;
    /// Sets the background to green.
    fn on_green(self) -> StyledString;
    /// Sets the background to yellow.
    fn on_yellow(self) -> StyledString;
    /// Sets the background to blue.
    fn on_blue(self) -> StyledString;
    /// Sets the background to magenta.
    fn on_magenta(self) -> StyledString;
    /// Sets the background to cyan.
    fn on_cyan(self) -> StyledString;
    /// Sets the background to white.
    fn on_white(self) -> StyledString;
    /// Sets the background to bright black.
    fn on_bright_black(self) -> StyledString;
    /// Sets the background to bright red.
    fn on_bright_red(self) -> StyledString;
    /// Sets the background to bright green.
    fn on_bright_green(self) -> StyledString;
    /// Sets the background to bright yellow.
    fn on_bright_yellow(self) -> StyledString;
    /// Sets the background to bright blue.
    fn on_bright_blue(self) -> StyledString;
    /// Sets the background to bright magenta.
    fn on_bright_magenta(self) -> StyledString;
    /// Sets the background to bright cyan.
    fn on_bright_cyan(self) -> StyledString;
    /// Sets the background to bright white.
    fn on_bright_white(self) -> StyledString;
    /// Enables bold text.
    fn bold(self) -> StyledString;
    /// Enables dim (faint) text.
    fn dim(self) -> StyledString;
    /// Enables italic text.
    fn italic(self) -> StyledString;
    /// Enables underlined text.
    fn underline(self) -> StyledString;
    /// Enables strikethrough text.
    fn strikethrough(self) -> StyledString;
}

impl Colorize for &str {
    fn black(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Black) }
    }
    fn red(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Red) }
    }
    fn green(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Green) }
    }
    fn yellow(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Yellow) }
    }
    fn blue(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Blue) }
    }
    fn magenta(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Magenta) }
    }
    fn cyan(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::Cyan) }
    }
    fn white(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::White) }
    }
    fn bright_black(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightBlack) }
    }
    fn bright_red(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightRed) }
    }
    fn bright_green(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightGreen) }
    }
    fn bright_yellow(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightYellow) }
    }
    fn bright_blue(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightBlue) }
    }
    fn bright_magenta(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightMagenta) }
    }
    fn bright_cyan(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightCyan) }
    }
    fn bright_white(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().fg(Color::BrightWhite) }
    }
    fn on_black(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Black) }
    }
    fn on_red(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Red) }
    }
    fn on_green(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Green) }
    }
    fn on_yellow(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Yellow) }
    }
    fn on_blue(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Blue) }
    }
    fn on_magenta(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Magenta) }
    }
    fn on_cyan(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::Cyan) }
    }
    fn on_white(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::White) }
    }
    fn on_bright_black(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightBlack) }
    }
    fn on_bright_red(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightRed) }
    }
    fn on_bright_green(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightGreen) }
    }
    fn on_bright_yellow(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightYellow) }
    }
    fn on_bright_blue(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightBlue) }
    }
    fn on_bright_magenta(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightMagenta) }
    }
    fn on_bright_cyan(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightCyan) }
    }
    fn on_bright_white(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bg(Color::BrightWhite) }
    }
    fn bold(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().bold() }
    }
    fn dim(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().dim() }
    }
    fn italic(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().italic() }
    }
    fn underline(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().underline() }
    }
    fn strikethrough(self) -> StyledString {
        StyledString { text: self.to_string(), style: Style::new().strikethrough() }
    }
}

impl Colorize for StyledString {
    fn black(mut self) -> StyledString {
        self.style.fg = Some(Color::Black);
        self
    }
    fn red(mut self) -> StyledString {
        self.style.fg = Some(Color::Red);
        self
    }
    fn green(mut self) -> StyledString {
        self.style.fg = Some(Color::Green);
        self
    }
    fn yellow(mut self) -> StyledString {
        self.style.fg = Some(Color::Yellow);
        self
    }
    fn blue(mut self) -> StyledString {
        self.style.fg = Some(Color::Blue);
        self
    }
    fn magenta(mut self) -> StyledString {
        self.style.fg = Some(Color::Magenta);
        self
    }
    fn cyan(mut self) -> StyledString {
        self.style.fg = Some(Color::Cyan);
        self
    }
    fn white(mut self) -> StyledString {
        self.style.fg = Some(Color::White);
        self
    }
    fn bright_black(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightBlack);
        self
    }
    fn bright_red(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightRed);
        self
    }
    fn bright_green(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightGreen);
        self
    }
    fn bright_yellow(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightYellow);
        self
    }
    fn bright_blue(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightBlue);
        self
    }
    fn bright_magenta(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightMagenta);
        self
    }
    fn bright_cyan(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightCyan);
        self
    }
    fn bright_white(mut self) -> StyledString {
        self.style.fg = Some(Color::BrightWhite);
        self
    }
    fn on_black(mut self) -> StyledString {
        self.style.bg = Some(Color::Black);
        self
    }
    fn on_red(mut self) -> StyledString {
        self.style.bg = Some(Color::Red);
        self
    }
    fn on_green(mut self) -> StyledString {
        self.style.bg = Some(Color::Green);
        self
    }
    fn on_yellow(mut self) -> StyledString {
        self.style.bg = Some(Color::Yellow);
        self
    }
    fn on_blue(mut self) -> StyledString {
        self.style.bg = Some(Color::Blue);
        self
    }
    fn on_magenta(mut self) -> StyledString {
        self.style.bg = Some(Color::Magenta);
        self
    }
    fn on_cyan(mut self) -> StyledString {
        self.style.bg = Some(Color::Cyan);
        self
    }
    fn on_white(mut self) -> StyledString {
        self.style.bg = Some(Color::White);
        self
    }
    fn on_bright_black(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightBlack);
        self
    }
    fn on_bright_red(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightRed);
        self
    }
    fn on_bright_green(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightGreen);
        self
    }
    fn on_bright_yellow(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightYellow);
        self
    }
    fn on_bright_blue(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightBlue);
        self
    }
    fn on_bright_magenta(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightMagenta);
        self
    }
    fn on_bright_cyan(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightCyan);
        self
    }
    fn on_bright_white(mut self) -> StyledString {
        self.style.bg = Some(Color::BrightWhite);
        self
    }
    fn bold(mut self) -> StyledString {
        self.style.bold = true;
        self
    }
    fn dim(mut self) -> StyledString {
        self.style.dim = true;
        self
    }
    fn italic(mut self) -> StyledString {
        self.style.italic = true;
        self
    }
    fn underline(mut self) -> StyledString {
        self.style.underline = true;
        self
    }
    fn strikethrough(mut self) -> StyledString {
        self.style.strikethrough = true;
        self
    }
}

/// Determines whether terminal colors should be used.
///
/// Checks the following in order:
/// 1. `NO_COLOR` env var — if set (any value), returns `false`
/// 2. `CLICOLOR_FORCE` env var — if set to non-`"0"`, returns `true`
/// 3. `CLICOLOR` env var — if set to `"0"`, returns `false`
/// 4. TTY detection — returns `false` if stdout is not a terminal
/// 5. Default: returns `true`
///
/// The result is cached for the lifetime of the process.
pub fn should_colorize() -> bool {
    static RESULT: OnceLock<bool> = OnceLock::new();
    *RESULT.get_or_init(|| {
        #[cfg(windows)]
        enable_ansi_support();

        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }

        if let Ok(val) = std::env::var("CLICOLOR_FORCE") {
            if val != "0" {
                return true;
            }
        }

        if let Ok(val) = std::env::var("CLICOLOR") {
            if val == "0" {
                return false;
            }
        }

        if !std::io::stdout().is_terminal() {
            return false;
        }

        true
    })
}

/// Enables ANSI escape sequence processing on Windows 10+.
///
/// Calls `SetConsoleMode` with `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on the
/// stdout handle. If this fails (e.g., older Windows), colors may still work
/// in modern terminal emulators.
#[cfg(windows)]
fn enable_ansi_support() {
    const STD_OUTPUT_HANDLE: i32 = -11;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    extern "system" {
        fn GetStdHandle(nStdHandle: i32) -> *mut core::ffi::c_void;
        fn GetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, lpMode: *mut u32) -> i32;
        fn SetConsoleMode(hConsoleHandle: *mut core::ffi::c_void, dwMode: u32) -> i32;
    }

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.is_null() {
            return;
        }
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return;
        }
        let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

/// Removes all ANSI escape sequences from a string.
///
/// Handles `ESC[...m` style SGR sequences. Does not use regex.
///
/// ```rust
/// use philiprehberger_clicolor::{strip_ansi, Colorize};
///
/// let styled = "hello".red().force_styled();
/// assert_eq!(strip_ansi(&styled), "hello");
/// ```
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Check for CSI sequence: ESC[
            if let Some(next) = chars.next() {
                if next == '[' {
                    // Consume until we hit a letter (the terminator)
                    for ch in chars.by_ref() {
                        if ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                } else {
                    // Not a CSI sequence — don't include the ESC or the next char
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_fg_codes() {
        assert_eq!(Color::Red.fg_code(), "31");
        assert_eq!(Color::BrightCyan.fg_code(), "96");
        assert_eq!(Color::Ansi256(42).fg_code(), "38;5;42");
        assert_eq!(Color::Rgb(255, 128, 0).fg_code(), "38;2;255;128;0");
    }

    #[test]
    fn test_color_bg_codes() {
        assert_eq!(Color::Blue.bg_code(), "44");
        assert_eq!(Color::BrightWhite.bg_code(), "107");
        assert_eq!(Color::Ansi256(200).bg_code(), "48;5;200");
        assert_eq!(Color::Rgb(10, 20, 30).bg_code(), "48;2;10;20;30");
    }

    #[test]
    fn test_style_builder_force_paint() {
        let s = Style::new().red().bold().force_paint("hello");
        assert_eq!(s, "\x1b[1;31mhello\x1b[0m");
    }

    #[test]
    fn test_style_fg_only() {
        let s = Style::new().green().force_paint("ok");
        assert_eq!(s, "\x1b[32mok\x1b[0m");
    }

    #[test]
    fn test_style_bg_only() {
        let s = Style::new().on_blue().force_paint("bg");
        assert_eq!(s, "\x1b[44mbg\x1b[0m");
    }

    #[test]
    fn test_style_all_attributes() {
        let s = Style::new()
            .bold()
            .dim()
            .italic()
            .underline()
            .strikethrough()
            .force_paint("styled");
        assert_eq!(s, "\x1b[1;2;3;4;9mstyled\x1b[0m");
    }

    #[test]
    fn test_style_with_fg_and_bg() {
        let s = Style::new().red().on_white().force_paint("combo");
        assert_eq!(s, "\x1b[31;47mcombo\x1b[0m");
    }

    #[test]
    fn test_style_empty_no_codes() {
        let s = Style::new().force_paint("plain");
        assert_eq!(s, "plain");
    }

    #[test]
    fn test_colorize_red() {
        let s = "hello".red().force_styled();
        assert_eq!(s, "\x1b[31mhello\x1b[0m");
    }

    #[test]
    fn test_colorize_bold() {
        let s = "text".bold().force_styled();
        assert_eq!(s, "\x1b[1mtext\x1b[0m");
    }

    #[test]
    fn test_colorize_chaining() {
        let s = "hello".red().bold().on_white().force_styled();
        assert_eq!(s, "\x1b[1;31;47mhello\x1b[0m");
    }

    #[test]
    fn test_colorize_chain_overrides_fg() {
        let s = "test".red().blue().force_styled();
        assert_eq!(s, "\x1b[34mtest\x1b[0m");
    }

    #[test]
    fn test_strip_ansi_basic() {
        let input = "\x1b[31mhello\x1b[0m";
        assert_eq!(strip_ansi(input), "hello");
    }

    #[test]
    fn test_strip_ansi_multiple() {
        let input = "\x1b[1;31mhello\x1b[0m \x1b[32mworld\x1b[0m";
        assert_eq!(strip_ansi(input), "hello world");
    }

    #[test]
    fn test_strip_ansi_no_codes() {
        assert_eq!(strip_ansi("plain text"), "plain text");
    }

    #[test]
    fn test_strip_ansi_empty() {
        assert_eq!(strip_ansi(""), "");
    }

    #[test]
    fn test_strip_ansi_roundtrip() {
        let styled = "test".red().bold().force_styled();
        assert_eq!(strip_ansi(&styled), "test");
    }

    #[test]
    fn test_empty_string_colorize() {
        let s = "".red().force_styled();
        assert_eq!(s, "\x1b[31m\x1b[0m");
    }

    #[test]
    fn test_ansi256_colorize() {
        let s = Style::new().fg(Color::Ansi256(100)).force_paint("256");
        assert_eq!(s, "\x1b[38;5;100m256\x1b[0m");
    }

    #[test]
    fn test_rgb_colorize() {
        let s = Style::new().fg(Color::Rgb(255, 0, 128)).force_paint("rgb");
        assert_eq!(s, "\x1b[38;2;255;0;128mrgb\x1b[0m");
    }

    #[test]
    fn test_rgb_background() {
        let s = Style::new().bg(Color::Rgb(0, 128, 255)).force_paint("bg");
        assert_eq!(s, "\x1b[48;2;0;128;255mbg\x1b[0m");
    }

    #[test]
    fn test_bright_colors() {
        let s = "hi".bright_red().force_styled();
        assert_eq!(s, "\x1b[91mhi\x1b[0m");
    }

    #[test]
    fn test_on_bright_colors() {
        let s = "hi".on_bright_green().force_styled();
        assert_eq!(s, "\x1b[102mhi\x1b[0m");
    }

    #[test]
    fn test_all_standard_fg_colors() {
        assert_eq!("x".black().force_styled(), "\x1b[30mx\x1b[0m");
        assert_eq!("x".red().force_styled(), "\x1b[31mx\x1b[0m");
        assert_eq!("x".green().force_styled(), "\x1b[32mx\x1b[0m");
        assert_eq!("x".yellow().force_styled(), "\x1b[33mx\x1b[0m");
        assert_eq!("x".blue().force_styled(), "\x1b[34mx\x1b[0m");
        assert_eq!("x".magenta().force_styled(), "\x1b[35mx\x1b[0m");
        assert_eq!("x".cyan().force_styled(), "\x1b[36mx\x1b[0m");
        assert_eq!("x".white().force_styled(), "\x1b[37mx\x1b[0m");
    }

    #[test]
    fn test_strikethrough() {
        let s = "deleted".strikethrough().force_styled();
        assert_eq!(s, "\x1b[9mdeleted\x1b[0m");
    }

    #[test]
    fn test_dim_italic_underline() {
        let s = "fancy".dim().force_styled();
        assert_eq!(s, "\x1b[2mfancy\x1b[0m");
        let s = "fancy".italic().force_styled();
        assert_eq!(s, "\x1b[3mfancy\x1b[0m");
        let s = "fancy".underline().force_styled();
        assert_eq!(s, "\x1b[4mfancy\x1b[0m");
    }
}
