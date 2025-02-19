use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{
    layout::Constraint,
    prelude::*,
    text::{Line, Span},
    widgets::*,
};
use std::collections::{HashMap, HashSet};
use std::io;
use std::time::Duration;

#[derive(PartialEq, Eq)]
enum SortField {
    Name,
    Rarity,
    Event,
}

#[derive(Debug, Clone)]
struct Skin {
    name: String,
    name_lower: String,
    rarity: String,
    rarity_lower: String,
    event: String,
    event_lower: String,
    year: Option<u32>,
    year_str: String,
    tags: Vec<String>,
    tags_lower: HashSet<String>,
}

#[derive(Default)]
struct TermInfo {
    is_name: bool,
    is_event: bool,
    is_rarity: bool,
    is_tag: bool,
    is_year: bool,
}

struct AppState {
    input: String,
    skins: Vec<Skin>,
    results: Vec<Skin>,
    table_state: TableState,
    all_terms: HashMap<String, TermInfo>,
    suggestion: Option<String>,
    suggestion_list: Vec<String>,
    suggestion_index: usize,
    name_map: HashMap<String, usize>,
    input_history: Vec<String>,
    history_index: usize,
    scroll_offset: usize,
    sort_field: SortField,
    sort_descending: bool,
    show_detail: bool,
}

impl AppState {
    fn new() -> Self {
        let skins = load_skins();
        let name_map: HashMap<_, _> = skins
            .iter()
            .enumerate()
            .map(|(i, s)| (s.name_lower.clone(), i))
            .collect();
        let all_terms = load_all_terms(&skins);
        let mut results = skins.clone();
        // Default sort: Name ascending.
        results.sort_by(|a, b| a.name_lower.cmp(&b.name_lower));
        AppState {
            input: String::new(),
            skins,
            results,
            table_state: TableState::default().with_selected(Some(0)),
            all_terms,
            suggestion: None,
            suggestion_list: Vec::new(),
            suggestion_index: 0,
            name_map,
            input_history: vec![String::new()],
            history_index: 0,
            scroll_offset: 0,
            sort_field: SortField::Name,
            sort_descending: false,
            show_detail: true,
        }
    }

    fn update_search(&mut self) {
        if self.input.trim().is_empty() {
            self.results = self.skins.clone();
            // When the search is cleared, default sort is by name ascending.
            self.sort_field = SortField::Name;
            self.sort_descending = false;
            self.sort_results();
            self.table_state.select(Some(0));
            self.suggestion_list.clear();
            self.suggestion_index = 0;
            self.suggestion = None;
            return;
        }

        let binding = self.input.to_lowercase();
        let tags: HashSet<&str> = binding.split_whitespace().collect();
        self.results = search_skins(&self.skins, &self.name_map, &tags);
        // After filtering, sort using current sort settings.
        self.sort_results();
        self.table_state.select(Some(0));
        self.update_suggestion();
    }

    fn sort_results(&mut self) {
        match self.sort_field {
            SortField::Name => {
                if self.sort_descending {
                    self.results.sort_by(|a, b| b.name_lower.cmp(&a.name_lower));
                } else {
                    self.results.sort_by(|a, b| a.name_lower.cmp(&b.name_lower));
                }
            }
            SortField::Rarity => {
                if self.sort_descending {
                    self.results
                        .sort_by(|a, b| b.rarity_lower.cmp(&a.rarity_lower));
                } else {
                    self.results
                        .sort_by(|a, b| a.rarity_lower.cmp(&b.rarity_lower));
                }
            }
            SortField::Event => {
                if self.sort_descending {
                    self.results
                        .sort_by(|a, b| b.event_lower.cmp(&a.event_lower));
                } else {
                    self.results
                        .sort_by(|a, b| a.event_lower.cmp(&b.event_lower));
                }
            }
        }
    }

    fn toggle_sort(&mut self, field: SortField) {
        if self.sort_field == field {
            self.sort_descending = !self.sort_descending;
        } else {
            self.sort_field = field;
            self.sort_descending = true;
        }
        self.sort_results();
    }

    fn update_suggestion(&mut self) {
        let input_parts: Vec<&str> = self.input.split_whitespace().collect();
        let last_part = input_parts.last().cloned().unwrap_or("");
        let last_part_lower = last_part.to_lowercase();
        self.suggestion_list.clear();
        self.suggestion_index = 0;

        if !last_part_lower.is_empty() {
            let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
            let mut suggestions = Vec::new();

            for (term, term_info) in &self.all_terms {
                let score = matcher
                    .fuzzy_match(term, &last_part_lower)
                    .unwrap_or(i64::MIN);
                let prefix_boost = if term.starts_with(&last_part_lower) {
                    1000
                } else {
                    0
                };
                let field_boost = if term_info.is_name || term_info.is_event {
                    500
                } else {
                    0
                };

                if score + prefix_boost + field_boost > 50 {
                    suggestions.push((score + prefix_boost + field_boost, term.clone()));
                }
            }

            suggestions.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
            self.suggestion_list = suggestions
                .into_iter()
                .map(|(_, term)| term)
                .take(5)
                .collect();

            self.suggestion = self.suggestion_list.first().cloned();
        } else {
            self.suggestion = None;
        }
    }

    fn cycle_suggestion(&mut self, direction: i32) {
        if !self.suggestion_list.is_empty() {
            self.suggestion_index = (self.suggestion_index as i32 + direction)
                .rem_euclid(self.suggestion_list.len() as i32)
                as usize;
            self.suggestion = Some(self.suggestion_list[self.suggestion_index].clone());
        }
    }

    fn accept_suggestion(&mut self) {
        if let Some(suggestion) = &self.suggestion {
            let mut parts: Vec<&str> = self.input.split_whitespace().collect();
            if parts.is_empty() {
                self.input = format!("{} ", suggestion);
            } else {
                let last_part = parts.last().unwrap().to_lowercase();
                if suggestion.starts_with(&last_part) {
                    parts.pop();
                }
                parts.push(suggestion);
                self.input = parts.join(" ") + " ";
            }
            self.update_search();
            self.suggestion_list.clear();
            self.suggestion_index = 0;
            self.record_input();
        }
    }

    fn next(&mut self) {
        let i = self.table_state.selected().map_or(0, |i| {
            if i + 1 < self.results.len() {
                i + 1
            } else {
                i
            }
        });
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = self
            .table_state
            .selected()
            .map_or(0, |i| if i > 0 { i - 1 } else { 0 });
        self.table_state.select(Some(i));
    }

    fn record_input(&mut self) {
        if self.history_index < self.input_history.len() - 1 {
            self.input_history.truncate(self.history_index + 1);
        }
        if self.input_history.last() != Some(&self.input) {
            self.input_history.push(self.input.clone());
            self.history_index = self.input_history.len() - 1;
        }
    }

    fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.input = self.input_history[self.history_index].clone();
            self.update_search();
        }
    }

    fn redo(&mut self) {
        if self.history_index + 1 < self.input_history.len() {
            self.history_index += 1;
            self.input = self.input_history[self.history_index].clone();
            self.update_search();
        }
    }
}

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

/// This function draws a full-screen help page overlay.
fn show_help<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let modal_area = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size)[0];

            let block = Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::LightCyan));

            let help_text = vec![
                Line::from("[CTRL+L] : Clear search bar"),
                Line::from("[CTRL+H] : Show this help page"),
                Line::from("[CTRL+D] : Toggle detailed view"),
                Line::from("[CTRL+Z] : Undo input in search bar"),
                Line::from("[CTRL+Y] : Redo input in search bar"),
                Line::from("[UP/DOWN ▲▼] Or Mouse Scroll: Navigate results"),
                Line::from("[TAB]: Cycle suggestions"),
                Line::from("[HOME/END] : Jump to first/last result"),
                Line::from("[RIGHT ►] : Accept suggestion and auto-fills"),
                Line::from("[ESC]: Exit help or exit application"),
            ];

            let paragraph = Paragraph::new(help_text)
                .block(block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, modal_area);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn get_rarity_color(skin: &Skin) -> Color {
    match skin.rarity_lower.as_str() {
        "pink" => Color::Magenta,
        "red" => Color::Red,
        "teal" => Color::Cyan,
        _ => Color::White,
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(f.size());

    let input_text = if app.input.is_empty() {
        Text::from(Line::from(Span::styled(
            "Type to search skins...",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )))
    } else {
        let mut line = Line::default();
        let mut current_token = String::new();
        let mut current_is_whitespace = false;

        // Split input into whitespace and term tokens
        for c in app.input.chars() {
            if c.is_whitespace() {
                if !current_is_whitespace && !current_token.is_empty() {
                    let lower_token = current_token.to_lowercase();
                    let style = get_term_style(&lower_token, &app.all_terms);
                    line.spans.push(Span::styled(current_token.clone(), style));
                    current_token.clear();
                }
                current_is_whitespace = true;
                current_token.push(c);
            } else {
                if current_is_whitespace && !current_token.is_empty() {
                    line.spans.push(Span::raw(current_token.clone()));
                    current_token.clear();
                }
                current_is_whitespace = false;
                current_token.push(c);
            }
        }

        // Add remaining token
        if !current_token.is_empty() {
            if current_is_whitespace {
                line.spans.push(Span::raw(current_token));
            } else {
                let lower_token = current_token.to_lowercase();
                let style = get_term_style(&lower_token, &app.all_terms);
                line.spans.push(Span::styled(current_token, style));
            }
        }

        // Add suggestion suffix if applicable
        if let Some(suggestion) = &app.suggestion {
            let last_part = app
                .input
                .split_whitespace()
                .last()
                .unwrap_or("")
                .to_lowercase();
            if suggestion.starts_with(&last_part) {
                let suffix = &suggestion[last_part.len()..];
                line.spans.push(Span::styled(
                    suffix,
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::DIM),
                ));
            }
        }

        Text::from(line)
    };

    let search_input = Paragraph::new(input_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .title("Search [Ex: Pink Summer]".bold()),
    );

    f.render_widget(search_input, chunks[0]);

    // Set the cursor position at the end of the input
    let inner_area = chunks[0].inner(&Margin {
        horizontal: 1,
        vertical: 1,
    });
    let cursor_x = inner_area.x + app.input.len() as u16;
    let cursor_y = inner_area.y;
    f.set_cursor(cursor_x, cursor_y);

    // Suggestions list
    let suggestions: Vec<ListItem> = app
        .suggestion_list
        .iter()
        .map(|t| {
            let lower_t = t.to_lowercase();
            let style = if let Some(term_info) = app.all_terms.get(&lower_t) {
                if term_info.is_rarity {
                    match lower_t.as_str() {
                        "pink" => Style::default().fg(Color::Magenta),
                        "red" => Style::default().fg(Color::Red),
                        "teal" => Style::default().fg(Color::Cyan),
                        _ => Style::default().fg(Color::White),
                    }
                } else if term_info.is_event {
                    Style::default().fg(Color::Magenta)
                } else if term_info.is_year {
                    Style::default().fg(Color::Blue)
                } else if term_info.is_tag {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::White)
                }
            } else {
                Style::default().fg(Color::Yellow)
            };

            let count = app
                .skins
                .iter()
                .filter(|s| {
                    s.name_lower == lower_t
                        || s.event_lower == lower_t
                        || s.tags_lower.contains(&lower_t)
                })
                .count();

            let mut spans = vec![Span::styled(t, style)];
            spans.push(Span::styled(
                format!(" ({})", count),
                Style::default().fg(Color::DarkGray),
            ));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.suggestion_index));
    let suggestion_list = List::new(suggestions)
        .block(
            Block::default()
                .title("Suggestions")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(suggestion_list, chunks[1], &mut list_state);

    let (table_area, detail_area) = if app.show_detail {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[2]);
        (chunks[0], Some(chunks[1]))
    } else {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(chunks[2]);
        (chunks[0], None)
    };

    render_table_view(f, app, table_area);
    if let Some(detail_area) = detail_area {
        render_detail_panel(f, app, detail_area);
    }

    // Status bar
    let status = Paragraph::new("Press ESC to exit | Ctrl + H for Help | Tab to cycle suggestions | ► to accept | Scroll or ▲▼ to select")
        .style(Style::default().fg(Color::LightBlue));
    f.render_widget(status, chunks[3]);
}

fn get_term_style(term: &str, all_terms: &HashMap<String, TermInfo>) -> Style {
    if let Some(term_info) = all_terms.get(term) {
        if term_info.is_rarity {
            match term {
                "pink" => Style::default().fg(Color::Magenta),
                "red" => Style::default().fg(Color::Red),
                "teal" => Style::default().fg(Color::Cyan),
                _ => Style::default().fg(Color::White),
            }
        } else if term_info.is_event {
            Style::default().fg(Color::Magenta)
        } else if term_info.is_year {
            Style::default().fg(Color::Blue)
        } else if term_info.is_tag {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        }
    } else {
        Style::default().fg(Color::White)
    }
}

fn render_table_view<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    if app.results.is_empty() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("No results found. Try a different search.")
            .border_style(Style::default().fg(Color::Red));

        let message = Paragraph::new("No matches found")
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(message, area);
    } else {
        // Build header titles
        let name_header = if app.sort_field == SortField::Name && app.sort_descending {
            "Name ↓"
        } else {
            "Name"
        };
        let rarity_header = if app.sort_field == SortField::Rarity && app.sort_descending {
            "Rarity ↓"
        } else {
            "Rarity"
        };
        let event_header = if app.sort_field == SortField::Event && app.sort_descending {
            "Event ↓"
        } else {
            "Event"
        };

        let header = Row::new(vec![
            name_header,
            rarity_header,
            event_header,
            "Year",
            "Tags",
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

        let rows: Vec<Row> = app
            .results
            .iter()
            .map(|skin| {
                let year = skin.year.map_or(String::from("N/A"), |y| y.to_string());
                Row::new(vec![
                    Line::from(Span::styled(&skin.name, Style::default().fg(Color::Cyan))),
                    Line::from(Span::styled(
                        &skin.rarity,
                        Style::default().fg(get_rarity_color(skin)),
                    )),
                    Line::from(Span::styled(
                        &skin.event,
                        Style::default().fg(Color::Magenta),
                    )),
                    Line::from(Span::styled(year, Style::default().fg(Color::Green))),
                    Line::from(Span::styled(
                        skin.tags.join(", "),
                        Style::default().fg(Color::White),
                    )),
                ])
            })
            .collect();

        let table = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Cyan))
                    .title(format!(
                        "Results: {} | Selected: {}",
                        app.results.len(),
                        app.table_state.selected().map(|i| i + 1).unwrap_or(0)
                    )),
            )
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(10),
                Constraint::Percentage(25),
                Constraint::Percentage(10),
                Constraint::Percentage(25),
            ])
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(table, area, &mut app.table_state);
    }
}

fn render_detail_panel<B: Backend>(f: &mut Frame<B>, app: &AppState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .title("Details")
        .style(Style::default());

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if let Some(selected) = app.table_state.selected() {
        if let Some(skin) = app.results.get(selected) {
            let details = vec![
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Cyan)),
                    Span::styled(&skin.name, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Rarity: ", Style::default().fg(Color::Yellow)),
                    Span::styled(&skin.rarity, Style::default().fg(get_rarity_color(skin))),
                ]),
                Line::from(vec![
                    Span::styled("Event: ", Style::default().fg(Color::Magenta)),
                    Span::styled(&skin.event, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Year: ", Style::default().fg(Color::Blue)),
                    Span::styled(
                        skin.year.map_or(String::from("N/A"), |y| y.to_string()),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Tags: ", Style::default().fg(Color::Green)),
                    Span::styled(skin.tags.join(", "), Style::default().fg(Color::White)),
                ]),
            ];

            let details_paragraph = Paragraph::new(details)
                .block(Block::default().borders(Borders::NONE))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(details_paragraph, inner_area);
        }
    }
}

fn load_all_terms(skins: &[Skin]) -> HashMap<String, TermInfo> {
    let mut terms = HashMap::new();
    for skin in skins {
        // Name words
        for word in skin.name_lower.split_whitespace() {
            let entry: &mut TermInfo = terms.entry(word.to_string()).or_default();
            entry.is_name = true;
        }
        // Event words
        for word in skin.event_lower.split_whitespace() {
            let entry: &mut TermInfo = terms.entry(word.to_string()).or_default();
            entry.is_event = true;
        }
        // Rarity
        let entry: &mut TermInfo = terms.entry(skin.rarity_lower.clone()).or_default();
        entry.is_rarity = true;
        // Tags
        for tag in &skin.tags_lower {
            let entry: &mut TermInfo = terms.entry(tag.clone()).or_default();
            entry.is_tag = true;
        }
        // Year
        if !skin.year_str.is_empty() {
            let entry: &mut TermInfo = terms.entry(skin.year_str.clone()).or_default();
            entry.is_year = true;
        }
    }
    terms
}

fn load_skins() -> Vec<Skin> {
    let mut skins = vec![
        // Valentine Case
        Skin {
            name: "Cupid".to_string(),
            name_lower: "cupid".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Valentine Case".to_string(),
            event_lower: "valentine case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Rainbow Periastron".to_string(),
            name_lower: "rainbow periastron".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Valentine Case (Exquisite)".to_string(),
            event_lower: "valentine case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec![
                "case".to_string(),
                "exquisite".to_string(),
                "periastron".to_string(),
            ],
            tags_lower: vec![
                "case".to_string(),
                "exquisite".to_string(),
                "periastron".to_string(),
            ]
            .into_iter()
            .map(|t| t.to_lowercase())
            .collect(),
        },
        Skin {
            name: "Crimson Periastron".to_string(),
            name_lower: "crimson periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Valentine Case".to_string(),
            event_lower: "valentine case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Heartsong".to_string(),
            name_lower: "heartsong".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Valentine Case".to_string(),
            event_lower: "valentine case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Ivory Periastron".to_string(),
            name_lower: "ivory periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Valentine Case (Exquisite)".to_string(),
            event_lower: "valentine case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec![
                "case".to_string(),
                "exquisite".to_string(),
                "periastron".to_string(),
            ],
            tags_lower: vec![
                "case".to_string(),
                "exquisite".to_string(),
                "periastron".to_string(),
            ]
            .into_iter()
            .map(|t| t.to_lowercase())
            .collect(),
        },
        Skin {
            name: "Diamond".to_string(),
            name_lower: "diamond".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Valentine Case (Exquisite)".to_string(),
            event_lower: "valentine case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Epicredness".to_string(),
            name_lower: "epicredness".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Valentine Case (Exquisite)".to_string(),
            event_lower: "valentine case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Birthday Case
        Skin {
            name: "Ghostly".to_string(),
            name_lower: "ghostly".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Birthday Case".to_string(),
            event_lower: "birthday case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Hellfire".to_string(),
            name_lower: "hellfire".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Birthday Case".to_string(),
            event_lower: "birthday case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Surge".to_string(),
            name_lower: "surge".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Birthday Case (Exquisite)".to_string(),
            event_lower: "birthday case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Epicblueness".to_string(),
            name_lower: "epicblueness".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Birthday Case (Exquisite)".to_string(),
            event_lower: "birthday case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Golden".to_string(),
            name_lower: "golden".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Birthday Case (Exquisite)".to_string(),
            event_lower: "birthday case (exquisite)".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Grimgold Periastron".to_string(),
            name_lower: "grimgold periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Birthday Case".to_string(),
            event_lower: "birthday case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Easter Case
        Skin {
            name: "Spring Growth".to_string(),
            name_lower: "spring growth".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Easter Case".to_string(),
            event_lower: "easter case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Amethyst Periastron".to_string(),
            name_lower: "amethyst periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Easter Case".to_string(),
            event_lower: "easter case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Bunny".to_string(),
            name_lower: "bunny".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Easter Case".to_string(),
            event_lower: "easter case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Guitar".to_string(),
            name_lower: "guitar".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Easter Case".to_string(),
            event_lower: "easter case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Joyful Periastron".to_string(),
            name_lower: "joyful periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Easter Case".to_string(),
            event_lower: "easter case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Noir Periastron".to_string(),
            name_lower: "noir periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Easter Case".to_string(),
            event_lower: "easter case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Summer Case
        Skin {
            name: "Midsummer".to_string(),
            name_lower: "midsummer".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Mystic".to_string(),
            name_lower: "mystic".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "popular".to_string()],
            tags_lower: vec!["case".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Void Lord".to_string(),
            name_lower: "void lord".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "popular".to_string()],
            tags_lower: vec!["case".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Warlord".to_string(),
            name_lower: "warlord".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Cythrex".to_string(),
            name_lower: "cythrex".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Dog".to_string(),
            name_lower: "dog".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "popular".to_string()],
            tags_lower: vec!["case".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Fire Wyvern".to_string(),
            name_lower: "fire wyvern".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Ghostfire".to_string(),
            name_lower: "ghostfire".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Inscription".to_string(),
            name_lower: "inscription".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Mummy".to_string(),
            name_lower: "mummy".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Retrowave".to_string(),
            name_lower: "retrowave".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Shikai".to_string(),
            name_lower: "shikai".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Summer Case".to_string(),
            event_lower: "summer case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Halloween Case
        Skin {
            name: "All Hallow's".to_string(),
            name_lower: "all hallow's".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "popular".to_string()],
            tags_lower: vec!["case".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Anansi".to_string(),
            name_lower: "anansi".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Dusekkar".to_string(),
            name_lower: "dusekkar".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Count".to_string(),
            name_lower: "count".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Dracula".to_string(),
            name_lower: "dracula".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Hallowing".to_string(),
            name_lower: "hallowing".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Orange Energy".to_string(),
            name_lower: "orange energy".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "energy".to_string()],
            tags_lower: vec!["case".to_string(), "energy".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Pumpkin".to_string(),
            name_lower: "pumpkin".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Halloween Case".to_string(),
            event_lower: "halloween case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Christmas Case
        Skin {
            name: "Evergreen".to_string(),
            name_lower: "evergreen".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Icycle".to_string(),
            name_lower: "icycle".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Santa".to_string(),
            name_lower: "santa".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Candy Energy".to_string(),
            name_lower: "candy energy".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "energy".to_string()],
            tags_lower: vec!["case".to_string(), "energy".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Festive Periastron".to_string(),
            name_lower: "festive periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Snowflake".to_string(),
            name_lower: "snowflake".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Snowman".to_string(),
            name_lower: "snowman".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Christmas Case".to_string(),
            event_lower: "christmas case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Easter Event
        Skin {
            name: "Azurite".to_string(),
            name_lower: "azurite".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Easter Event".to_string(),
            event_lower: "easter event".to_string(),
            year: Some(2022),
            year_str: "2022".to_string(),
            tags: vec!["event".to_string(), "popular".to_string()],
            tags_lower: vec!["event".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Corrupted".to_string(),
            name_lower: "corrupted".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Easter Event".to_string(),
            event_lower: "easter event".to_string(),
            year: Some(2023),
            year_str: "2023".to_string(),
            tags: vec!["event".to_string(), "popular".to_string()],
            tags_lower: vec!["event".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Sun Slayer".to_string(),
            name_lower: "sun slayer".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Easter Event".to_string(),
            event_lower: "easter event".to_string(),
            year: Some(2024),
            year_str: "2024".to_string(),
            tags: vec!["event".to_string()],
            tags_lower: vec!["event".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Summer Bundle
        Skin {
            name: "Cartoony Rainbow".to_string(),
            name_lower: "cartoony rainbow".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2023),
            year_str: "2023".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Cyberlight".to_string(),
            name_lower: "cyberlight".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2023),
            year_str: "2023".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Frostburn".to_string(),
            name_lower: "frostburn".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2023),
            year_str: "2023".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Inferno Angel".to_string(),
            name_lower: "inferno angel".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2023),
            year_str: "2023".to_string(),
            tags: vec!["bundle".to_string(), "popular".to_string()],
            tags_lower: vec!["bundle".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Azure Dragon".to_string(),
            name_lower: "azure dragon".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2024),
            year_str: "2024".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Darkness".to_string(),
            name_lower: "darkness".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2024),
            year_str: "2024".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Vilethorn".to_string(),
            name_lower: "vilethorn".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2024),
            year_str: "2024".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Winged".to_string(),
            name_lower: "winged".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Summer Bundle".to_string(),
            event_lower: "summer bundle".to_string(),
            year: Some(2024),
            year_str: "2024".to_string(),
            tags: vec!["bundle".to_string(), "popular".to_string()],
            tags_lower: vec!["bundle".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Valentine Bundle
        Skin {
            name: "Cupid's Revenge".to_string(),
            name_lower: "cupid's revenge".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Valentine Bundle".to_string(),
            event_lower: "valentine bundle".to_string(),
            year: Some(2025),
            year_str: "2025".to_string(),
            tags: vec!["bundle".to_string()],
            tags_lower: vec!["bundle".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Love Scepter".to_string(),
            name_lower: "love scepter".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Valentine Bundle".to_string(),
            event_lower: "valentine bundle".to_string(),
            year: Some(2025),
            year_str: "2025".to_string(),
            tags: vec!["bundle".to_string(), "popular".to_string()],
            tags_lower: vec!["bundle".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Wicked Rose".to_string(),
            name_lower: "wicked rose".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Valentine Bundle".to_string(),
            event_lower: "valentine bundle".to_string(),
            year: Some(2025),
            year_str: "2025".to_string(),
            tags: vec!["bundle".to_string(), "popular".to_string()],
            tags_lower: vec!["bundle".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Christmas Event
        Skin {
            name: "Redmaster".to_string(),
            name_lower: "redmaster".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2022),
            year_str: "2022".to_string(),
            tags: vec!["event".to_string(), "rare".to_string()],
            tags_lower: vec!["event".to_string(), "rare".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Yellowflame".to_string(),
            name_lower: "yellowflame".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2022),
            year_str: "2022".to_string(),
            tags: vec!["event".to_string(), "rare".to_string()],
            tags_lower: vec!["event".to_string(), "rare".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Goldenrod".to_string(),
            name_lower: "goldenrod".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2022),
            year_str: "2022".to_string(),
            tags: vec!["event".to_string(), "rare".to_string()],
            tags_lower: vec!["event".to_string(), "rare".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Whisper".to_string(),
            name_lower: "whisper".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2022),
            year_str: "2022".to_string(),
            tags: vec!["event".to_string(), "rare".to_string()],
            tags_lower: vec!["event".to_string(), "rare".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Gingerblade".to_string(),
            name_lower: "gingerblade".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2022),
            year_str: "2022".to_string(),
            tags: vec!["event".to_string(), "rare".to_string()],
            tags_lower: vec!["event".to_string(), "rare".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Candy Cane".to_string(),
            name_lower: "candy cane".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2023),
            year_str: "2023".to_string(),
            tags: vec!["event".to_string()],
            tags_lower: vec!["event".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Iceblade".to_string(),
            name_lower: "iceblade".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Christmas Event".to_string(),
            event_lower: "christmas event".to_string(),
            year: Some(2024),
            year_str: "2024".to_string(),
            tags: vec!["event".to_string(), "popular".to_string()],
            tags_lower: vec!["event".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Code Redeemed Skins (Teal)
        Skin {
            name: "Bubbles".to_string(),
            name_lower: "bubbles".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code Redeemed".to_string(),
            event_lower: "code redeemed".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string()],
            tags_lower: vec!["code".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Butter".to_string(),
            name_lower: "butter".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code Redeemed".to_string(),
            event_lower: "code redeemed".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string(), "duped".to_string()],
            tags_lower: vec!["code".to_string(), "duped".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Fireworks".to_string(),
            name_lower: "fireworks".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code Redeemed".to_string(),
            event_lower: "code redeemed".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string()],
            tags_lower: vec!["code".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Pearl".to_string(),
            name_lower: "pearl".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code Redeemed".to_string(),
            event_lower: "code redeemed".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string()],
            tags_lower: vec!["code".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Tin".to_string(),
            name_lower: "tin".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code Redeemed".to_string(),
            event_lower: "code redeemed".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string()],
            tags_lower: vec!["code".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Launch Skins (Teal)
        Skin {
            name: "Blastoff".to_string(),
            name_lower: "blastoff".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Launch".to_string(),
            event_lower: "launch".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["launch".to_string()],
            tags_lower: vec!["launch".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Exquisite Case (Pinks & Reds)
        Skin {
            name: "Behemoth".to_string(),
            name_lower: "behemoth".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Blizzard".to_string(),
            name_lower: "blizzard".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Crescendo".to_string(),
            name_lower: "crescendo".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Demon".to_string(),
            name_lower: "demon".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Overseer".to_string(),
            name_lower: "overseer".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Redcliff".to_string(),
            name_lower: "redcliff".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Skeletal".to_string(),
            name_lower: "skeletal".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Telamonster".to_string(),
            name_lower: "telamonster".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Unseen".to_string(),
            name_lower: "unseen".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Bombastic".to_string(),
            name_lower: "bombastic".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Crimsonwrath".to_string(),
            name_lower: "crimsonwrath".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Sanctum".to_string(),
            name_lower: "sanctum".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Exquisite Case".to_string(),
            event_lower: "exquisite case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "exquisite".to_string()],
            tags_lower: vec!["case".to_string(), "exquisite".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Animal Case
        Skin {
            name: "Spider".to_string(),
            name_lower: "spider".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Animal Case".to_string(),
            event_lower: "animal case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Unicorn".to_string(),
            name_lower: "unicorn".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Animal Case".to_string(),
            event_lower: "animal case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Bacon".to_string(),
            name_lower: "bacon".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Animal Case".to_string(),
            event_lower: "animal case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Salmon".to_string(),
            name_lower: "salmon".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Animal Case".to_string(),
            event_lower: "animal case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "popular".to_string()],
            tags_lower: vec!["case".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Shark".to_string(),
            name_lower: "shark".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Animal Case".to_string(),
            event_lower: "animal case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Slither".to_string(),
            name_lower: "slither".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Animal Case".to_string(),
            event_lower: "animal case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Camouflage Case
        Skin {
            name: "Dragon's Forge".to_string(),
            name_lower: "dragon's forge".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Glacial".to_string(),
            name_lower: "glacial".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Chartreuse Periastron".to_string(),
            name_lower: "chartreuse periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Fallen".to_string(),
            name_lower: "fallen".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Prehistoric".to_string(),
            name_lower: "prehistoric".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Shadow".to_string(),
            name_lower: "shadow".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Violet Energy".to_string(),
            name_lower: "violet energy".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Camouflage Case".to_string(),
            event_lower: "camouflage case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "energy".to_string()],
            tags_lower: vec!["case".to_string(), "energy".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Future Case
        Skin {
            name: "Laser".to_string(),
            name_lower: "laser".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Orinthan".to_string(),
            name_lower: "orinthan".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Azure Periastron".to_string(),
            name_lower: "azure periastron".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "periastron".to_string()],
            tags_lower: vec!["case".to_string(), "periastron".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Celestial".to_string(),
            name_lower: "celestial".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Galactic".to_string(),
            name_lower: "galactic".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Green Energy".to_string(),
            name_lower: "green energy".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string(), "energy".to_string()],
            tags_lower: vec!["case".to_string(), "energy".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Motherboard".to_string(),
            name_lower: "motherboard".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Omega".to_string(),
            name_lower: "omega".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Future Case".to_string(),
            event_lower: "future case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Material Case
        Skin {
            name: "Crystal".to_string(),
            name_lower: "crystal".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Material Case".to_string(),
            event_lower: "material case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Adurite".to_string(),
            name_lower: "adurite".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Material Case".to_string(),
            event_lower: "material case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Bluesteel".to_string(),
            name_lower: "bluesteel".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Material Case".to_string(),
            event_lower: "material case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Wooden".to_string(),
            name_lower: "wooden".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Material Case".to_string(),
            event_lower: "material case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Nature Case
        Skin {
            name: "Crystallised".to_string(),
            name_lower: "crystallised".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Elven".to_string(),
            name_lower: "elven".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Molten".to_string(),
            name_lower: "molten".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Autumnal".to_string(),
            name_lower: "autumnal".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Beach".to_string(),
            name_lower: "beach".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Breeze".to_string(),
            name_lower: "breeze".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Earth".to_string(),
            name_lower: "earth".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Ocean".to_string(),
            name_lower: "ocean".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Nature Case".to_string(),
            event_lower: "nature case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Pattern Case
        Skin {
            name: "Monochrome".to_string(),
            name_lower: "monochrome".to_string(),
            rarity: "Pink".to_string(),
            rarity_lower: "pink".to_string(),
            event: "Pattern Case".to_string(),
            event_lower: "pattern case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Relic".to_string(),
            name_lower: "relic".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Pattern Case".to_string(),
            event_lower: "pattern case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Sorcus".to_string(),
            name_lower: "sorcus".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Pattern Case".to_string(),
            event_lower: "pattern case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        // Refined Case
        Skin {
            name: "Archon".to_string(),
            name_lower: "archon".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Breaker".to_string(),
            name_lower: "breaker".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Divine".to_string(),
            name_lower: "divine".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Enforcer".to_string(),
            name_lower: "enforcer".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Frosted".to_string(),
            name_lower: "frosted".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Hunter".to_string(),
            name_lower: "hunter".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Neon".to_string(),
            name_lower: "neon".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Pharaoh".to_string(),
            name_lower: "pharaoh".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Skyward".to_string(),
            name_lower: "skyward".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Steampunk".to_string(),
            name_lower: "steampunk".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "Refined Case".to_string(),
            event_lower: "refined case".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["case".to_string()],
            tags_lower: vec!["case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "No Dagger".to_string(),
            name_lower: "no dagger".to_string(),
            rarity: "Red".to_string(),
            rarity_lower: "red".to_string(),
            event: "April Fools".to_string(),
            event_lower: "April Fools".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["all case".to_string()],
            tags_lower: vec!["all case".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
    ];

    // Post-process to populate derived fields
    for skin in &mut skins {
        skin.name_lower = skin.name.to_lowercase();
        skin.rarity_lower = skin.rarity.to_lowercase();
        skin.event_lower = skin.event.to_lowercase();
        skin.year_str = skin.year.map(|y| y.to_string()).unwrap_or_default();
        skin.tags_lower = skin.tags.iter().map(|t| t.to_lowercase()).collect();
    }

    skins
}

fn search_skins(
    skins: &[Skin],
    name_map: &HashMap<String, usize>,
    tags: &HashSet<&str>,
) -> Vec<Skin> {
    let exact_matches: Vec<Skin> = tags
        .iter()
        .filter_map(|tag| name_map.get(*tag))
        .map(|&i| skins[i].clone())
        .collect();
    if !exact_matches.is_empty() {
        return exact_matches;
    }

    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
    let mut scored_skins: Vec<(i64, &Skin)> = skins
        .iter()
        .filter_map(|skin| {
            let all_tags_matched = tags.iter().all(|&tag| {
                skin.name_lower.contains(tag)
                    || skin.rarity_lower == tag
                    || skin.event_lower.contains(tag)
                    || skin.year_str == tag
                    || skin.tags_lower.contains(tag)
            });
            if !all_tags_matched {
                return None;
            }

            let mut score = 0;
            for tag in tags {
                if skin.name_lower.contains(tag) {
                    score += 100;
                }
                if skin.rarity_lower == *tag {
                    score += 80;
                }
                if skin.event_lower.contains(tag) {
                    score += 60;
                }
                if let Some(s) = matcher.fuzzy_match(&skin.name_lower, tag) {
                    score += s;
                }
                if skin.tags_lower.contains(&tag.to_string()) {
                    score += 40;
                }
                if skin.year_str == *tag {
                    score += 20;
                }
            }
            Some((score, skin))
        })
        .collect();

    scored_skins.sort_by(|a, b| b.0.cmp(&a.0));
    scored_skins.into_iter().map(|(_, s)| s.clone()).collect()
}
