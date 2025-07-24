use prettui::prelude::*;

fn main() -> anyhow::Result<()> {
    let ic = InputConfig {
        input_text_color: Color::Blue,
        ..Default::default()
    };
    let name = read_input(&ic)?;
    // Without log level:
    let oc = OutputConfig {
        log_level: None,
        ..Default::default()
    };
    write_output(&oc, &format!("Hello, {}!", name))?;
    // With log level:
    let oc2 = OutputConfig {
        log_level: Some("DEBUG".into()),
        ..Default::default()
    };
    write_output(&oc2, "This is a debug message.")?;

    Ok(())
}
