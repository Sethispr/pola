<div align="center">
  <video src="https://github.com/user-attachments/assets/bd971774-46bf-4335-b142-67b117d1db55" width="300" controls></video>
</div>

sadb is a terminal-based application or TUI for searching all known skins in SA (reds, pinks, teals). Blazing fast search speeds for filtering skins by name, rarity, event, year tags. The application is built using Rust and uses the `ratatui` crate.

---

## Features

- **Fuzzy Search**: Search skins using fuzzy matching for names, tags, and events (with a scoring system for better sorting).
- **Interactive UI**: Navigate through search results using keyboard shortcuts.
- **Detailed View**: View detailed information about a selected skin, including rarity, event, year, and tags.
- **Tag-Based Filtering**: Filter skins by single or multiple tags.
- **Suggestions**: Get real-time suggestions for search terms based on available data.

---

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/sethispr/sadb
   cd sadb
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

---

## Usage

### Keyboard Shortcuts

- **Type**: Start typing to search for skins.
- **Tab**: Accept the suggested search term.
- **Up/Down Arrows**: Navigate through the search results.
- **Home/End**: Jump to the first or last result.
- **Enter**: View detailed information about the selected skin.
- **Ctrl+L**: Clear the search input.
- **Esc**: Exit the application.

---

## Example

```bash
# Search for skins with the case tag "summer" and the rarity tag "pink"
Type: pink summer
```

---

## Frameworks

- [ratatui](https://github.com/tui-rs-revival/ratatui) for the terminal UI framework.
- [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling.
- [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher) for fuzzy search functionality.

---

Made by [sethyl](https://github.com/sethispr)

## High Priority

### 1. Gather High-Quality Skin Images
- Get **clear images** of each skin in-game (some sources: https://www.youtube.com/watch?v=iZGB3j8N9eQ)
- Contribution credits if applicable

### 2. Collect Case & Bundle Images
- List out **every case and bundle** in the game
- Gather images of **case designs** and Bundle image (?)
- Put **drop rates and contents** 

## Low Priority
- Orange - yellow - green - blue skins
- Trade simulation and automatic warning system ( warns if the username is flagged / trade is unfair
- Songs in sa?
- Add db to nerd discord bot? use the dropdown list embed for filtering
- Scammer/duper watchlist username
- Comment section/forum (no login required)
  
