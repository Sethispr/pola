<p align="center">
  <img src="https://github.com/user-attachments/assets/6a688773-c764-4dd4-969b-47035dc549eb" alt="demo" width="600">

<p align="center">
    Rust TUI for searching all known skins in SA faster.
    [Online Demo (Unstable)](https://sethispr.github.io/sadb/)
    <br />
    </p>

---

## Features

- **Fuzzy Search**: Search skins using fuzzy matching for names, tags, and events (with a scoring system for better sorting).
- **Interactive UI**: Navigate through search results using keyboard shortcuts.
- **Detailed View**: View detailed information about a selected skin, including value of item and current known owners.
- **Tag-Based Filtering**: Filter skins by single or multiple tags.
- **Suggestions**: Get real-time suggestions for search terms based on available data. (Only Teals, Pinks and Red)

---

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Git

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/sethispr/sadb
   cd sadb
   ```

2. Build and run the application
   ```bash
   cargo build
   cargo run
   ```
   
---

### Keyboard Shortcuts

- **Tab**: Automatically fills in the suggested search term.
- **Up/Down Arrows (or mouse scroll)**: Navigate through the search results.
- **Home/End**: Jump to the first or last result.
- **Enter**: View detailed information about the selected skin.
- **Ctrl+L**: Clear the search input.
- **Esc**: Exit the application.

---

### Example Usage of Tags

```bash
# Search for skins with the case tag "summer" and the rarity tag "pink"
Type: pink summer
```

---

### Frameworks

- [ratatui](https://github.com/tui-rs-revival/ratatui) for the terminal UI framework.
- [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal handling.
- [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher) for fuzzy search functionality.

---
