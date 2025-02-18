<p align="center">
  <img src="https://github.com/user-attachments/assets/687e92f5-9e79-4282-b9ad-6a088f185df8" alt="demo" width="700"> 
</p>

<p align="center">
  <a href="https://sethispr.github.io/sadb/"><strong>Online Demo (Incomplete Skins)</strong></a>
  <br><br>
  Rust TUI for searching all known skins in SA faster!
</p>

---

## Installation

Install the .exe (not the zip) from the [Release Page](https://github.com/Sethispr/sadb/releases/tag/v0.12a) or follow these steps:

> [!NOTE]
> Windows Defender will flag the .exe as a potential threat since it's not signed. This is a false positive (and signing costs around $200).  
> To proceed, click "More info" and then "Run anyway" to install.

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

### Example Usage of Tags

```bash
# Search for skins with the case tag "summer", rarity tag "pink" and a skin containg the word "void"
Type: pink summer void
Result: Void Lord
```

> [!TIP]
> Tags such as bundle, code, year, event, periastron, energy, duped, rare or popular are also accepted.

---
