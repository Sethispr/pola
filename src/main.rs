use std::collections::HashSet;
use std::io;
use std::time::Duration;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::text::{Span, Line, Text};
use ratatui::widgets::Clear;
use ratatui::prelude::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::FuzzyMatcher;

#[derive(Debug, Clone)]
struct Skin {
    name: String,
    rarity: String,
    event: String,
    year: Option<u32>,
    tags: Vec<String>,
}

struct AppState {
    input: String,
    skins: Vec<Skin>,
    results: Vec<Skin>,
    list_state: ListState,
    all_terms: HashSet<String>,
    suggestion: Option<String>,
    show_detail: bool,
}

impl AppState {
    fn new() -> Self {
        let skins = load_skins(); // Assume this function is defined elsewhere
        let all_terms = load_all_terms(&skins);
        let results = skins.clone();
        AppState {
            input: String::new(),
            skins,
            results,
            list_state: ListState::default().with_selected(Some(0)),
            all_terms,
            suggestion: None,
            show_detail: false,
        }
    }

    fn update_search(&mut self) {
        let binding = self.input.to_lowercase();
        let tags: HashSet<&str> = binding.split_whitespace().collect();
        self.results = search_skins(&self.skins, &tags); // Assume this function is defined elsewhere
        self.list_state.select(Some(0));
        self.update_suggestion();
    }

    fn update_suggestion(&mut self) {
        let input_parts: Vec<&str> = self.input.split_whitespace().collect();
        let last_part = input_parts.last().cloned().unwrap_or("").to_lowercase();
        self.suggestion = None;

        if !last_part.is_empty() {
            let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
            let mut best_score = i64::MIN;
            let mut best_term = None;

            for term in &self.all_terms {
                if let Some(score) = matcher.fuzzy_match(term, &last_part) {
                    if score > best_score {
                        best_score = score;
                        best_term = Some(term.clone());
                    }
                }
            }

            self.suggestion = best_term;
        }
    }

    fn next(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected + 1 < self.results.len() {
                self.list_state.select(Some(selected + 1));
            }
        }
    }

    fn previous(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected > 0 {
                self.list_state.select(Some(selected - 1));
            }
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
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('l') {
                        app.input.clear();
                        app.update_search();
                        continue;
                    }
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.update_search();
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                            app.update_search();
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Home => app.list_state.select(Some(0)),
                        KeyCode::End => app.list_state.select(Some(app.results.len().saturating_sub(1))),
                        KeyCode::Tab => {
                            if let Some(suggestion) = &app.suggestion {
                                let mut parts: Vec<&str> = app.input.split_whitespace().collect();
                                if parts.is_empty() {
                                    app.input = format!("{} ", suggestion);
                                } else {
                                    parts.pop();
                                    parts.push(suggestion);
                                    app.input = parts.join(" ") + " ";
                                }
                                app.update_search();
                            }
                        }
                        KeyCode::Enter => {
                            if app.show_detail {
                                app.show_detail = false;
                            } else if app.list_state.selected().is_some() {
                                app.show_detail = true;
                            }
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollDown => app.next(),
                        MouseEventKind::ScrollUp => app.previous(),
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Search input with placeholder and suggestions
    let input_text = if app.input.is_empty() {
        Text::from(Line::from(Span::styled(
            "Type to search skins...",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        )))
    } else {
        let mut line = Line::default();
        let parts: Vec<&str> = app.input.split_whitespace().collect();
        let last_part = parts.last().cloned().unwrap_or("");
        let last_part_lower = last_part.to_lowercase();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                line.spans.push(Span::raw(" "));
            }
            line.spans.push(Span::raw(*part));
        }

        if let Some(suggestion) = &app.suggestion {
            if suggestion.starts_with(&last_part_lower) {
                let suffix = &suggestion[last_part.len()..];
                line.spans.push(Span::styled(
                    suffix,
                    Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
                ));
            }
        }

        Text::from(line)
    };

    let search_input = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search (e.g., 'pink summer')".bold()),
        );
    f.render_widget(search_input, chunks[0]);

    // Results list or no results message
    if app.show_detail {
        render_detail_view(f, app);
    } else {
        render_list_view(f, app, chunks[1]);
    }

    let status = Paragraph::new("Press ESC to exit | Tab to accept suggestion | Enter: Details")
        .style(Style::default().fg(Color::LightBlue));
    f.render_widget(status, chunks[2]);
}

fn render_list_view<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
    if app.results.is_empty() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Results: 0 | No matches found"))
            .border_style(Style::default().fg(Color::White));

        let message = Paragraph::new("No results found. Try a different search.")
            .block(block)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(message, area);
    } else {
        let items: Vec<ListItem> = app.results.iter().enumerate().map(|(i, skin)| {
            let is_selected = app.list_state.selected() == Some(i);
            let style = if is_selected {
                Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let name = skin.name.clone();
            let rarity = skin.rarity.clone();
            let event_name = skin.event.clone();
            let year = skin.year.map_or("N/A".to_string(), |y| y.to_string());
            let tags_text = skin.tags.join(", ");

            ListItem::new(Line::from(vec![
                Span::styled(name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(" ("),
                Span::styled(rarity, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::styled(event_name, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::styled(year, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                Span::raw(") "),
                Span::styled(tags_text, Style::default().fg(Color::DarkGray)),
            ])).style(style)
        }).collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(
                        "Results: {} | Selected: {} | ESC: Exit | Ctrl+L: Clear | Home/End: Jump",
                        app.results.len(),
                        app.list_state.selected().map(|i| i + 1).unwrap_or(0)
                    ))
                    .border_style(Style::default().fg(Color::White)),
            )
            .highlight_style(Style::default().bg(Color::Rgb(70, 70, 70)).add_modifier(Modifier::BOLD));
        f.render_stateful_widget(list, area, &mut app.list_state);
    }
}

fn render_detail_view<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
    let area = centered_rect(60, 60, f.size());
    f.render_widget(Clear, area);
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Skin Details")
        .style(Style::default().bg(Color::Rgb(30, 30, 30)));
    f.render_widget(block, area);

    if let Some(selected) = app.list_state.selected() {
        if let Some(skin) = app.results.get(selected) {
            let inner_area = area.inner(&Margin { vertical: 1, horizontal: 1 });
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Length(20), Constraint::Min(20)])
                .split(inner_area);

            // Image placeholder
            let image_block = Block::default()
                .borders(Borders::ALL)
                .title("Image")
                .style(Style::default().bg(Color::DarkGray));
            f.render_widget(image_block, chunks[0]);

            // Details text
            let details = vec![
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Cyan)),
                    Span::styled(&skin.name, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Rarity: ", Style::default().fg(Color::Yellow)),
                    Span::styled(&skin.rarity, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Event: ", Style::default().fg(Color::Magenta)),
                    Span::styled(&skin.event, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Year: ", Style::default().fg(Color::Blue)),
                    Span::styled(
                        skin.year.map_or(String::from("N/A"), |y| y.to_string()),
                        Style::default().fg(Color::White)
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Tags: ", Style::default().fg(Color::Green)),
                    Span::styled(
                        skin.tags.join(", "),
                        Style::default().fg(Color::White)
                    ),
                ]),
            ];

            let details_paragraph = Paragraph::new(details)
                .block(Block::default().borders(Borders::NONE))
                .alignment(Alignment::Left);
            f.render_widget(details_paragraph, chunks[1]);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn load_all_terms(skins: &[Skin]) -> HashSet<String> {
    let mut terms = HashSet::new();
    for skin in skins {
        terms.insert(skin.name.to_lowercase());
        for word in skin.name.to_lowercase().split_whitespace() {
            terms.insert(word.to_string());
        }
        terms.insert(skin.event.to_lowercase());
        for word in skin.event.to_lowercase().split_whitespace() {
            terms.insert(word.to_string());
        }
        for tag in &skin.tags {
            terms.insert(tag.to_lowercase());
        }
        terms.insert(skin.rarity.to_lowercase());
        if let Some(year) = skin.year {
            terms.insert(year.to_string());
        }
    }
    terms
}

fn load_skins() -> Vec<Skin> {
    vec![
        // Valentine Case
        Skin { name: "Cupid".to_string(), rarity: "Pink".to_string(), event: "Valentine Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Rainbow Periastron".to_string(), rarity: "Pink".to_string(), event: "Valentine Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Crimson Periastron".to_string(), rarity: "Red".to_string(), event: "Valentine Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Heartsong".to_string(), rarity: "Red".to_string(), event: "Valentine Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Ivory Periastron".to_string(), rarity: "Red".to_string(), event: "Valentine Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Diamond".to_string(), rarity: "Red".to_string(), event: "Valentine Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Epicredness".to_string(), rarity: "Red".to_string(), event: "Valentine Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },

        // Birthday Case
        Skin { name: "Ghostly".to_string(), rarity: "Pink".to_string(), event: "Birthday Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Hellfire".to_string(), rarity: "Pink".to_string(), event: "Birthday Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Surge".to_string(), rarity: "Pink".to_string(), event: "Birthday Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Epicblueness".to_string(), rarity: "Red".to_string(), event: "Birthday Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Golden".to_string(), rarity: "Red".to_string(), event: "Birthday Case (Exquisite)".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Grimgold Periastron".to_string(), rarity: "Red".to_string(), event: "Birthday Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Easter Case
        Skin { name: "Spring Growth".to_string(), rarity: "Pink".to_string(), event: "Easter Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Amethyst Periastron".to_string(), rarity: "Red".to_string(), event: "Easter Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Bunny".to_string(), rarity: "Red".to_string(), event: "Easter Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Guitar".to_string(), rarity: "Red".to_string(), event: "Easter Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Joyful Periastron".to_string(), rarity: "Red".to_string(), event: "Easter Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Noir Periastron".to_string(), rarity: "Red".to_string(), event: "Easter Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Summer Case
        Skin { name: "Midsummer".to_string(), rarity: "Pink".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Mystic".to_string(), rarity: "Pink".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Void Lord".to_string(), rarity: "Pink".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Warlord".to_string(), rarity: "Pink".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Cythrex".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Dog".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Fire Wyvern".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Ghostfire".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Inscription".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Mummy".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Retrowave".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Shikai".to_string(), rarity: "Red".to_string(), event: "Summer Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Halloween Case
        Skin { name: "All Hallow's".to_string(), rarity: "Pink".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Anansi".to_string(), rarity: "Pink".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Dusekkar".to_string(), rarity: "Pink".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Count".to_string(), rarity: "Red".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Dracula".to_string(), rarity: "Red".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Hallowing".to_string(), rarity: "Red".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Orange Energy".to_string(), rarity: "Red".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Pumpkin".to_string(), rarity: "Red".to_string(), event: "Halloween Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Christmas Case
        Skin { name: "Evergreen".to_string(), rarity: "Pink".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Icycle".to_string(), rarity: "Pink".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Santa".to_string(), rarity: "Pink".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Candy Energy".to_string(), rarity: "Red".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Festive Periastron".to_string(), rarity: "Red".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Snowflake".to_string(), rarity: "Red".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Snowman".to_string(), rarity: "Red".to_string(), event: "Christmas Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Easter Event
        Skin { name: "Azurite".to_string(), rarity: "Pink".to_string(), event: "Easter Event".to_string(), year: Some(2022), tags: vec!["event".to_string()] },
        Skin { name: "Corrupted".to_string(), rarity: "Pink".to_string(), event: "Easter Event".to_string(), year: Some(2023), tags: vec!["event".to_string()] },
        Skin { name: "Sun Slayer".to_string(), rarity: "Pink".to_string(), event: "Easter Event".to_string(), year: Some(2024), tags: vec!["event".to_string()] },

        // Summer Bundle
        Skin { name: "Cartoony Rainbow".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2023), tags: vec!["bundle".to_string()] },
        Skin { name: "Cyberlight".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2023), tags: vec!["bundle".to_string()] },
        Skin { name: "Frostburn".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2023), tags: vec!["bundle".to_string()] },
        Skin { name: "Inferno Angel".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2023), tags: vec!["bundle".to_string()] },
        Skin { name: "Azure Dragon".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2024), tags: vec!["bundle".to_string()] },
        Skin { name: "Darkness".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2024), tags: vec!["bundle".to_string()] },
        Skin { name: "Vilethorn".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2024), tags: vec!["bundle".to_string()] },
        Skin { name: "Winged".to_string(), rarity: "Teal".to_string(), event: "Summer Bundle".to_string(), year: Some(2024), tags: vec!["bundle".to_string()] },

        // Valentine Bundle
        Skin { name: "Cupid's Revenge".to_string(), rarity: "Teal".to_string(), event: "Valentine Bundle".to_string(), year: Some(2025), tags: vec!["bundle".to_string()] },
        Skin { name: "Love Scepter".to_string(), rarity: "Teal".to_string(), event: "Valentine Bundle".to_string(), year: Some(2025), tags: vec!["bundle".to_string()] },
        Skin { name: "Wicked Rose".to_string(), rarity: "Teal".to_string(), event: "Valentine Bundle".to_string(), year: Some(2025), tags: vec!["bundle".to_string()] },

        // Christmas Event
        Skin { name: "Redmaster".to_string(), rarity: "Red".to_string(), event: "Christmas Event".to_string(), year: Some(2022), tags: vec!["event".to_string()] },
        Skin { name: "Yellowflame".to_string(), rarity: "Red".to_string(), event: "Christmas Event".to_string(), year: Some(2022), tags: vec!["event".to_string()] },
        Skin { name: "Golden Rod".to_string(), rarity: "Pink".to_string(), event: "Christmas Event".to_string(), year: Some(2022), tags: vec!["event".to_string()] },
        Skin { name: "Whisper".to_string(), rarity: "Pink".to_string(), event: "Christmas Event".to_string(), year: Some(2022), tags: vec!["event".to_string()] },
        Skin { name: "Gingerblade".to_string(), rarity: "Teal".to_string(), event: "Christmas Event".to_string(), year: Some(2022), tags: vec!["event".to_string()] },
        Skin { name: "Candy Cane".to_string(), rarity: "Teal".to_string(), event: "Christmas Event".to_string(), year: Some(2023), tags: vec!["event".to_string()] },
        Skin { name: "Iceblade".to_string(), rarity: "Teal".to_string(), event: "Christmas Event".to_string(), year: Some(2024), tags: vec!["event".to_string()] },

        // Code Redeemed Skins (Teal)
        Skin { name: "Bubbles".to_string(), rarity: "Teal".to_string(), event: "Code Redeemed".to_string(), year: None, tags: vec!["code".to_string()] },
        Skin { name: "Butter".to_string(), rarity: "Teal".to_string(), event: "Code Redeemed".to_string(), year: None, tags: vec!["code".to_string()] },
        Skin { name: "Fireworks".to_string(), rarity: "Teal".to_string(), event: "Code Redeemed".to_string(), year: None, tags: vec!["code".to_string()] },
        Skin { name: "Pearl".to_string(), rarity: "Teal".to_string(), event: "Code Redeemed".to_string(), year: None, tags: vec!["code".to_string()] },
        Skin { name: "Tin".to_string(), rarity: "Teal".to_string(), event: "Code Redeemed".to_string(), year: None, tags: vec!["code".to_string()] },

        // Launch Skins (Teal)
        Skin { name: "Blastoff".to_string(), rarity: "Teal".to_string(), event: "Launch".to_string(), year: None, tags: vec!["launch".to_string()] },

        // Exquisite Case (Pinks & Reds)
        Skin { name: "Behemoth".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Blizzard".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Crescendo".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Demon".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Overseer".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Redcliff".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Skeletal".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Telamonster".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Unseen".to_string(), rarity: "Pink".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Bombastic".to_string(), rarity: "Red".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Crimsonwrath".to_string(), rarity: "Red".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },
        Skin { name: "Sanctum".to_string(), rarity: "Red".to_string(), event: "Exquisite Case".to_string(), year: None, tags: vec!["case".to_string(), "exquisite".to_string()] },

        // Animal Case
        Skin { name: "Spider".to_string(), rarity: "Pink".to_string(), event: "Animal Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Unicorn".to_string(), rarity: "Pink".to_string(), event: "Animal Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Bacon".to_string(), rarity: "Red".to_string(), event: "Animal Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Salmon".to_string(), rarity: "Red".to_string(), event: "Animal Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Shark".to_string(), rarity: "Red".to_string(), event: "Animal Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Slither".to_string(), rarity: "Red".to_string(), event: "Animal Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Camouflage Case
        Skin { name: "Dragon's Forge".to_string(), rarity: "Pink".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Glacial".to_string(), rarity: "Pink".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Chartreuse Periastron".to_string(), rarity: "Red".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Fallen".to_string(), rarity: "Red".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Prehistoric".to_string(), rarity: "Red".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Shadow".to_string(), rarity: "Red".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Violet Energy".to_string(), rarity: "Red".to_string(), event: "Camouflage Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Future Case
        Skin { name: "Laser".to_string(), rarity: "Pink".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Orinthan".to_string(), rarity: "Pink".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Azure Periastron".to_string(), rarity: "Red".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Celestial".to_string(), rarity: "Red".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Galactic".to_string(), rarity: "Red".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Green Energy".to_string(), rarity: "Red".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Motherboard".to_string(), rarity: "Red".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Omega".to_string(), rarity: "Red".to_string(), event: "Future Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Material Case
        Skin { name: "Crystal".to_string(), rarity: "Pink".to_string(), event: "Material Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Adurite".to_string(), rarity: "Red".to_string(), event: "Material Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Bluesteel".to_string(), rarity: "Red".to_string(), event: "Material Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Wooden".to_string(), rarity: "Red".to_string(), event: "Material Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Nature Case
        Skin { name: "Crystallised".to_string(), rarity: "Pink".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Elven".to_string(), rarity: "Pink".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Molten".to_string(), rarity: "Pink".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Autumnal".to_string(), rarity: "Red".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Beach".to_string(), rarity: "Red".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Breeze".to_string(), rarity: "Red".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Earth".to_string(), rarity: "Red".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Ocean".to_string(), rarity: "Red".to_string(), event: "Nature Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Pattern Case
        Skin { name: "Monochrome".to_string(), rarity: "Pink".to_string(), event: "Pattern Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Relic".to_string(), rarity: "Red".to_string(), event: "Pattern Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Sorcus".to_string(), rarity: "Red".to_string(), event: "Pattern Case".to_string(), year: None, tags: vec!["case".to_string()] },

        // Refined Case
        Skin { name: "Archon".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Breaker".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Divine".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Enforcer".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Frosted".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Hunter".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Neon".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Pharaoh".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Skyward".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
        Skin { name: "Steampunk".to_string(), rarity: "Red".to_string(), event: "Refined Case".to_string(), year: None, tags: vec!["case".to_string()] },
    ]
}
fn search_skins(skins: &[Skin], tags: &HashSet<&str>) -> Vec<Skin> {
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();

    // First, try to find exact name matches
    let exact_matches: Vec<Skin> = skins
        .iter()
        .filter(|skin| {
            tags.iter().any(|&tag| skin.name.to_lowercase() == tag)
        })
        .cloned()
        .collect();

    if !exact_matches.is_empty() {
        return exact_matches;
    }

    // If no exact matches, enforce AND logic for all tags
    let mut scored_skins: Vec<(i64, &Skin)> = skins
        .iter()
        .filter_map(|skin| {
            // Check if ALL tags match at least one field
            let all_tags_matched = tags.iter().all(|&tag| {
                skin.name.to_lowercase().contains(tag) ||
                skin.rarity.to_lowercase() == tag ||
                skin.event.to_lowercase().contains(tag) ||
                skin.year.map_or(false, |y| y.to_string() == tag) ||
                skin.tags.iter().any(|t| t.to_lowercase() == tag)
            });

            if !all_tags_matched {
                return None;
            }

            // Calculate relevance score for sorting
            let mut score = 0;
            for tag in tags {
                if skin.name.to_lowercase().contains(tag) {
                    score += 100; // Highest priority for name matches
                }
                if skin.rarity.to_lowercase() == *tag {
                    score += 80;
                }
                if skin.event.to_lowercase().contains(tag) {
                    score += 60;
                }
                if let Some(s) = matcher.fuzzy_match(&skin.name.to_lowercase(), tag) {
                    score += s; // Bonus for fuzzy name matches
                }
                if skin.tags.iter().any(|t| t.to_lowercase() == *tag) {
                    score += 40;
                }
                if skin.year.map_or(false, |y| y.to_string() == *tag) {
                    score += 20;
                }
            }

            Some((score, skin))
        })
        .collect();

    // Sort by score in descending order
    scored_skins.sort_by(|a, b| b.0.cmp(&a.0));

    scored_skins.into_iter().map(|(_, skin)| skin.clone()).collect()
}
