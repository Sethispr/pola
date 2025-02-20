use crate::models::Skin;
use crate::state::AppState;
use ratatui::{prelude::*, widgets::*};

pub fn show_help<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
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

pub fn get_rarity_color(skin: &Skin) -> Color {
    match skin.rarity_lower.as_str() {
        "pink" => Color::Magenta,
        "red" => Color::Red,
        "teal" => Color::Cyan,
        _ => Color::White,
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut AppState) {
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
    let status = Line::from(vec![
        Span::styled(
            " ESC ",
            Style::default().bg(Color::DarkGray).fg(Color::White),
        ),
        Span::raw(" Exit  "),
        Span::styled(
            " CTRL+H ",
            Style::default().bg(Color::Blue).fg(Color::White),
        ),
        Span::raw(" Help  "),
        Span::styled(
            " TAB ",
            Style::default().bg(Color::Magenta).fg(Color::White),
        ),
        Span::raw(" Cycle suggestions  "),
        Span::styled(" ► ", Style::default().bg(Color::Green).fg(Color::Black)),
        Span::raw(" Accept "),
        Span::styled(
            " ▲/▼ ",
            Style::default().bg(Color::DarkGray).fg(Color::White),
        ),
        Span::raw(" Select"),
    ]);
    let status_bar = Paragraph::new(status)
        .style(Style::default())
        .alignment(Alignment::Center);
    f.render_widget(status_bar, chunks[3]);
}

pub fn get_term_style(term: &str, all_terms: &HashMap<String, TermInfo>) -> Style {
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

pub fn render_table_view<B: Backend>(f: &mut Frame<B>, app: &mut AppState, area: Rect) {
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

pub fn render_detail_panel<B: Backend>(f: &mut Frame<B>, app: &AppState, area: Rect) {
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
                Line::from(
                    std::iter::once(Span::styled("Tags: ", Style::default().fg(Color::Green)))
                        .chain(render_tags(&skin.tags))
                        .collect::<Vec<_>>(),
                ),
            ];

            let details_paragraph = Paragraph::new(details)
                .block(Block::default().borders(Borders::NONE))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(details_paragraph, inner_area);
        }
    }
}

pub fn render_tags(tags: &[String]) -> Vec<Span> {
    let mut spans = Vec::new();
    for tag in tags {
        spans.push(Span::styled(
            format!(" {} ", tag),
            Style::default()
                .bg(Color::DarkGray) // Background color for the block
                .fg(Color::White) // Text color
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(" ")); // Space between tags
    }
    if !spans.is_empty() {
        spans.pop(); // Remove the trailing space
    }
    spans
}
