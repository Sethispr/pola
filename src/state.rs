use crate::models::{Skin, SortField, TermInfo};
use crate::utils::{load_all_terms, load_skins, search_skins};
use std::collections::{HashMap, HashSet};

pub struct AppState {
    pub input: String,
    pub skins: Vec<Skin>,
    pub results: Vec<Skin>,
    pub table_state: TableState,
    pub all_terms: HashMap<String, TermInfo>,
    pub suggestion: Option<String>,
    pub suggestion_list: Vec<String>,
    pub suggestion_index: usize,
    pub name_map: HashMap<String, usize>,
    pub input_history: Vec<String>,
    pub history_index: usize,
    pub scroll_offset: usize,
    pub sort_field: SortField,
    pub sort_descending: bool,
    pub show_detail: bool,
}

use fuzzy_matcher::skim::SkimMatcherV2;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct AppState {
    pub input: String,
    pub skins: Vec<Skin>,
    pub results: Vec<Skin>,
    pub table_state: TableState,
    pub all_terms: HashMap<String, TermInfo>,
    pub suggestion: Option<String>,
    pub suggestion_list: Vec<String>,
    pub suggestion_index: usize,
    pub name_map: HashMap<String, usize>,
    pub input_history: Vec<String>,
    pub history_index: usize,
    pub scroll_offset: usize,
    pub sort_field: SortField,
    pub sort_descending: bool,
    pub show_detail: bool,
}

impl AppState {
    // Constructor method
    pub fn new() -> Self {
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

    // Update search based on input
    pub fn update_search(&mut self) {
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

    // Sort the results based on the current sorting field
    pub fn sort_results(&mut self) {
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

    // Toggle the sorting direction for a specific field
    pub fn toggle_sort(&mut self, field: SortField) {
        if self.sort_field == field {
            self.sort_descending = !self.sort_descending;
        } else {
            self.sort_field = field;
            self.sort_descending = true;
        }
        self.sort_results();
    }

    // Update suggestions based on the input
    pub fn update_suggestion(&mut self) {
        let input_parts: Vec<&str> = self.input.split_whitespace().collect();
        let last_part = input_parts.last().cloned().unwrap_or("");
        let last_part_lower = last_part.to_lowercase();
        self.suggestion_list.clear();
        self.suggestion_index = 0;

        if !last_part_lower.is_empty() {
            let matcher = SkimMatcherV2::default();
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

    // Cycle through suggestions based on direction
    pub fn cycle_suggestion(&mut self, direction: i32) {
        if !self.suggestion_list.is_empty() {
            self.suggestion_index = (self.suggestion_index as i32 + direction)
                .rem_euclid(self.suggestion_list.len() as i32)
                as usize;
            self.suggestion = Some(self.suggestion_list[self.suggestion_index].clone());
        }
    }

    // Accept the current suggestion
    pub fn accept_suggestion(&mut self) {
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

    // Select next result in the table
    pub fn next(&mut self) {
        let i = self.table_state.selected().map_or(0, |i| {
            if i + 1 < self.results.len() {
                i + 1
            } else {
                i
            }
        });
        self.table_state.select(Some(i));
    }

    // Select previous result in the table
    pub fn previous(&mut self) {
        let i = self
            .table_state
            .selected()
            .map_or(0, |i| if i > 0 { i - 1 } else { 0 });
        self.table_state.select(Some(i));
    }

    // Record the current input for history
    pub fn record_input(&mut self) {
        if self.history_index < self.input_history.len() - 1 {
            self.input_history.truncate(self.history_index + 1);
        }
        if self.input_history.last() != Some(&self.input) {
            self.input_history.push(self.input.clone());
            self.history_index = self.input_history.len() - 1;
        }
    }

    // Undo the last input
    pub fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.input = self.input_history[self.history_index].clone();
            self.update_search();
        }
    }

    // Redo the last undone input
    pub fn redo(&mut self) {
        if self.history_index + 1 < self.input_history.len() {
            self.history_index += 1;
            self.input = self.input_history[self.history_index].clone();
            self.update_search();
        }
    }
}
