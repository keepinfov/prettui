// src/io/output.rs
use crate::color::Color;
use crate::io::input::wrap_text;
use crossterm::{
    ExecutableCommand,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{self, Write};

/// Configuration for printing styled output to the console.
///
/// This struct controls how messages are formatted:
/// - `prefix`: text shown before each message (e.g., a label)
/// - `prefix_color`: color applied to the prefix and log level tag
/// - `text_color`: color applied to the message body
/// - `log_level`: optional tag (e.g., INFO, WARN) displayed before the message
/// - `indent_level`: number of spaces to indent each line
/// - `max_chars_per_line`: maximum width before wrapping occurs
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Text to display before each message (e.g., module name).
    pub prefix: String,
    /// Color for the prefix and log level tag.
    pub prefix_color: Color,
    /// Color for the main message text.
    pub text_color: Color,
    /// Optional log level label (e.g., INFO, WARN).
    pub log_level: Option<String>,
    /// Number of spaces to indent each line of output.
    pub indent_level: usize,
    /// Maximum number of characters per line before wrapping.
    pub max_chars_per_line: usize,
}

impl Default for OutputConfig {
    /// Returns a default `OutputConfig` with:
    /// - empty prefix
    /// - green prefix color
    /// - white text color
    /// - no log level tag
    /// - no indentation
    /// - line wrap at 80 characters
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
///
/// # Examples
/// ```no_run
/// let mut cfg = OutputConfig::default();
/// cfg.prefix = String::from("[MyApp]");
/// cfg.log_level = Some(String::from("INFO"));
/// write_output(&cfg, "Application started").unwrap();
/// ```
pub fn write_output(cfg: &OutputConfig, message: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let wrapped = wrap_text(message, cfg.max_chars_per_line);

    for line in wrapped {
        // Apply indentation
        if cfg.indent_level > 0 {
            let indent = " ".repeat(cfg.indent_level);
            stdout.execute(Print(indent.clone()))?;
        }
        // Print prefix if provided
        if !cfg.prefix.is_empty() {
            stdout.execute(PrintStyledContent(
                cfg.prefix.clone().with(cfg.prefix_color.into()),
            ))?;
        }
        // Print log level tag if present
        if let Some(ref level) = cfg.log_level {
            stdout.execute(PrintStyledContent(
                format!("[{}] ", level).with(cfg.prefix_color.into()),
            ))?;
        }
        // Print message text
        stdout.execute(PrintStyledContent(line.with(cfg.text_color.into())))?;
        // Newline
        stdout.execute(Print("\n"))?;
    }
    // Ensure all output is flushed
    stdout.flush()
}
