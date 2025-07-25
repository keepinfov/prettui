use prettui::color::Color;
use prettui::io::input::{InputConfig, read_input, read_multiline_input, read_secret_input};

fn main() -> std::io::Result<()> {
    let cfg = InputConfig {
        prefix: String::from("[Email] "),
        prompt: String::from(">> "),
        prefix_color: Color::Magenta,
        prompt_color: Color::Cyan,
        input_text_color: Color::White,
        max_chars_per_line: 80,
        indent_level: 2,
    };

    // Single-line input
    let subject = read_input(&cfg)?;
    println!("Subject: {}", subject);

    // Multiline body: prompt shown once, end with '.' line
    let body = read_multiline_input(&cfg, ".")?;
    println!("Body:\n{}", body);

    let secret = read_secret_input(&cfg)?;
    println!("Secret: {}", secret);

    Ok(())
}
