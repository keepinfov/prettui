use crate::color::Color;
use crossterm::{
    ExecutableCommand,
    style::{Print, PrintStyledContent, ResetColor, SetForegroundColor, Stylize},
};
use std::io::{self, Write};

/// Configuration for reading input
#[derive(Debug, Clone)]
pub struct InputConfig {
    pub prefix: String,
    pub prompt: String,
    pub prefix_color: Color,
    pub prompt_color: Color,
    pub input_text_color: Color,
    pub max_chars_per_line: usize,
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

/// Reads a line from stdin using the given configuration
pub fn read_input(cfg: &InputConfig) -> io::Result<String> {
    let mut stdout = io::stdout();

    if cfg.indent_level > 0 {
        let indent = " ".repeat(cfg.indent_level);
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

/// Wraps text into lines of at most `max_width` chars
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
