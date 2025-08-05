use anyhow::Result;
use crossterm::{
    cursor::{position, MoveTo},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::io::{stdout, Write};

use crate::color::Color;

/// Configuration for layout and visual behavior of the list selection UI.
#[derive(Debug, Clone)]
pub struct ListConfig {
    /// Number of items displayed per row.
    pub items_per_row: usize,
    /// Number of rows displayed per page.
    pub rows_per_page: usize,
    /// Width of each cell in characters.
    pub cell_width: u16,
    /// Foreground color for non-highlighted items.
    pub normal_fg: Color,
    /// Foreground color for the highlighted (selected) item.
    pub highlight_fg: Color,
}

impl Default for ListConfig {
    /// Returns a default configuration with:
    /// - 3 items per row
    /// - 5 rows per page
    /// - 20-character-wide cells
    /// - white text for normal items
    /// - yellow text for selected items
    fn default() -> Self {
        Self {
            items_per_row: 3,
            rows_per_page: 5,
            cell_width: 20,
            normal_fg: Color::White,
            highlight_fg: Color::Yellow,
        }
    }
}

impl ListConfig {
    /// Set number of items per row.
    pub fn items_per_row(mut self, val: usize) -> Self {
        self.items_per_row = val;
        self
    }

    /// Set number of rows per page.
    pub fn rows_per_page(mut self, val: usize) -> Self {
        self.rows_per_page = val;
        self
    }

    /// Set cell width for rendering items.
    pub fn cell_width(mut self, val: u16) -> Self {
        self.cell_width = val;
        self
    }

    /// Set the normal foreground color.
    pub fn normal_fg(mut self, color: Color) -> Self {
        self.normal_fg = color;
        self
    }

    /// Set the highlight foreground color.
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = color;
        self
    }
}

/// Display a selectable, paginated list in the terminal, with keyboard navigation and numeric input.
///
/// # Parameters
/// - `items`: A slice of items to display. Each item must implement `ToString`.
/// - `config`: A reference to a `ListConfig` that controls visual layout and colors.
///
/// # Returns
/// Returns `Ok(Some(index))` when the user makes a selection via `Enter`
/// (either via arrow navigation or numeric input),
/// `Ok(None)` if the user presses `Esc`,
/// or an `Err` if a terminal I/O error occurs.
///
/// # Features
/// - Navigate with arrow keys, PageUp/PageDown
/// - Type numbers to jump to an item directly
/// - Backspace to edit input buffer
/// - Realtime visual updates with highlighted selection
/// - Automatic terminal space management
pub fn choose_from_list<T: ToString>(items: &[T], config: &ListConfig) -> Result<Option<usize>> {
    enable_raw_mode()?;
    let (start_col, start_row) = position()?;
    let mut stdout = stdout();

    // Ensure we have enough space in the terminal
    let display_start_row = ensure_display_space(start_row, config)?;

    let total = items.len();
    let per_page = config.items_per_row * config.rows_per_page;
    let mut selected = 0;
    let mut digit_buffer = String::new();

    render_page(
        items,
        selected,
        &digit_buffer,
        config,
        start_col,
        display_start_row,
    )?;

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char(c) if c.is_digit(10) => digit_buffer.push(c),
                KeyCode::Backspace if !digit_buffer.is_empty() => {
                    digit_buffer.pop();
                }
                KeyCode::Left if selected > 0 => {
                    digit_buffer.clear();
                    selected -= 1;
                }
                KeyCode::Right if selected + 1 < total => {
                    digit_buffer.clear();
                    selected += 1;
                }
                KeyCode::Up if selected >= config.items_per_row => {
                    digit_buffer.clear();
                    selected -= config.items_per_row;
                }
                KeyCode::Down if selected + config.items_per_row < total => {
                    digit_buffer.clear();
                    selected += config.items_per_row;
                }
                KeyCode::PageDown if selected + per_page < total => {
                    digit_buffer.clear();
                    selected += per_page;
                }
                KeyCode::PageUp if selected >= per_page => {
                    digit_buffer.clear();
                    selected -= per_page;
                }
                KeyCode::Enter => {
                    let choice = if !digit_buffer.is_empty() {
                        digit_buffer
                            .parse::<usize>()
                            .ok()
                            .and_then(|n| (1..=total).contains(&n).then(|| n - 1))
                    } else {
                        Some(selected)
                    };
                    cleanup(&mut stdout, start_col, display_start_row, config)?;
                    disable_raw_mode()?;
                    return Ok(choice);
                }
                KeyCode::Esc => {
                    cleanup(&mut stdout, start_col, display_start_row, config)?;
                    disable_raw_mode()?;
                    return Ok(None);
                }
                _ => {}
            }

            render_page(
                items,
                selected,
                &digit_buffer,
                config,
                start_col,
                display_start_row,
            )?;
        }
    }
}

/// Ensure there's enough space in the terminal to display the list.
/// If not enough space, create additional lines by printing newlines.
///
/// # Parameters
/// - `current_row`: Current cursor row position
/// - `config`: Configuration containing display dimensions
///
/// # Returns
/// The row where the list should start displaying
fn ensure_display_space(current_row: u16, config: &ListConfig) -> Result<u16> {
    let mut stdout = stdout();
    let (_, terminal_height) = size()?;

    // Calculate required space: rows_per_page + 1 (for input line)
    let required_lines = config.rows_per_page as u16 + 1;
    let available_lines = terminal_height.saturating_sub(current_row);

    if available_lines < required_lines {
        // Need to create more space
        let lines_to_create = required_lines - available_lines;

        // Print newlines to create space and scroll the terminal
        for _ in 0..lines_to_create {
            execute!(stdout, Print("\n"))?;
        }
        stdout.flush()?;

        // Get new position after creating space
        let (_, new_row) = position()?;
        // The display should start from a position that leaves enough space
        Ok(new_row.saturating_sub(required_lines))
    } else {
        // Enough space available, use current position
        Ok(current_row)
    }
}

/// Compute the start index of the current page based on the selected item and page size.
fn calculate_page_start(selected: usize, per_page: usize) -> usize {
    (selected / per_page) * per_page
}

/// Clear the displayed list from the terminal.
///
/// Used to clean up the screen after the list UI is dismissed.
fn cleanup(
    stdout: &mut impl Write,
    start_col: u16,
    start_row: u16,
    config: &ListConfig,
) -> anyhow::Result<()> {
    let page_size = config.items_per_row * config.rows_per_page;
    for idx in 0..page_size {
        let row = idx / config.items_per_row;
        let col = idx % config.items_per_row;
        execute!(
            stdout,
            MoveTo(
                start_col + col as u16 * config.cell_width,
                start_row + row as u16
            ),
            Print(" ".repeat(config.cell_width as usize))
        )?;
    }
    // Clear input line
    execute!(
        stdout,
        MoveTo(start_col, start_row + config.rows_per_page as u16),
        Print(" ".repeat(config.cell_width as usize)),
        MoveTo(start_col, start_row + config.rows_per_page as u16)
    )?;
    Ok(())
}

/// Render the current page of items to the terminal, with selection and optional digit input.
///
/// # Parameters
/// - `items`: List of displayable items
/// - `selected`: Index of the currently selected item
/// - `digit_buffer`: Currently typed numeric input (if any)
/// - `config`: Layout and color configuration
/// - `start_col`: Starting column position in the terminal
/// - `start_row`: Starting row position in the terminal
fn render_page<T: ToString>(
    items: &[T],
    selected: usize,
    digit_buffer: &str,
    config: &ListConfig,
    start_col: u16,
    start_row: u16,
) -> Result<()> {
    let mut stdout = stdout();
    let page_size = config.items_per_row * config.rows_per_page;
    let page_start = calculate_page_start(selected, page_size);

    // Clear previous content
    for idx in 0..page_size {
        let row = idx / config.items_per_row;
        let col = idx % config.items_per_row;
        execute!(
            stdout,
            MoveTo(
                start_col + col as u16 * config.cell_width,
                start_row + row as u16
            ),
            Print(" ".repeat(config.cell_width as usize))
        )?;
    }

    // Draw items
    for idx in 0..page_size {
        let global = page_start + idx;
        if global >= items.len() {
            break;
        }
        let row = idx / config.items_per_row;
        let col = idx % config.items_per_row;
        let x = start_col + col as u16 * config.cell_width;
        let y = start_row + row as u16;
        execute!(stdout, MoveTo(x, y))?;
        let fg = if global == selected {
            config.highlight_fg
        } else {
            config.normal_fg
        };
        execute!(
            stdout,
            SetForegroundColor(fg.into()),
            Print(format!(
                "{num:>2}. {text:<width$}",
                num = global + 1,
                text = items[global].to_string(),
                width = (config.cell_width as usize - 4)
            ))
        )?;
    }

    // Draw digit input buffer
    execute!(
        stdout,
        MoveTo(start_col, start_row + config.rows_per_page as u16),
        Print(" ".repeat(config.cell_width as usize)),
        MoveTo(start_col, start_row + config.rows_per_page as u16)
    )?;
    if !digit_buffer.is_empty() {
        execute!(
            stdout,
            SetForegroundColor(Color::White.into()),
            Print(format!("Input: {}", digit_buffer))
        )?;
    }
    execute!(
        stdout,
        ResetColor,
        MoveTo(start_col, start_row + config.rows_per_page as u16)
    )?;
    stdout.flush()?;
    Ok(())
}
