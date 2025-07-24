//! A terminal-based lib to make pretty command-line ui -> prettui
//!
//! # Features
//!
//! - Configurable list with items per row, rows per page, and cell width
//! - Arrow and page list navigation
//! - Real-time multi-digit numeric list input with live feedback
//! - Customizable colors
//!
//! # Example
//!
//! ```rust
//! use prettui::prelude::*;
//!
//! fn main() -> anyhow::Result<()> {
//!     let items: Vec<String> = (1..=100).map(|i| format!("Item {}", i)).collect();
//!     let config = ListConfig::default()
//!         .items_per_row(1)
//!         .rows_per_page(10)
//!         .cell_width(30)
//!         .normal_fg(Color::DarkGrey)
//!         .highlight_fg(Color::Green);
//!
//!     println!("Example of using");
//!     println!(
//!         "Use arrows/PageUp/PageDown to navigate, type digits, Backspace to delete, Enter to confirm, Esc to cancel."
//!     );
//!     if let Some(idx) = choose_from_list(&items, &config)? {
//!         println!("You chose: {}", items[idx]);
//!     } else {
//!         println!("Selection cancelled.");
//!     }
//!     Ok(())
//! }
//! ```

pub mod color;
pub mod list;

pub mod prelude {
    pub use super::color::*;
    pub use super::list::*;
}
