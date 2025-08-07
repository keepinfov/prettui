//! A terminal input utility for styled prompts and text wrapping.
//!
//! This module provides:
//! - `InputConfig`: customize prompt appearance, colors, wrapping, and indentation.
//! - `read_input`: read a single line from stdin with styling and EOF handling.
//! - `read_multiline_input`: read multiple lines until a terminator is entered, showing prompt only once.
//! - `wrap_text`: word-wrap long strings into lines of a specified width.
//! - `read_secret_input`: read a secret line of input without echoing to the terminal.
//!
//! # Full Example
//!
//! ```rust
//! use prettui::io::input::{InputConfig, read_input, read_multiline_input};
//! use prettui::color::Color;
//!
//! fn main() -> std::io::Result<()> {
//!     let cfg = InputConfig {
//!         prefix: String::from("[TEST] "),
//!         prompt: String::from(">> "),
//!         prefix_color: Color::Magenta,
//!         prompt_color: Color::Cyan,
//!         input_text_color: Color::White,
//!         max_chars_per_line: 80,
//!         indent_level: 2,
//!     };
//!
//!     // Single-line input
//!     let subject = read_input(&cfg)?;
//!     println!("Subject: {}", subject);
//!
//!     // Multiline body: prompt shown once, end with '.' line
//!     let body = read_multiline_input(&cfg, ".")?;
//!     println!("Body:\n{}", body);
//!
//!     // Secret input
//!     let body = read_secret_input(&cfg)?;
//!     println!("Secret:\n{}", body);
//!
//!     Ok(())
//! }
//!```

use crate::color::Color;
use crossterm::{
    style::{Print, PrintStyledContent, ResetColor, SetForegroundColor, Stylize},
    ExecutableCommand,
};
use std::io::{self, BufRead, BufReader, Write};

/// Configuration for reading input from the user.
#[derive(Debug, Clone)]
pub struct InputConfig {
    /// Text to display before the prompt (e.g., a label).
    pub prefix: String,
    /// The prompt string shown before reading input (e.g., `"→ "`).
    pub prompt: String,
    /// Color for the prefix text.
    pub prefix_color: Color,
    /// Color for the prompt text.
    pub prompt_color: Color,
    /// Color for the user’s input text.
    pub input_text_color: Color,
    /// Maximum number of characters per line before wrapping.
    pub max_chars_per_line: usize,
    /// Number of spaces to indent before printing the prompt.
    pub indent_level: usize,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            prefix: String::new(),
            prompt: String::from(">> "),
            prefix_color: Color::Blue,
            prompt_color: Color::White,
            input_text_color: Color::White,
            max_chars_per_line: 80,
            indent_level: 0,
        }
    }
}

/// Reads a line of input from stdin using the provided configuration.
pub fn read_input(cfg: &InputConfig) -> io::Result<String> {
    let mut stdout = io::stdout();
    // Indentation
    if cfg.indent_level > 0 {
        let indent = " ".repeat(cfg.indent_level);
        stdout.execute(Print(indent))?;
    }
    // Prefix
    if !cfg.prefix.is_empty() {
        stdout.execute(PrintStyledContent(
            cfg.prefix.clone().with(cfg.prefix_color.into()),
        ))?;
    }
    // Prompt
    stdout.execute(PrintStyledContent(
        cfg.prompt.clone().with(cfg.prompt_color.into()),
    ))?;
    stdout.flush()?;
    // Read
    stdout.execute(SetForegroundColor(cfg.input_text_color.into()))?;
    let mut buf = String::new();
    let bytes = io::stdin().read_line(&mut buf)?;
    stdout.execute(ResetColor)?;
    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "EOF while reading input",
        ));
    }
    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    Ok(buf)
}

/// Reads multiple lines of input until the `terminator` line is entered.
/// Displays the prompt only once; subsequent lines show no prompt.
pub fn read_multiline_input(cfg: &InputConfig, terminator: &str) -> io::Result<String> {
    let mut stdout = io::stdout();
    // Print initial prompt line
    if cfg.indent_level > 0 {
        let indent = " ".repeat(cfg.indent_level);
        stdout.execute(Print(indent.clone()))?;
    }
    if !cfg.prefix.is_empty() {
        stdout.execute(PrintStyledContent(
            cfg.prefix.clone().with(cfg.prefix_color.into()),
        ))?;
    }
    let header = format!("{} (end with '{}' on new line)\n", cfg.prompt, terminator);
    stdout.execute(PrintStyledContent(header.with(cfg.prompt_color.into())))?;
    stdout.flush()?;

    // Read raw lines without prompt
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut lines = Vec::new();
    for line in reader.lines() {
        let input = line?;
        if input.trim() == terminator {
            break;
        }
        lines.push(input);
    }
    Ok(lines.join("\n"))
}

/// Wraps the given text into multiple lines, none exceeding `max_width` characters.
pub(crate) fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > max_width {
            lines.push(current.trim_end().to_string());
            current.clear();
        }
        current.push_str(word);
        current.push(' ');
    }
    if !current.is_empty() {
        lines.push(current.trim_end().to_string());
    }
    lines
}

/// Reads a secret line of input without echoing to the terminal.
///
/// This function:
/// 1. Applies indentation and prints the styled prompt once.
/// 2. Enables raw mode to suppress echo.
/// 3. Reads key events until Enter is pressed, collecting characters.
/// 4. Disables raw mode and moves to a new line.
/// 5. Returns the entered string (without newline).
///
/// # Errors
/// Returns an `io::Error` if terminal manipulation or reading fails.

pub fn read_secret_input(cfg: &InputConfig) -> io::Result<String> {
    use crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
        style::{Print, PrintStyledContent},
        terminal::{disable_raw_mode, enable_raw_mode},
        ExecutableCommand,
    };
    use std::io::{self, Write};

    // Print styled prompt
    let mut stdout = io::stdout();
    if cfg.indent_level > 0 {
        let indent = " ".repeat(cfg.indent_level);
        // <-- use Print for a plain string
        stdout.execute(Print(indent))?;
    }
    if !cfg.prefix.is_empty() {
        stdout.execute(PrintStyledContent(
            cfg.prefix.clone().with(cfg.prefix_color.into()),
        ))?;
    }
    stdout.execute(PrintStyledContent(
        cfg.prompt.clone().with(cfg.prompt_color.into()),
    ))?;
    stdout.flush()?;

    // Enable raw mode (suppress echo)
    enable_raw_mode()?;
    let mut input = String::new();
    loop {
        // Read next key event, ignoring the extra fields
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            // Ctrl+C -> cancel input
            if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
                disable_raw_mode()?;
                println!();
                return Err(io::Error::new(io::ErrorKind::Interrupted, "Input canceled"));
            }
            match code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            }
        }
    }
    // Restore terminal
    disable_raw_mode()?;
    // Move to next line
    println!();
    Ok(input)
}
