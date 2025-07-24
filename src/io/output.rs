use crate::color::Color;
use crate::io::input::wrap_text;
use crossterm::{
    ExecutableCommand,
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{self, Write};

/// Configuration for printing output (logging optional)
#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub prefix: String,
    pub prefix_color: Color,
    pub text_color: Color,
    /// Optional log level label (e.g., INFO, WARN)
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

/// Prints a message to stdout using the given configuration
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
