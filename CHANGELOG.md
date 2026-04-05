# Changelog

## [0.2.0] - 2026-04-05

- Add `gradient()` and `force_gradient()` functions for RGB color gradients across text
- Linearly interpolates R, G, B channels per character between two RGB colors

## 0.1.2 (2026-03-31)

- Standardize README to 3-badge format with emoji Support section
- Update CI checkout action to v5 for Node.js 24 compatibility

## 0.1.1 (2026-03-27)

- Add GitHub issue templates, PR template, and dependabot configuration
- Update README badges and add Support section

## 0.1.0 (2026-03-19)

- Initial release
- Colorize trait for &str with 16 standard colors and bright variants
- Text styles: bold, dim, italic, underline, strikethrough
- Background colors
- Style struct for reusable style definitions
- NO_COLOR, CLICOLOR, CLICOLOR_FORCE environment variable support
- TTY detection with automatic color disabling for piped output
- Windows 10+ ANSI support
- strip_ansi() utility
