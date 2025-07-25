use prettui::io::InputConfig;
use prettui::io::{ConfirmConfig, NumberConfig, RegexConfig, confirm, read_matching, read_number};
use regex::Regex;

fn main() -> std::io::Result<()> {
    let input_cfg = InputConfig::default();

    // Confirmation prompt with default 'yes'
    let conf_cfg = ConfirmConfig {
        default: Some(true),
        case_sensitive: false,
    };
    let proceed = confirm("Continue installation?", &conf_cfg, &input_cfg)?;
    println!("User chose to proceed: {}", proceed);

    let conf_cfg = ConfirmConfig {
        default: None,
        case_sensitive: true,
    };
    let proceed = confirm("Continue sensitive installation?", &conf_cfg, &input_cfg)?;
    println!("User chose to proceed: {}", proceed);

    // Regex-validated input (email)
    let email_pattern = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    let regex_cfg = RegexConfig {
        error_message: Some("Invalid email".into()),
        show_pattern: false,
        ..Default::default()
    };
    let email = read_matching("Enter your email", &email_pattern, &regex_cfg, &input_cfg)?;
    println!("Email: {}", email);

    // Number input with range 1-10
    let num_cfg = NumberConfig {
        min: Some(1),
        max: Some(10),
        error_message: Some("Must be 1-10".into()),
        ..Default::default()
    };
    let count = read_number("Select count", &num_cfg, &input_cfg)?;
    println!("Count: {}", count);

    Ok(())
}
