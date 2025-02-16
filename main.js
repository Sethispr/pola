const MOCK_SKINS = [
  {
    name: "Cupid",
    rarity: "Pink",
    event: "Valentine Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Rainbow Periastron",
    rarity: "Pink",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Crimson Periastron",
    rarity: "Red",
    event: "Valentine Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Heartsong",
    rarity: "Red",
    event: "Valentine Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Ghostly",
    rarity: "Pink",
    event: "Birthday Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Spring Growth",
    rarity: "Pink",
    event: "Easter Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Midsummer",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "All Hallow's",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Evergreen",
    rarity: "Pink",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Azurite",
    rarity: "Pink",
    event: "Easter Event",
    year: 2022,
    tags: ["event"]
  },
  {
    name: "Cartoony Rainbow",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"]
  },
  {
    name: "Cupid's Revenge",
    rarity: "Teal",
    event: "Valentine Bundle",
    year: 2025,
    tags: ["bundle"]
  },
  {
    name: "Bubbles",
    rarity: "Teal",
    event: "Code Redeemed",
    year: null,
    tags: ["code"]
  },
  {
    name: "Blastoff",
    rarity: "Teal",
    event: "Launch",
    year: null,
    tags: ["launch"]
  },
  {
    name: "Dragon Slayer",
    rarity: "Legendary",
    event: "Summer Games",
    year: 2023,
    tags: ["Dragon", "Fantasy", "Epic"]
  },
  {
    name: "Neon Rider",
    rarity: "Epic",
    event: "Anniversary",
    year: 2022,
    tags: ["Cyberpunk", "Colorful"]
  },
  {
    name: "Ivory Periastron",
    rarity: "Red", 
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Diamond",
    rarity: "Red",
    event: "Valentine Case (Exquisite)", 
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Spider",
    rarity: "Pink",
    event: "Animal Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Unicorn",
    rarity: "Pink", 
    event: "Animal Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Monochrome",
    rarity: "Pink",
    event: "Pattern Case", 
    year: null,
    tags: ["case"]
  },
  {
    name: "Relic",
    rarity: "Red",
    event: "Pattern Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Archon",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Breaker",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  // Additional Skins
  {
    name: "Epicredness",
    rarity: "Red",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Hellfire",
    rarity: "Pink",
    event: "Birthday Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Surge",
    rarity: "Pink",
    event: "Birthday Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Epicblueness",
    rarity: "Red",
    event: "Birthday Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Golden",
    rarity: "Red",
    event: "Birthday Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Grimgold Periastron",
    rarity: "Red",
    event: "Birthday Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Amethyst Periastron",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Bunny",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Guitar",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Joyful Periastron",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Noir Periastron",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Mystic",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Void Lord",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Warlord",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Cythrex",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Dog",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Fire Wyvern",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Ghostfire",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Inscription",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Mummy",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Retrowave",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Shikai",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Anansi",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Dusekkar",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Count",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Dracula",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Hallowing",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Orange Energy",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Pumpkin",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Icycle",
    rarity: "Pink",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Santa",
    rarity: "Pink",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Candy Energy",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Festive Periastron",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Snowflake",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Snowman",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Corrupted",
    rarity: "Pink",
    event: "Easter Event",
    year: 2023,
    tags: ["event"]
  },
  {
    name: "Sun Slayer",
    rarity: "Pink",
    event: "Easter Event",
    year: 2024,
    tags: ["event"]
  },
  {
    name: "Cyberlight",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"]
  },
  {
    name: "Frostburn",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"]
  },
  {
    name: "Inferno Angel",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"]
  },
  {
    name: "Azure Dragon",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"]
  },
  {
    name: "Darkness",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"]
  },
  {
    name: "Vilethorn",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"]
  },
  {
    name: "Winged",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"]
  },
  {
    name: "Love Scepter",
    rarity: "Teal",
    event: "Valentine Bundle",
    year: 2025,
    tags: ["bundle"]
  },
  {
    name: "Wicked Rose",
    rarity: "Teal",
    event: "Valentine Bundle",
    year: 2025,
    tags: ["bundle"]
  },
  {
    name: "Redmaster",
    rarity: "Red",
    event: "Christmas Event",
    year: 2022,
    tags: ["event"]
  },
  {
    name: "Yellowflame",
    rarity: "Red",
    event: "Christmas Event",
    year: 2022,
    tags: ["event"]
  },
  {
    name: "Goldenrod",
    rarity: "Pink",
    event: "Christmas Event",
    year: 2022,
    tags: ["event"]
  },
  {
    name: "Whisper",
    rarity: "Pink",
    event: "Christmas Event",
    year: 2022,
    tags: ["event"]
  },
  {
    name: "Gingerblade",
    rarity: "Teal",
    event: "Christmas Event",
    year: 2022,
    tags: ["event"]
  },
  {
    name: "Candy Cane",
    rarity: "Teal",
    event: "Christmas Event",
    year: 2023,
    tags: ["event"]
  },
  {
    name: "Iceblade",
    rarity: "Teal",
    event: "Christmas Event",
    year: 2024,
    tags: ["event"]
  },
  {
    name: "Butter",
    rarity: "Teal",
    event: "Code Redeemed",
    year: null,
    tags: ["code"]
  },
  {
    name: "Fireworks",
    rarity: "Teal",
    event: "Code Redeemed",
    year: null,
    tags: ["code"]
  },
  {
    name: "Pearl",
    rarity: "Teal",
    event: "Code Redeemed",
    year: null,
    tags: ["code"]
  },
  {
    name: "Tin",
    rarity: "Teal",
    event: "Code Redeemed",
    year: null,
    tags: ["code"]
  },
  {
    name: "Behemoth",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Blizzard",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Crescendo",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Demon",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Overseer",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Redcliff",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Skeletal",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Telamonster",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Unseen",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Bombastic",
    rarity: "Red",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Crimsonwrath",
    rarity: "Red",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Sanctum",
    rarity: "Red",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"]
  },
  {
    name: "Bacon",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Salmon",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Shark",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Slither",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Dragon's Forge",
    rarity: "Pink",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Glacial",
    rarity: "Pink",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Chartreuse Periastron",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Fallen",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Prehistoric",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Shadow",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Violet Energy",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Laser",
    rarity: "Pink",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Orinthan",
    rarity: "Pink",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Azure Periastron",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Celestial",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Galactic",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Green Energy",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Motherboard",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Omega",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Crystal",
    rarity: "Pink",
    event: "Material Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Adurite",
    rarity: "Red",
    event: "Material Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Bluesteel",
    rarity: "Red",
    event: "Material Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Wooden",
    rarity: "Red",
    event: "Material Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Crystallised",
    rarity: "Pink",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Elven",
    rarity: "Pink",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Molten",
    rarity: "Pink",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Autumnal",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Beach",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Breeze",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Earth",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Ocean",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Sorcus",
    rarity: "Red",
    event: "Pattern Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Divine",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Enforcer",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Frosted",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Hunter",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Neon",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Pharaoh",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Skyward",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  },
  {
    name: "Steampunk",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"]
  }
].sort((a, b) => a.name.localeCompare(b.name));

class AppState {
  constructor() {
    this.skins = MOCK_SKINS;
    this.results = [...this.skins];
    this.selectedIndex = 0;
    this.input = '';
    this.allTerms = this.loadAllTerms();
    this.suggestions = [];
    
    this.setupEventListeners();
    this.render();
    
    // Auto-focus search input on load
    document.getElementById('search-input').focus();
  }

  loadAllTerms() {
    const terms = new Set();
    this.skins.forEach(skin => {
      terms.add(skin.name.toLowerCase());
      skin.name.toLowerCase().split(' ').forEach(word => terms.add(word));
      terms.add(skin.event.toLowerCase());
      skin.event.toLowerCase().split(' ').forEach(word => terms.add(word));
      skin.tags.forEach(tag => terms.add(tag.toLowerCase()));
      terms.add(skin.rarity.toLowerCase());
      if (skin.year) terms.add(skin.year.toString());
    });
    return Array.from(terms);
  }

  updateSearch(value) {
    this.input = value;
    const terms = value.toLowerCase().split(' ');
    
    this.results = this.skins.filter(skin => {
      return terms.every(term => {
        if (!term) return true;
        const searchString = `${skin.name} ${skin.rarity} ${skin.event} ${skin.year || ''} ${skin.tags.join(' ')}`.toLowerCase();
        return searchString.includes(term);
      });
    });

    requestAnimationFrame(() => {
      this.updateSuggestion();
      this.selectedIndex = 0;
      this.render();
    });
  }

  updateSuggestion() {
    const lastTerm = this.input.split(' ').pop().toLowerCase();
    this.suggestions = [];

    if (lastTerm) {
      this.suggestions = fuzzysort.go(lastTerm, this.allTerms, {
        threshold: -10000,
        limit: 5
      }).map(match => ({
        term: match.target,
        highlighted: fuzzysort.highlight(match, '<span class="suggestion-highlight">', '</span>')
      }));
    }
  }

  acceptSuggestion(term) {
    const terms = this.input.split(' ');
    terms.pop();
    terms.push(term);
    this.input = terms.join(' ') + ' ';
    
    const searchInput = document.getElementById('search-input');
    searchInput.value = this.input;
    searchInput.focus();
    this.updateSearch(this.input);
  }

  clearSearch() {
    const searchInput = document.getElementById('search-input');
    searchInput.value = '';
    this.updateSearch('');
    searchInput.focus();
  }

  setupEventListeners() {
    const searchInput = document.getElementById('search-input');
    const clearButton = document.querySelector('.clear-icon');
    
    searchInput.addEventListener('input', (e) => this.updateSearch(e.target.value));
    
    searchInput.addEventListener('keydown', (e) => {
      if (e.key === 'Tab' && this.suggestions.length > 0) {
        e.preventDefault();
        this.acceptSuggestion(this.suggestions[0].term);
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (this.suggestions.length > 0) {
          const items = document.querySelectorAll('.suggestion-item');
          items[0]?.focus();
        } else {
          this.selectedIndex = Math.min(this.selectedIndex + 1, this.results.length - 1);
          this.render();
        }
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        this.selectedIndex = Math.max(this.selectedIndex - 1, 0);
        this.render();
      } else if (e.key === 'Escape') {
        e.preventDefault();
        this.clearSearch();
      }
    });

    clearButton.addEventListener('click', () => {
      this.clearSearch();
    });

    document.addEventListener('keydown', (e) => {
      if (e.key === '/') {
        e.preventDefault();
        searchInput.focus();
      }
      if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
        e.preventDefault();
      }
    });

    // Click event for focusing search
    document.querySelector('.search-container').addEventListener('click', () => {
      searchInput.focus();
    });

    document.addEventListener('click', (e) => {
      if (!e.target.closest('.input-wrapper')) {
        document.querySelector('.suggestions-container').style.display = 'none';
      }
    });
  }

  renderStats() {
    const statsEl = document.querySelector('.table-header');
    statsEl.textContent = `${this.results.length} results | Selected: ${this.selectedIndex + 1}`;
  }

  render() {
    this.renderTable();
    this.renderSuggestion();
    this.renderDetails();
    this.renderStats();
  }

  renderTable() {
    const tbody = document.querySelector('#results-table tbody');
    const html = this.results.map((skin, index) => `
      <tr class="${index === this.selectedIndex ? 'selected' : ''}" 
          onclick="app.selectedIndex = ${index}; app.render()">
        <td>${skin.name}</td>
        <td class="rarity-${skin.rarity}">${skin.rarity}</td>
        <td>${skin.event}</td>
        <td>${skin.year || 'N/A'}</td>
        <td>${skin.tags.join(', ')}</td>
      </tr>
    `).join('');
    
    tbody.innerHTML = html;
    
    // Use requestAnimationFrame for smooth scrolling
    if (this.selectedIndex >= 0) {
      requestAnimationFrame(() => {
        const selectedRow = tbody.children[this.selectedIndex];
        if (selectedRow) {
          selectedRow.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
        }
      });
    }
  }

  renderSuggestion() {
    const container = document.querySelector('.suggestions-container');
    if (!this.input || this.suggestions.length === 0) {
      container.style.display = 'none';
      return;
    }

    const html = this.suggestions.map(suggestion => `
      <div class="suggestion-item" data-term="${suggestion.term}">
        ${suggestion.highlighted || suggestion.term}
      </div>
    `).join('');

    container.innerHTML = html;
    container.style.display = 'block';
    
    // Add click handlers for suggestions
    container.querySelectorAll('.suggestion-item').forEach(item => {
      item.addEventListener('click', () => {
        this.acceptSuggestion(item.dataset.term);
      });
    });
  }

  renderDetails() {
    const detailsEl = document.getElementById('skin-details');
    const skin = this.results[this.selectedIndex];
    
    if (!skin) {
      detailsEl.innerHTML = '<p>No skin selected</p>';
      return;
    }

    detailsEl.innerHTML = `
      <div class="detail-image">
        <div class="placeholder-image">
          <i class="ri-image-line"></i>
          <span>No preview available</span>
        </div>
      </div>
      <div class="detail-content">
        <div class="detail-row">
          <span class="detail-label">Name:</span>
          <span>${skin.name}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Rarity:</span>
          <span class="rarity-${skin.rarity}">${skin.rarity}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Event:</span>
          <span>${skin.event}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Year:</span>
          <span>${skin.year || 'N/A'}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Tags:</span>
          <span>${skin.tags.join(', ')}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Value:</span>
          <span>N/A</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Notes:</span>
          <span>N/A</span>
        </div>
      </div>
    `;
  }
}

const app = new AppState();
window.app = app;
