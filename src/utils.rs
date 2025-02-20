use crate::models::{Skin, TermInfo};
use std::collections::{HashMap, HashSet};

pub fn load_all_terms(skins: &[Skin]) -> HashMap<String, TermInfo> {
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

pub fn load_skins() -> Vec<Skin> {
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

pub fn search_skins(
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
