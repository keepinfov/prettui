use crate::color::Color;
use crossterm::{
    ExecutableCommand,
    style::{Print, PrintStyledContent, ResetColor, SetForegroundColor, Stylize},
};
use std::io::{self, Write};

/// Configuration for reading input from the user.
///
/// This struct defines how the prompt is displayed—its prefix,
/// prompt string, colors, maximum line width, and indentation level.
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
    /// Returns a default `InputConfig` with:
    /// - empty prefix
    /// - prompt `">> "`
    /// - blue prefix color
    /// - white prompt and input text colors
    /// - line wrap at 80 characters
    /// - no indentation
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
///
/// This function:
/// 1. Applies indentation if configured.
/// 2. Prints the prefix and prompt with styled colors.
/// 3. Flushes stdout to ensure prompt appears immediately.
/// 4. Sets the input text color.
/// 5. Reads a line from stdin, handling EOF as an error.
/// 6. Resets the terminal color and trims trailing newline/carriage returns.
///
/// # Errors
/// Returns an `io::Error` if:
/// - Writing to stdout fails.
/// - Reading from stdin hits EOF without any input.
///
/// # Examples
/// ```no_run
/// let cfg = InputConfig::default();
/// let name = read_input(&cfg).expect("Failed to read input");
/// println!("Hello, {}!", name);
/// ```
pub fn read_input(cfg: &InputConfig) -> io::Result<String> {
    let mut stdout = io::stdout();

    // Apply indentation
    if cfg.indent_level > 0 {
        let indent = " ".repeat(cfg.indent_level);
        stdout.execute(Print(indent))?;
    }

    // Print prefix if any
    if !cfg.prefix.is_empty() {
        stdout.execute(PrintStyledContent(
            cfg.prefix.clone().with(cfg.prefix_color.into()),
        ))?;
    }

    // Print prompt
    stdout.execute(PrintStyledContent(
        cfg.prompt.clone().with(cfg.prompt_color.into()),
    ))?;
    stdout.flush()?;

    // Set color for user input
    stdout.execute(SetForegroundColor(cfg.input_text_color.into()))?;
    let mut buf = String::new();
    let bytes = io::stdin().read_line(&mut buf)?;
    stdout.execute(ResetColor)?;

    // Handle EOF
    if bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "EOF while reading input",
        ));
    }

    // Trim newline and carriage return
    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }

    Ok(buf)
}

/// Wraps the given text into multiple lines, none exceeding `max_width` characters.
///
/// Splits on whitespace and ensures words are not broken across lines. Remaining text
/// is trimmed and returned as a vector of lines.
///
/// # Examples
/// ```
/// let lines = wrap_text("This is a long sentence.", 10);
/// assert_eq!(lines, vec!["This is a", "long", "sentence."]);
/// ```
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
