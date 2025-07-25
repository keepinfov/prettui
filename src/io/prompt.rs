//! Interactive prompts for confirmation, pattern-validated, and numeric input.
//!
//! # Examples
//!
//! ```rust
//! use prettui::io::prompt::{confirm, read_matching, read_number, ConfirmConfig, RegexConfig, NumberConfig};
//! use prettui::io::input::InputConfig;
//! use regex::Regex;
//!
//! fn main() -> std::io::Result<()> {
//!     let input_cfg = InputConfig::default();
//!
//!     // Confirmation prompt with default 'yes'
//!     let conf_cfg = ConfirmConfig { default: Some(true), case_sensitive: false };
//!     let proceed = confirm("Continue installation?", &conf_cfg, &input_cfg)?;
//!     println!("User chose to proceed: {}", proceed);
//!
//!     // Regex-validated input (email)
//!     let email_pattern = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
//!     let regex_cfg = RegexConfig {
//!         error_message: Some("Invalid email".into()),
//!         show_pattern: true,
//!         ..Default::default()
//!     };
//!     let email = read_matching("Enter your email", &email_pattern, &regex_cfg, &input_cfg)?;
//!     println!("Email: {}", email);
//!
//!     // Number input with range 1-10
//!     let num_cfg = NumberConfig { min: Some(1), max: Some(10), error_message: Some("Must be 1-10".into()), ..Default::default() };
//!     let count = read_number("Select count", &num_cfg, &input_cfg)?;
//!     println!("Count: {}", count);
//!
//!     Ok(())
//! }
//!```

use crate::color::Stylize;
use crate::io::input::InputConfig;
use regex::Regex;
use std::io::{self, Write};

/// Configuration for confirmation prompts (yes/no).
#[derive(Debug, Clone)]
pub struct ConfirmConfig {
    /// Default choice when user presses Enter without input.
    /// - Some(true): default to yes ([Y/n])
    /// - Some(false): default to no  ([y/N])
    /// - None: no default ([y/n])
    pub default: Option<bool>,
    /// If true, 'yes' and 'no' must match case literally.
    pub case_sensitive: bool,
}

impl Default for ConfirmConfig {
    fn default() -> Self {
        Self {
            default: None,
            case_sensitive: false,
        }
    }
}

/// Configuration for regex-validated input.
#[derive(Debug, Clone)]
pub struct RegexConfig {
    /// Optional error message shown on mismatch.
    pub error_message: Option<String>,
    /// Maximum retry attempts; None = unlimited.
    pub max_attempts: Option<usize>,
    /// If true, show the regex pattern in the prompt.
    pub show_pattern: bool,
}

impl Default for RegexConfig {
    fn default() -> Self {
        Self {
            error_message: None,
            max_attempts: Some(3),
            show_pattern: false,
        }
    }
}

/// Configuration for numeric input prompts.
#[derive(Debug, Clone)]
pub struct NumberConfig {
    /// Minimum allowed value.
    pub min: Option<i64>,
    /// Maximum allowed value.
    pub max: Option<i64>,
    /// Optional error message on invalid input or out of range.
    pub error_message: Option<String>,
    /// Maximum retry attempts; None = unlimited.
    pub max_attempts: Option<usize>,
}

impl Default for NumberConfig {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
            error_message: None,
            max_attempts: Some(3),
        }
    }
}

/// Ask user a yes/no question and return true for yes, false for no.
///
/// # Errors
/// Returns io::Error if stdin/stdout fail.
pub fn confirm(message: &str, cfg: &ConfirmConfig, input_cfg: &InputConfig) -> io::Result<bool> {
    // Build indicator suffix
    let indicator = match cfg.default {
        Some(true) => "[Y/n]",
        Some(false) => "[y/N]",
        None => "[y/n]",
    };

    loop {
        // Print prompt
        let prompt = format!("{} {} {}: ", input_cfg.prefix.trim(), message, indicator);
        print_styled(&prompt, input_cfg);
        io::stdout().flush()?;

        // Read
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let input = line.trim();

        // Default
        if input.is_empty() {
            if let Some(def) = cfg.default {
                return Ok(def);
            }
        }

        // Normalize
        let val = if cfg.case_sensitive {
            input.to_string()
        } else {
            input.to_lowercase()
        };

        // Match
        match val.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => {
                print_error("Please enter 'y' or 'n'", input_cfg);
            }
        }
    }
}

/// Read a line matching a regex pattern.
///
/// # Errors
/// Returns Err after max_attempts or IO errors.
pub fn read_matching(
    message: &str,
    pattern: &Regex,
    cfg: &RegexConfig,
    input_cfg: &InputConfig,
) -> io::Result<String> {
    let mut attempts = 0;
    loop {
        // Build prompt
        let hint = if cfg.show_pattern {
            format!(" (pattern: {})", pattern.as_str())
        } else {
            String::new()
        };
        let prompt = format!("{} {}{}: ", input_cfg.prefix.trim(), message, hint);
        print_styled(&prompt, input_cfg);
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let input = line.trim().to_string();

        if pattern.is_match(&input) {
            return Ok(input);
        }

        attempts += 1;
        if let Some(max) = cfg.max_attempts {
            if attempts >= max {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    cfg.error_message
                        .clone()
                        .unwrap_or_else(|| "Invalid input".into()),
                ));
            }
        }
        print_error(
            cfg.error_message
                .as_deref()
                .unwrap_or("Input does not match pattern"),
            input_cfg,
        );
    }
}

/// Read an integer within optional bounds.
///
/// # Errors
/// Returns Err after max_attempts or IO errors.
pub fn read_number(message: &str, cfg: &NumberConfig, input_cfg: &InputConfig) -> io::Result<i64> {
    let mut attempts = 0;
    loop {
        // Build range hint
        let range = match (cfg.min, cfg.max) {
            (Some(min), Some(max)) => format!(" ({}-{})", min, max),
            (Some(min), None) => format!(" (>= {})", min),
            (None, Some(max)) => format!(" (<= {})", max),
            (None, None) => "".to_string(),
        };
        let prompt = format!("{} {}{}: ", input_cfg.prefix.trim(), message, range);
        print_styled(&prompt, input_cfg);
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let text = line.trim();
        match text.parse::<i64>() {
            Ok(num) if cfg.min.map_or(true, |m| num >= m) && cfg.max.map_or(true, |m| num <= m) => {
                return Ok(num);
            }
            _ => {
                attempts += 1;
                if let Some(max) = cfg.max_attempts {
                    if attempts >= max {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            cfg.error_message
                                .clone()
                                .unwrap_or_else(|| "Invalid number input".into()),
                        ));
                    }
                }
                let msg = cfg.error_message.as_deref().unwrap_or("Invalid number");
                print_error(msg, input_cfg);
            }
        }
    }
}

/// Print a prompt or text using InputConfig styling.
fn print_styled(text: &str, cfg: &InputConfig) {
    let mut styled = String::new();
    // indent
    if cfg.indent_level > 0 {
        styled.push_str(&" ".repeat(cfg.indent_level));
    }
    // prefix
    styled.push_str(&cfg.prefix);
    // finally the prompt text in prompt_color
    styled.push_str(text);
    print!("{}", styled.with(cfg.prompt_color.into()));
}

/// Print an error message to stderr using input_text_color.
fn print_error(message: &str, cfg: &InputConfig) {
    eprintln!(
        "{}",
        format!("Error: {}", message).with(cfg.input_text_color.into())
    );
}
