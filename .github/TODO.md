# pola To-Do List

## Performance Improvements

- Reduce memory allocations.  
- Use more cache-friendly data structures.  
- Replace **FuzzyMatcher** with **Nucleo** (8× faster).  
- Optimize index-based operations for efficiency.  
- Reuse the fuzzy matcher instance.  
- Use indices instead of cloning entire skin objects.  
- Compress term metadata with bitflags in **TermInfo** to reduce memory usage.  
- Improve tag handling, especially long ones (e.g., "Valentine Case (Exquisite)") by separating them into individual tags like "Exquisite" and "Valentine."  

## New TUI Features

- Add **value** and **owner** tags.  
- Allow tag exclusion using `--`, `?`, or `-`.  
- Enable combined rarity tags (e.g., searching "Pink" and "Red" should return both).
- Fix the **number of suggestions** feature. **(fixed)**
- Prevent non-tags from being mistakenly tagged.
- Implement a **custom keybind changer**. **(done)**
- Transition from **TUI** to **GUI** or a standalone terminal.  
- Expand **256-bit color support** from Ratatui (currently applied to FG, border color, and keybinds but not full BG). **(done)**
- Add **mod-only skins**. **(done)**
- Display skin descriptions in **detailed view**.
- Add **placeholder images** for all skins and begin populating the assets folder.
- Improve **auto-suggestions** (e.g., typing "Valentine" should suggest a skin from the Valentine Case). **(done)**
- Fix **auto-suggestion bugs**, such as lingering previous input when accepting a suggestion (e.g., typing "OMG" → accepting "Omega" leaves "OMG" in the search bar). **(fixed slightly)**
- Highlight search terms in suggested skins (similar to the website). **(done)**
- Display **10 results per table page** and allow navigation via keybinds or arrows (Multi-paginated view). **(done)**
- Add **logging** and **advanced error handling**.
- Implement a **performance monitoring** view with keybinds.
- Add a **scrollbar**. **(done)**
- Modernize UI and improve compatibility with different terminals.
- Refactor code into separate files instead of a single **main.rs** file.
- Introduce a **caret** in the search bar. **(done)**
- Keybind to toggle **detailed view** (default: ON). **(done)**
- **Ctrl + R** to select a random skin.  
- Improve **contextual suggestions** (e.g., differentiating between "Valentine Case" and "Valentine Bundle"). **(done)** 
- Allow **year-based searching** (e.g., `2022-2025`). **(done)**
- Ensure skins with the "Popular" tag appear first in **name-based searches**.
- Improve **autocomplete logic** (e.g., searching "Red S" should return skins like "Salmon" or "Sanctum"). **(done)**  
- Implement **multilingual support** (Spanish, Russian, etc.) with a language config option.  
- Add a **favorites** system and the ability to create/edit tags (e.g., "Favorite," "For Trade"). **(done for favorites)**
- Support **light mode and high contrast themes** via persistent background color settings.  
- Enable **importing skin data** from CSV or JSON.  
- Allow selecting multiple skins via **Shift + Up/Down** for exporting, favoriting, or comparison.  
- Display **search result statistics** and overall skin stats.  
- Add export functionality for **CSV, JSON, and TXT formats**.  
- Add an **"X" icon** in the search bar to clear input.  
- Introduce logical search operators (`AND`, `OR`, `NOT`).  
- Display **current shop listings** (requires live backend).  
- Add **search history** with a clear history option.  
- Fix block styling when displaying **multiple tags** (e.g., "Valentine/Birthday Exquisite Case Skins").  
- Paginate search results (10 skins per page) to resolve **click offset issues** (partially implemented).
- Prioritize **suggestions with more matching outcomes** (e.g., suggesting "Energy" over "Enforcer").  
- Ensure sorting logic prioritizes matches over alphabetic order (e.g., searching "er" shouldn’t return "Azurite" first).  
- Fix **scrolling bugs** where accidental middle-mouse clicks break scrolling behavior.  
- Improve **table sorting** to allow multi-sorting and remove default ascending/descending states.  
- Add shader effects using **TachyonFX** ([GitHub](https://github.com/junkdog/tachyonfx)).  
- Allow users to create **custom tags** (e.g., **Ctrl + T** to create a tag with color/style options, **Shift + T** to apply tags).  
- Enable sorting skins by **year** in table headers.  
- Implement a **filter menu** with buttons for including/excluding tags (similar to the website).  
- **Ctrl + J** to jump to a specific skin name.  
- Add **mouse-hover tooltips**.  
- **Ctrl + S** for a **stats panel**, including user-created tags.  
- Support **resizable layouts**.  
- Enhance **mouse interactions**, including clickable suggestions, close buttons (`X`), and panel controls.  
- Add **fast clickable filters** (e.g., clicking "Pink" instantly filters pink skins).  
- Prevent **duplicate tags** in suggestions and block excessive suggestion acceptance spam.
- Make TUI "hackable"

---

## TUI Bug Fixes

- Improve **suggestions** (make them more contextual and ensure they clear input upon acceptance). **(fixed)**  
- Fix **sorting issues** (turning off detailed view triggers incorrect sorting due to state mismatches).  
- Resolve **Ctrl + Y (redo) not working**.  
- Fix **click offset bug** when there are more than 12 results.  
- Ensure the **number of suggestions feature** displays correctly **(fixed)**.  
- Prevent the **caret from appearing in block-styled tags** when scrolling fast.
- Fix the scrollbar issues

---

### Website To-Do List

- Round only the **top-left** border for skin images (full rounding looks bad).
- Optimize **suggestion speed**.  
- Add missing images for the few skins currently on the website.  
- Populate **all skins** and their images in assets. **(done)**
- Fix **tags/help modal** and include **rarity, value, owner, and year tags**. **(done)**
- Gather an **owner list** via Trade-SA or user input.
- Remove **redundant info** (name and rarity) from **detailed view** (already visible in skin image).  
- Set a **max-width** for images.
- Capitalize the **first letter** of all tags. **(done)**

---

## pola-cli To-Do List

- Add an **animated ASCII intro**. **(done)**
- Feature a **"Skin of the Day"**.  
- Implement a **random skin generator**.  
- Support **exporting results**.  
- Allow favoriting skins.  
- Introduce **search operators** (`AND`, `OR`, `NOT`).  
- Use **ANSI colors**.  
- Add **search history** with a clear history function. **(done)**
- Display **numbered results** for quick selection.
- Add a **compact view** mode.  
- Implement a **value converter** (e.g., "Pink = 4 Reds = 256 Blues").  
- Support **paginated results**. **(done)**
- Make **history navigation more intuitive** (typing a number like `1-1999` should return past searches). **(done)**

---

## GitHub To-Do List

- Clean up **tables** in `skin.md`. **(done)**
- Move all GitHub-related `.md` files into a `.github` folder. **(done)**

---
