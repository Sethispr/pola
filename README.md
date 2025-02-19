<p align="center">
  <img src="https://github.com/user-attachments/assets/687e92f5-9e79-4282-b9ad-6a088f185df8" alt="demo" width="700"> 
</p>

<p align="center">
  <a href="https://sethispr.github.io/sadb/"><strong>Online Demo (Incomplete Skins)</strong></a>
  <br><br>
  Rust TUI for searching all known skins in SA faster!
</p>

---

### Example Usage of Tags

```bash
# Search for skins with the case tag "summer", rarity tag "pink" and a skin containg the word "void"
Type: pink summer void
Result: Void Lord
```

<details>
<summary>Tags List</summary>

| Tag              | Description                         | Tag              | Description                         |
|-----------------|-------------------------------------|-----------------|-------------------------------------|
| <kbd>Event</kbd>       | Event skins                         | <kbd>Bundle</kbd>      | Bundle skins                        |
| <kbd>Code</kbd>        | Code-redeemed skins                 | <kbd>Launch</kbd>      | Skins obtained from game launch     |
| <kbd>Case</kbd>        | Case skins                          | <kbd>Red</kbd>         | Red skin rarity                     |
| <kbd>Pink</kbd>        | Pink skin rarity                    | <kbd>Teal</kbd>        | Teal skin rarity                    |
| <kbd>2022</kbd>       | 2022 skins                          | <kbd>2023</kbd>       | 2023 skins                          |
| <kbd>2024</kbd>       | 2024 skins                          | <kbd>2025</kbd>       | 2025 skins                          |
| <kbd>Valentine</kbd>  | Valentine case skins                | <kbd>Birthday</kbd>   | Birthday case skins                 |
| <kbd>Easter</kbd>     | Easter case skins                   | <kbd>Summer</kbd>     | Summer case skins                   |
| <kbd>Halloween</kbd>  | Halloween case skins                | <kbd>Christmas</kbd>  | Christmas case skins                |
| <kbd>Exquisite</kbd>  | Exquisite case skins                | <kbd>Animal</kbd>     | Skins from the Animal case          |
| <kbd>Camouflage</kbd> | Skins from the Camouflage case      | <kbd>Future</kbd>     | Skins from the Future case          |
| <kbd>Material</kbd>   | Skins from the Material case        | <kbd>Nature</kbd>     | Skins from the Nature case          |
| <kbd>Pattern</kbd>    | Skins from the Pattern case         | <kbd>Refined</kbd>    | Skins from the Refined case         |

</details>

<details>
<summary>Keybinds</summary>

| Bind                | Description                      | Bind                | Description                      |
|---------------------|--------------------------------|---------------------|--------------------------------|
| <kbd>ctrl+h</kbd>  | Show help                      | <kbd>▲</kbd> <kbd>▼</kbd>  | Navigate results              |
| <kbd>►</kbd>       | Accept suggestion              | <kbd>tab</kbd>      | Cycle suggestions              |
| <kbd>home</kbd>    | Go to first                    | <kbd>end</kbd>      | Go to last                     |
| <kbd>ctrl+l</kbd>  | Clear search input             | <kbd>esc</kbd>      | Close TUI/Help                 |
| <kbd>ctrl+y</kbd>  | Redo                           | <kbd>ctrl+z</kbd>   | Undo                            |

</details>

> [!TIP]
> Tags such as bundle, code, year, event, periastron, energy, duped, rare or popular are also accepted.
> 
> If installing isn't your thing, feel free to look at the [Online Demo](https://sethispr.github.io/sadb) or [Skin List](https://github.com/Sethispr/sadb/blob/main/skins.md)

---

## Installation

Install the .exe (not the zip) from the [Release Page](https://github.com/Sethispr/sadb/releases/tag/v0.12a) or follow these steps:

> [!NOTE]
> Windows Defender will flag the .exe as a potential threat since it's not signed. 
> 
> This is a false positive (and signing costs around $200). To proceed, click "More info" and then "Run anyway" to install.

---

Install the latest stable versions of [Rust](https://www.rust-lang.org/), [Cargo](https://doc.rust-lang.org/cargo/), and [Git](https://git-scm.com/) first.

1. Clone the repository:
   ```bash
   git clone https://github.com/sethispr/sadb
   cd sadb
   ```

2. Build and run the application
   ```bash
   cargo build --release
   cargo run --release
   ```

---
