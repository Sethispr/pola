use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::FuzzyMatcher;
use image::{DynamicImage, GenericImageView};
use ratatui::{
    layout::Constraint,
    prelude::*,
    text::{Line, Span},
    widgets::*,
};
use std::collections::{HashMap, HashSet};
use std::io;
use std::path::Path;
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
    image_path: Option<String>,
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

fn load_image(image_path: &str) -> Option<DynamicImage> {
    if image_path.starts_with("http") {
        // Fetch the image from the URL using reqwest.
        let response = reqwest::blocking::get(image_path).ok()?;
        let bytes = response.bytes().ok()?;
        image::load_from_memory(&bytes).ok()
    } else if Path::new(image_path).exists() {
        // Load from a local file.
        image::open(image_path).ok()
    } else {
        None
    }
}

fn image_to_ascii(image_path: &str, width: u32) -> Option<String> {
    let img = load_image(image_path)?;
    let (img_width, img_height) = img.dimensions();
    // Calculate height to maintain the aspect ratio (adjust as needed)
    let height = (width as f32 * img_height as f32 / img_width as f32) as u32;
    let img = img.resize_exact(width, height, image::imageops::FilterType::Nearest);

    let ascii_chars = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', ',', '.'];
    let mut ascii_art = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness =
                (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                    / 255.0;
            let char_index = ((1.0 - brightness) * (ascii_chars.len() - 1) as f32).round() as usize;
            ascii_art.push(ascii_chars[char_index]);
        }
        ascii_art.push('\n');
    }
    Some(ascii_art)
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

    // Search input
    let input_text = if app.input.is_empty() {
        Text::from(Line::from(Span::styled(
            "Type to search skins...",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )))
    } else {
        let mut line = Line::default();
        let parts: Vec<&str> = app.input.split_whitespace().collect();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                line.spans.push(Span::raw(" "));
            }
            let lower_part = part.to_lowercase();
            let style = if let Some(term_info) = app.all_terms.get(&lower_part) {
                if term_info.is_rarity {
                    match lower_part.as_str() {
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
            };
            line.spans.push(Span::styled(*part, style));
        }

        if let Some(suggestion) = &app.suggestion {
            let last_part = parts.last().unwrap_or(&"").to_lowercase();
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

    // Main content area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[2]);

    // Results table
    render_table_view(f, app, main_chunks[0]);
    // Detail panel
    render_detail_panel(f, app, main_chunks[1]);

    // Status bar
    let status = Paragraph::new("Press ESC to exit | Ctrl + H for Help | Tab to cycle suggestions | ► to accept | Scroll or ▲▼ to select")
        .style(Style::default().fg(Color::LightBlue));
    f.render_widget(status, chunks[3]);
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
        .title("Details");
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if let Some(selected) = app.table_state.selected() {
        if let Some(skin) = app.results.get(selected) {
            let mut details = vec![
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

            if let Some(ref image_path) = skin.image_path {
                if let Some(ascii_art) = image_to_ascii(image_path, 40) {
                    // adjust width as needed
                    details.push(Line::from("")); // spacer
                    details.push(Line::from(Span::styled(
                        "Image:",
                        Style::default().fg(Color::Green),
                    )));
                    // Collect each line as an owned String.
                    let ascii_lines: Vec<String> =
                        ascii_art.lines().map(|line| line.to_string()).collect();
                    for line in ascii_lines {
                        details.push(Line::from(Span::raw(line)));
                    }
                }
            }

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
    let mut skins = vec![Skin {
        name: "No Dagger".to_string(),
        name_lower: "no dagger".to_string(),
        rarity: "Red".to_string(),
        rarity_lower: "red".to_string(),
        event: "April Fools".to_string(),
        event_lower: "april fools".to_string(),
        year: None,
        year_str: "".to_string(),
        tags: vec!["all case".to_string()],
        tags_lower: vec!["all case".to_string()]
            .into_iter()
            .map(|t| t.to_lowercase())
            .collect(),
        image_path: Some("https://i.ibb.co/ynPprrzM/IMG-0069.png".to_string()),
    }];

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
