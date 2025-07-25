# prettui

A terminal-based library to build beautiful and interactive command-line user interfaces in Rust.

## Features

* **Configurable Lists**: Customize items per row, rows per page, and cell width.
* **Arrow & Page Navigation**: Navigate lists using arrow keys, PageUp/PageDown.
* **Numeric Input**: Real-time, multi-digit numeric list input with live feedback.
* **Custom Colors**: Easily style prefixes, prompts, and output text with configurable colors.
* **Input and Output Utilities**: Flexible input prompts and styled, wrapped console output.

## Installation

Add `prettui` to your `Cargo.toml`:

```toml
[dependencies]
prettui = "0.3.0"
```

Or using command:

```
cargo add prettui
```

Then import the prelude in your code:

```rust
use prettui::prelude::*;
```

## Example

```rust
use prettui::prelude::*;

fn main() -> anyhow::Result<()> {
    // Prepare a list of 100 items
    let items: Vec<String> = (1..=100).map(|i| format!("Item {}", i)).collect();

    // Configure the list display
    let config = ListConfig::default()
        .items_per_row(1)
        .rows_per_page(10)
        .cell_width(30)
        .normal_fg(Color::DarkGrey)
        .highlight_fg(Color::Green);

    println!("Example of using prettui list chooser");
    println!(
        "Use arrows/PageUp/PageDown to navigate, type digits, Backspace to delete, Enter to confirm, Esc to cancel."
    );

    // Let the user choose an index or cancel
    if let Some(idx) = choose_from_list(&items, &config)? {
        println!("You chose: {}", items[idx]);
    } else {
        println!("Selection cancelled.");
    }

    // Prompt for user input
    let ic = InputConfig {
        input_text_color: Color::Blue,
        ..Default::default()
    };
    let name = read_input(&ic)?;

    // Print styled output without log level
    let oc = OutputConfig {
        log_level: None,
        ..Default::default()
    };
    write_output(&oc, &format!("Hello, {}!", name))?;

    // Print styled output with a log level tag
    let oc2 = OutputConfig {
        log_level: Some("DEBUG".into()),
        ..Default::default()
    };
    write_output(&oc2, "This is a debug message.")?;

    Ok(())
}
```

## Modules

* **`color`**: Defines the `Color` enum and conversions to terminal color types.
* **`io::input`**: Utilities for reading user input, including configurable prompts and text wrapping.
* **`io::output`**: Functions for printing styled and wrapped text, with optional log level tags.
* **`list`**: Interactive list chooser with navigation and numeric input support.

## Prelude

For convenience, `prettui::prelude` re-exports the most commonly used types and functions:

```rust
pub use crate::color::*;
pub use crate::io::*;
pub use crate::list::*;
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
