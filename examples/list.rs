use prettui::prelude::*;

fn main() -> anyhow::Result<()> {
    let items: Vec<String> = (1..=100).map(|i| format!("Item {}", i)).collect();
    let config = ListConfig::default()
        .items_per_row(1)
        .rows_per_page(10)
        .cell_width(30)
        .normal_fg(Color::DarkGrey)
        .highlight_fg(Color::Green);

    println!("Example of using");
    println!(
        "Use arrows/PageUp/PageDown to navigate, type digits, Backspace to delete, Enter to confirm, Esc to cancel."
    );
    if let Some(idx) = choose_from_list(&items, &config)? {
        println!("You chose: {}", items[idx]);
    } else {
        println!("Selection cancelled.");
    }

    Ok(())
}
