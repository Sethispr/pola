use dirs;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

const POLA_ASCII: &str = r#"
 ________  ________  ___       ________ 
|\   __  \|\   __  \|\  \     |\   __  \    
\ \  \|\  \ \  \|\  \ \  \    \ \  \|\  \   
 \ \   ____\ \  \\\  \ \  \    \ \   __  \  
  \ \  \___|\ \  \\\  \ \  \____\ \  \ \  \ 
   \ \__\    \ \_______\ \_______\ \__\ \__\
    \|__|     \|_______|\|_______|\|__|\|__|
"#;

const FAV_INDICATOR: &str = "★";
const UNFAV_INDICATOR: &str = "☆";

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

struct History {
    entries: Vec<String>,
    file_path: PathBuf,
}

impl History {
    fn new() -> Self {
        let path = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(".pola_history");
        let entries = if let Ok(content) = std::fs::read_to_string(&path) {
            content.lines().map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        };
        Self {
            entries,
            file_path: path,
        }
    }

    fn add(&mut self, query: String) {
        self.entries.push(query.clone());
        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)
        {
            let _ = writeln!(file, "{}", query);
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
        let _ = std::fs::write(&self.file_path, "");
    }

    fn get_entries(&self) -> &[String] {
        &self.entries
    }
}

#[derive(Debug)]
struct Favorites {
    skins: HashSet<String>,
    file_path: PathBuf,
}

impl Favorites {
    fn new() -> Self {
        let path = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(".pola_favorites");
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let skins = content.lines().map(|s| s.trim().to_lowercase()).collect();

        Self {
            skins,
            file_path: path,
        }
    }

    fn add(&mut self, skin_name: &str) {
        let name = skin_name.to_lowercase();
        if self.skins.insert(name.clone()) {
            self.save();
        }
    }

    fn remove(&mut self, skin_name: &str) {
        let name = skin_name.to_lowercase();
        if self.skins.remove(&name) {
            self.save();
        }
    }

    fn clear(&mut self) {
        self.skins.clear();
        self.save();
    }

    fn contains(&self, skin_name: &str) -> bool {
        self.skins.contains(&skin_name.to_lowercase())
    }

    fn save(&self) {
        let _ = std::fs::write(
            &self.file_path,
            self.skins.iter().cloned().collect::<Vec<_>>().join("\n"),
        );
    }
}

fn main() {
    let skins = load_skins();
    let name_map: HashMap<String, usize> = skins
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name_lower.clone(), i))
        .collect();
    let mut history = History::new();
    let mut favorites = Favorites::new();

    println!("{}", POLA_ASCII);
    println!("Welcome to Pola CLI!");
    println!(
        "Enter search terms separated by spaces to find matching skins. (ex: pink summer void)"
    );
    println!("- Leave blank to see all skins.");
    println!("- Type 'exit' to quit.");
    println!("- Type 'help' for commands, 'clear' to clear the screen, and 'about' for more info.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let trimmed_input = input.trim();
        let trimmed_lower = trimmed_input.to_lowercase();

        match trimmed_lower.as_str() {
            "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                print_help();
                continue;
            }
            "clear" => {
                clear_screen();
                continue;
            }
            "about" => {
                print_about();
                continue;
            }
            "history" => {
                display_history(&mut history, &skins, &name_map, &favorites);
                continue;
            }
            "clearhistory" => {
                history.clear();
                println!("History cleared.");
                continue;
            }
            "favorites" => {
                display_favorites(&favorites, &skins);
                continue;
            }
            "clearfavorites" => {
                favorites.clear();
                println!("Favorites cleared.");
                continue;
            }
            "stats" => {
                println!("Total skins loaded: {}", skins.len());
                println!("Favorites count: {}", favorites.skins.len());
                println!("History count: {}", history.entries.len());
                continue;
            }
            _ => {
                if trimmed_lower.starts_with("fav ") {
                    handle_favorite_command(&trimmed_input, &skins, &mut favorites);
                } else if trimmed_lower.starts_with("unfav ") {
                    handle_unfavorite_command(&trimmed_input, &mut favorites);
                } else if let Some(query) = check_rerun_command(&trimmed_lower, &history) {
                    process_query(query, &skins, &name_map, &favorites);
                    history.add(trimmed_input.to_string());
                } else {
                    process_query(trimmed_input.to_string(), &skins, &name_map, &favorites);
                    if !trimmed_input.is_empty() {
                        history.add(trimmed_input.to_string());
                    }
                }
            }
        }
    }
}

fn handle_favorite_command(input: &str, skins: &[Skin], favorites: &mut Favorites) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: fav <skin-name>");
        return;
    }

    let skin_name = parts[1..].join(" ");
    if let Some(_) = skins
        .iter()
        .find(|s| s.name.eq_ignore_ascii_case(&skin_name))
    {
        favorites.add(&skin_name);
        println!("Added '{}' to favorites {}", skin_name, FAV_INDICATOR);
    } else {
        println!("Skin '{}' not found", skin_name);
    }
}

fn handle_unfavorite_command(input: &str, favorites: &mut Favorites) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: unfav <skin-name>");
        return;
    }

    let skin_name = parts[1..].join(" ");
    favorites.remove(&skin_name);
    println!("Removed '{}' from favorites {}", skin_name, UNFAV_INDICATOR);
}

fn display_favorites(favorites: &Favorites, skins: &[Skin]) {
    let fav_skins: Vec<&Skin> = skins
        .iter()
        .filter(|s| favorites.contains(&s.name))
        .collect();

    if fav_skins.is_empty() {
        println!("No favorited skins.");
        return;
    }

    println!("\nFavorited Skins ({}):", fav_skins.len());
    display_results(fav_skins.into_iter().cloned().collect(), favorites);
}

fn display_results(results: Vec<Skin>, favorites: &Favorites) {
    if results.is_empty() {
        println!("No skins found matching your search.");
        return;
    }

    let page_size = 10; // Number of results per page
    let mut page = 0;

    loop {
        let start = page * page_size;
        let end = (page + 1) * page_size;
        let page_results = results
            .get(start..std::cmp::min(end, results.len()))
            .unwrap_or(&[]);

        println!("\nFound {} skins (Page {}):\n", results.len(), page + 1);
        for skin in page_results {
            let fav_status = if favorites.contains(&skin.name) {
                FAV_INDICATOR
            } else {
                UNFAV_INDICATOR
            };
            println!("{} Name: {}", fav_status, skin.name);
            println!("Rarity: {}", skin.rarity);
            println!("Event: {}", skin.event);
            println!(
                "Year: {}",
                skin.year.map_or("N/A".to_string(), |y| y.to_string())
            );
            println!("Tags: {}", skin.tags.join(", "));
            println!("------------------------------");
        }

        // Check if there are more results to display
        if end >= results.len() {
            break;
        }

        // Prompt for next page or quit
        println!("\nPress Enter to see the next page, or 'q' to quit.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        page += 1;
    }
}

fn check_rerun_command(input: &str, history: &History) -> Option<String> {
    if input.starts_with('!') {
        let id_str = &input[1..];
        if let Ok(id) = id_str.parse::<usize>() {
            if id > 0 && id <= history.get_entries().len() {
                return Some(history.get_entries()[id - 1].clone());
            }
        }
    }
    None
}

fn process_query(
    query: String,
    skins: &[Skin],
    name_map: &HashMap<String, usize>,
    favorites: &Favorites,
) {
    let query_lower = query.to_lowercase();
    let tags: HashSet<&str> = query_lower.split_whitespace().collect();
    let results = if tags.is_empty() {
        skins.to_vec()
    } else {
        search_skins(skins, name_map, &tags)
    };
    display_results(results, favorites);
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

    let matcher = SkimMatcherV2::default();
    let mut scored_skins: Vec<(i64, &Skin)> = skins
        .iter()
        .filter_map(|skin| {
            let all_tags_matched = tags.iter().all(|&tag| {
                skin.name_lower.contains(tag)
                    || skin.rarity_lower == tag
                    || skin.event_lower.contains(tag)
                    || skin.year_str.contains(tag)
                    || skin.tags_lower.iter().any(|t| t.contains(tag))
            });

            if !all_tags_matched {
                return None;
            }

            let mut score = 0;
            for tag in tags {
                if skin.rarity_lower == *tag {
                    score += 1000;
                }
                if skin.year_str == *tag {
                    score += 800;
                }
                if skin.tags_lower.contains(*tag) {
                    score += 600;
                }
                if let Some(s) = matcher.fuzzy_match(&skin.name_lower, tag) {
                    score += s;
                }
                if let Some(s) = matcher.fuzzy_match(&skin.event_lower, tag) {
                    score += s;
                }
            }
            Some((score, skin))
        })
        .collect();

    scored_skins.sort_by(|a, b| b.0.cmp(&a.0));
    scored_skins.into_iter().map(|(_, s)| s.clone()).collect()
}

fn print_help() {
    println!("\nAvailable Commands:");
    println!("  help          - Show this help message");
    println!("  clear         - Clear the screen");
    println!("  about         - Display information about Pola CLI");
    println!("  exit          - Quit the application");
    println!("  history       - View search history");
    println!("  clearhistory  - Clear search history");
    println!("  fav <skin>    - Add skin to favorites");
    println!("  unfav <skin>  - Remove skin from favorites");
    println!("  favorites     - List favorited skins");
    println!("  clearfavorites - Clear all favorites");
    println!("  [text]        - Any other text is treated as search terms/tags\n");
}

fn print_about() {
    println!("{}", POLA_ASCII);
    println!("\nPola CLI – Fast SA Skin Search");
    println!("Version: 0.1.0-alpha");
    println!("For traders, made by a fellow trader! ");
    println!("Enjoy easily searching skins you need information about\n");
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
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
            tags: vec!["case".to_string(), "popular".to_string()],
            tags_lower: vec!["case".to_string(), "popular".to_string()]
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
            event: "Code".to_string(),
            event_lower: "code".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string(), "gamenight".to_string()],
            tags_lower: vec!["code".to_string(), "gamenight".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Butter".to_string(),
            name_lower: "butter".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code".to_string(),
            event_lower: "code".to_string(),
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
            event: "Code".to_string(),
            event_lower: "code".to_string(),
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
            event: "Code".to_string(),
            event_lower: "code".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string(), "gamenight".to_string()],
            tags_lower: vec!["code".to_string(), "gamenight".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Tin".to_string(),
            name_lower: "tin".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Code".to_string(),
            event_lower: "code".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["code".to_string(), "gamenight".to_string()],
            tags_lower: vec!["code".to_string(), "gamenight".to_string()]
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
            tags: vec![
                "case".to_string(),
                "exquisite".to_string(),
                "popular".to_string(),
            ],
            tags_lower: vec![
                "case".to_string(),
                "exquisite".to_string(),
                "popular".to_string(),
            ]
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
            name: "Orinthian".to_string(),
            name_lower: "orinthian".to_string(),
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
            tags: vec!["all case".to_string(), "popular".to_string()],
            tags_lower: vec!["all case".to_string(), "popular".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Whiteheart".to_string(),
            name_lower: "whiteheart".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Worthy Individuals".to_string(),
            event_lower: "worthy individuals".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["special".to_string(), "rare".to_string()],
            tags_lower: vec!["special".to_string(), "rare".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Darkheart".to_string(),
            name_lower: "Darkheart".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Worthy Individuals".to_string(),
            event_lower: "worthy individuals".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["special".to_string()],
            tags_lower: vec!["special".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Banana".to_string(),
            name_lower: "banana".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Pre-release Tester".to_string(),
            event_lower: "pre-release tester".to_string(),
            year: None,
            year_str: "2021".to_string(),
            tags: vec!["special".to_string()],
            tags_lower: vec!["special".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Hammer".to_string(),
            name_lower: "hammer".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Builder".to_string(),
            event_lower: "builder".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["special".to_string()],
            tags_lower: vec!["special".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Paintbrush".to_string(),
            name_lower: "paintbrush".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Artist".to_string(),
            event_lower: "artist".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["special".to_string()],
            tags_lower: vec!["special".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "Riddling".to_string(),
            name_lower: "riddling".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "Worthy Individuals".to_string(),
            event_lower: "worthy individuals".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["special".to_string()],
            tags_lower: vec!["special".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
        Skin {
            name: "VIP".to_string(),
            name_lower: "vip".to_string(),
            rarity: "Teal".to_string(),
            rarity_lower: "teal".to_string(),
            event: "VIP Players".to_string(),
            event_lower: "vip players".to_string(),
            year: None,
            year_str: "".to_string(),
            tags: vec!["special".to_string(), "VIP".to_string()],
            tags_lower: vec!["special".to_string(), "vip".to_string()]
                .into_iter()
                .map(|t| t.to_lowercase())
                .collect(),
        },
    ];

    // Post-process skins
    for skin in &mut skins {
        skin.name_lower = skin.name.to_lowercase();
        skin.rarity_lower = skin.rarity.to_lowercase();
        skin.event_lower = skin.event.to_lowercase();
        skin.year_str = skin.year.map(|y| y.to_string()).unwrap_or_default();
        skin.tags_lower = skin.tags.iter().map(|t| t.to_lowercase()).collect();
    }

    skins
}

fn display_history(
    history: &mut History,
    skins: &[Skin],
    name_map: &HashMap<String, usize>,
    favorites: &Favorites,
) {
    println!("\nSearch History:");
    for (i, entry) in history.get_entries().iter().enumerate() {
        println!("{:>4}: {}", i + 1, entry);
    }
    println!();

    println!("Enter a number to re-run a search, 'clearhistory' to clear the history, 'back' to return to the main prompt, or type a new search query.");
    loop {
        print!("history> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("back") {
            break;
        } else if trimmed.eq_ignore_ascii_case("clearhistory") {
            history.clear();
            println!("History cleared.");
            break;
        } else if let Ok(index) = trimmed.parse::<usize>() {
            if index > 0 && index <= history.get_entries().len() {
                let query = history.get_entries()[index - 1].clone();
                println!("Re-running search: {}", query);
                process_query(query, skins, name_map, favorites);
                break;
            } else {
                println!("Invalid history number. Please try again.");
            }
        } else {
            // Treat any other input as a new search query
            println!("Running search for: {}", trimmed);
            process_query(trimmed.to_string(), skins, name_map, favorites);
            history.add(trimmed.to_string());
            break;
        }
    }
}
