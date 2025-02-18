<p align="center">
  <img src="https://github.com/user-attachments/assets/687e92f5-9e79-4282-b9ad-6a088f185df8" alt="demo" width="700"> 
</p>

<center>
  <h3>
    <a href="https://sethispr.github.io/sadb/">Online Demo (Incomplete Skins)</a>
    <br><br>
    Rust TUI for searching all known skins in SA faster.
  </h3>
</center>


---

## Installation

Install the .exe from the <a href="https://github.com/Sethispr/sadb/releases/tag/v0.12a">Release Page</a> OR

> [!IMPORTANT]  
> You need to first install the latest stable version of Rust, Cargo and Git.

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
