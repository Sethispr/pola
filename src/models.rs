use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq)]
pub enum SortField {
    Name,
    Rarity,
    Event,
}

#[derive(Debug, Clone)]
pub struct Skin {
    pub name: String,
    pub name_lower: String,
    pub rarity: String,
    pub rarity_lower: String,
    pub event: String,
    pub event_lower: String,
    pub year: Option<u32>,
    pub year_str: String,
    pub tags: Vec<String>,
    pub tags_lower: HashSet<String>,
}

#[derive(Default)]
pub struct TermInfo {
    pub is_name: bool,
    pub is_event: bool,
    pub is_rarity: bool,
    pub is_tag: bool,
    pub is_year: bool,
}
