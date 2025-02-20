mod models;
mod state;
mod ui;
mod utils;

use state::AppState;
use ui::ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        crossterm::terminal::DisableLineWrap,
    )?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();
    app.update_search();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    match key.code {
                        KeyCode::Esc => break,

                        // Ctrl+L: clear the search bar
                        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.input.clear();
                            app.update_search();
                            app.record_input();
                        }
                        // Ctrl+H: show help page
                        KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            show_help(&mut terminal)?;
                        }
                        // Ctrl+D toggle detailed view
                        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.show_detail = !app.show_detail;
                        }
                        // Ctrl+Z: undo input
                        KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.undo();
                        }
                        // Ctrl+Y: redo input
                        KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.redo();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.update_search();
                            app.record_input();
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                            app.update_search();
                            app.record_input();
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Home => app.table_state.select(Some(0)),
                        KeyCode::End => app
                            .table_state
                            .select(Some(app.results.len().saturating_sub(1))),
                        KeyCode::Tab => {
                            if key.modifiers.contains(KeyModifiers::SHIFT) {
                                app.cycle_suggestion(-1);
                            } else {
                                app.cycle_suggestion(1);
                            }
                        }
                        KeyCode::Right => app.accept_suggestion(),
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollDown => {
                            app.scroll_offset = app.scroll_offset.saturating_add(1);
                            app.next();
                        }
                        MouseEventKind::ScrollUp => {
                            app.scroll_offset = app.scroll_offset.saturating_sub(1);
                            app.previous();
                        }
                        MouseEventKind::Down(_button) => {
                            // Recalculate layout to determine the table area
                            let term_area = terminal.size()?;
                            let outer_chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .margin(1)
                                .constraints([
                                    Constraint::Length(3),
                                    Constraint::Length(5),
                                    Constraint::Min(3),
                                    Constraint::Length(1),
                                ])
                                .split(term_area);
                            let main_chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints([
                                    Constraint::Percentage(60),
                                    Constraint::Percentage(40),
                                ])
                                .split(outer_chunks[2]);
                            let table_area = main_chunks[0];

                            // Adjust for the block borders and header row
                            let inner_y = table_area.y + 1; // skip top border
                            let header_height = 1; // header row height

                            // Check if the click is within the header row.
                            if mouse_event.row == inner_y {
                                // Calculate the relative x position in the table.
                                let relative_x = mouse_event.column.saturating_sub(table_area.x);
                                let table_width = table_area.width;
                                // Based on the header column widths (30%, 10%, 25%, 10%, 25%)
                                let name_width = (table_width as f32 * 0.30).round() as u16;
                                let rarity_width = (table_width as f32 * 0.10).round() as u16;
                                let event_width = (table_width as f32 * 0.25).round() as u16;
                                if relative_x < name_width {
                                    app.toggle_sort(SortField::Name);
                                } else if relative_x < name_width + rarity_width {
                                    app.toggle_sort(SortField::Rarity);
                                } else if relative_x < name_width + rarity_width + event_width {
                                    app.toggle_sort(SortField::Event);
                                }
                            } else {
                                // Otherwise, treat as row selection.
                                let results_start_y = inner_y + header_height;
                                let visible_index = (mouse_event.row - results_start_y) as usize;
                                let absolute_index = app.scroll_offset + visible_index;
                                if absolute_index < app.results.len() {
                                    app.table_state.select(Some(absolute_index));
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        crossterm::terminal::EnableLineWrap,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
