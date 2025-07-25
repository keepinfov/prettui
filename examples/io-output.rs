use prettui::color::Color;
use prettui::io::output::{OutputConfig, write_output};

fn main() -> std::io::Result<()> {
    // Example 1: simple message with default config
    let cfg1 = OutputConfig::default();
    write_output(
        &cfg1,
        "Hello, world! This is a long message that will wrap around at the default width of 80 characters to demonstrate text wrapping functionality.",
    )?;

    // Example 2: with prefix and indent
    let mut cfg2 = OutputConfig::default();
    cfg2.prefix = String::from("[App] ");
    cfg2.prefix_color = Color::Blue;
    cfg2.indent_level = 4;
    write_output(&cfg2, "Indented message with a prefix.")?;

    // Example 3: with log level tag and custom text color
    let mut cfg3 = OutputConfig::default();
    cfg3.log_level = Some(String::from("INFO"));
    cfg3.text_color = Color::Green;
    cfg3.prefix = String::from("[Server] ");
    cfg3.prefix_color = Color::Magenta;
    write_output(&cfg3, "Server started on port 8080.")?;

    Ok(())
}
