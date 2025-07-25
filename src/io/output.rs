//! Configuration for printing styled output to the console.
//!
//! This struct controls how messages are formatted:
//! - `prefix`: text shown before each message (e.g., a label)
//! - `prefix_color`: color applied to the prefix and log level tag
//! - `text_color`: color applied to the message body
//! - `log_level`: optional tag (e.g., INFO, WARN) displayed before the message
//! - `indent_level`: number of spaces to indent each line
//! - `max_chars_per_line`: maximum width before wrapping occurs
//!
//! # Full Example
//!
//! ```rust
//! use prettui::io::output::{OutputConfig, write_output};
//! use prettui::color::Color;
//!
//! fn main() -> std::io::Result<()> {
//!     // Example 1: simple message with default config
//!     let mut cfg1 = OutputConfig::default();
//!     write_output(&cfg1, "Hello, world! This is a long message that will wrap around at the default width of 80 characters to demonstrate text wrapping functionality.")?;
//!
//!     // Example 2: with prefix and indent
//!     let mut cfg2 = OutputConfig::default();
//!     cfg2.prefix = String::from("[App] ");
//!     cfg2.prefix_color = Color::Blue;
//!     cfg2.indent_level = 4;
//!     write_output(&cfg2, "Indented message with a prefix.")?;
//!
//!     // Example 3: with log level tag and custom text color
//!     let mut cfg3 = OutputConfig::default();
//!     cfg3.log_level = Some(String::from("INFO"));
//!     cfg3.text_color = Color::Green;
//!     cfg3.prefix = String::from("[Server] ");
//!     cfg3.prefix_color = Color::Magenta;
//!     write_output(&cfg3, "Server started on port 8080.")?;
//!
//!     Ok(())
//! }
//! ```

use crate::color::Color;
use crate::io::input::wrap_text;
use crossterm::{
    ExecutableCommand,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub prefix: String,
    pub prefix_color: Color,
    pub text_color: Color,
    pub log_level: Option<String>,
    pub indent_level: usize,
    pub max_chars_per_line: usize,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            prefix_color: Color::Green,
            text_color: Color::White,
            log_level: None,
            indent_level: 0,
            max_chars_per_line: 80,
        }
    }
}

/// Writes a styled and optionally wrapped message to stdout.
///
/// This function:
/// 1. Wraps the message at `cfg.max_chars_per_line` using `wrap_text`.
/// 2. Iterates over each line and applies:
///    - indentation (if `cfg.indent_level > 0`).
///    - prefix (if non-empty), styled with `cfg.prefix_color`.
///    - log level tag (if `cfg.log_level` is `Some`), styled with `cfg.prefix_color`.
///    - message text, styled with `cfg.text_color`.
/// 3. Prints a newline after each line and flushes stdout at the end.
///
/// # Errors
/// Returns an `io::Error` if writing to stdout fails.
pub fn write_output(cfg: &OutputConfig, message: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let wrapped = wrap_text(message, cfg.max_chars_per_line);

    for line in wrapped {
        if cfg.indent_level > 0 {
            let indent = " ".repeat(cfg.indent_level);
            stdout.execute(Print(indent.clone()))?;
        }
        if !cfg.prefix.is_empty() {
            stdout.execute(PrintStyledContent(
                cfg.prefix.clone().with(cfg.prefix_color.into()),
            ))?;
        }
        if let Some(ref level) = cfg.log_level {
            stdout.execute(PrintStyledContent(
                format!("[{}] ", level).with(cfg.prefix_color.into()),
            ))?;
        }
        stdout.execute(PrintStyledContent(line.with(cfg.text_color.into())))?;
        stdout.execute(Print("\n"))?;
    }
    stdout.flush()
}
