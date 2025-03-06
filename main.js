function debounce(func, delay) {
  let timer;
  return function(...args) {
    clearTimeout(timer);
    timer = setTimeout(() => func.apply(this, args), delay);
  }
}

const SKIN_COLLECTION = [
  {
    name: "Cupid",
    rarity: "Pink",
    event: "Valentine Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Rainbow Periastron",
    rarity: "Pink",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite", "periastron"],
    img: null
  },
  {
    name: "Crimson Periastron",
    rarity: "Red",
    event: "Valentine Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Heartsong",
    rarity: "Red",
    event: "Valentine Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Ivory Periastron",
    rarity: "Red",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite", "periastron"],
    img: null
  },
  {
    name: "Diamond",
    rarity: "Red",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Epicredness",
    rarity: "Red",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Ghostly",
    rarity: "Pink",
    event: "Birthday Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Hellfire",
    rarity: "Pink",
    event: "Birthday Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Surge",
    rarity: "Pink",
    event: "Birthday Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Epicblueness",
    rarity: "Red",
    event: "Birthday Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Golden",
    rarity: "Red",
    event: "Birthday Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Grimgold Periastron",
    rarity: "Red",
    event: "Birthday Case",
    year: null,
    tags: ["case", "periastron", "popular"],
    img: null
  },
  {
    name: "Spring Growth",
    rarity: "Pink",
    event: "Easter Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Amethyst Periastron",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Bunny",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Guitar",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Joyful Periastron",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Noir Periastron",
    rarity: "Red",
    event: "Easter Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Midsummer",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Mystic",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case", "popular"],
    img: null
  },
  {
    name: "Void Lord",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case", "popular"],
    img: null
  },
  {
    name: "Warlord",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Cythrex",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Dog",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case", "popular"],
    img: null
  },
  {
    name: "Fire Wyvern",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Ghostfire",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Inscription",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Mummy",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Retrowave",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Shikai",
    rarity: "Red",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "All Hallow's",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case", "popular"],
    img: null
  },
  {
    name: "Anansi",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Dusekkar",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case", "popular"],
    img: null
  },
  {
    name: "Count",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Dracula",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Hallowing",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Orange Energy",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case", "energy"],
    img: null
  },
  {
    name: "Pumpkin",
    rarity: "Red",
    event: "Halloween Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Evergreen",
    rarity: "Pink",
    event: "Christmas Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Icycle",
    rarity: "Pink",
    event: "Christmas Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Santa",
    rarity: "Pink",
    event: "Christmas Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Candy Energy",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case", "energy"],
    img: null
  },
  {
    name: "Festive Periastron",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Snowflake",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Snowman",
    rarity: "Red",
    event: "Christmas Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Azurite",
    rarity: "Pink",
    event: "Easter Event",
    year: 2022,
    tags: ["event", "popular"],
    img: null
  },
  {
    name: "Corrupted",
    rarity: "Teal",
    event: "Easter Event",
    year: 2023,
    tags: ["event", "popular"],
    img: null
  },
  {
    name: "Sun Slayer",
    rarity: "Pink",
    event: "Easter Event",
    year: 2024,
    tags: ["event"],
    img: null
  },
  {
    name: "Cartoony Rainbow",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Cyberlight",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Frostburn",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Inferno Angel",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2023,
    tags: ["bundle", "popular"],
    img: null
  },
  {
    name: "Azure Dragon",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Darkness",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Vilethorn",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Winged",
    rarity: "Teal",
    event: "Summer Bundle",
    year: 2024,
    tags: ["bundle", "popular"],
    img: null
  },
  {
    name: "Cupid's Revenge",
    rarity: "Teal",
    event: "Valentine Bundle",
    year: 2025,
    tags: ["bundle"],
    img: null
  },
  {
    name: "Love Scepter",
    rarity: "Teal",
    event: "Valentine Bundle",
    year: 2025,
    tags: ["bundle", "popular"],
    img: null
  },
  {
    name: "Wicked Rose",
    rarity: "Teal",
    event: "Valentine Bundle",
    year: 2025,
    tags: ["bundle", "popular"],
    img: null
  },
  {
    name: "Redmaster",
    rarity: "Red",
    event: "Christmas Event",
    year: 2022,
    tags: ["event", "rare"],
    img: null
  },
  {
    name: "Yellowflame",
    rarity: "Red",
    event: "Christmas Event",
    year: 2022,
    tags: ["event", "rare"],
    img: null
  },
  {
    name: "Goldenrod",
    rarity: "Pink",
    event: "Christmas Event",
    year: 2022,
    tags: ["event", "rare"],
    img: null
  },
  {
    name: "Whisper",
    rarity: "Pink",
    event: "Christmas Event",
    year: 2022,
    tags: ["event", "rare"],
    img: null
  },
  {
    name: "Gingerblade",
    rarity: "Teal",
    event: "Christmas Event",
    year: 2022,
    tags: ["event", "rare"],
    img: null
  },
  {
    name: "Candy Cane",
    rarity: "Teal",
    event: "Christmas Event",
    year: 2023,
    tags: ["event"],
    img: null
  },
  {
    name: "Iceblade",
    rarity: "Teal",
    event: "Christmas Event",
    year: 2024,
    tags: ["event", "popular"],
    img: null
  },
  {
    name: "Bubbles",
    rarity: "Teal",
    event: "Code",
    year: null,
    tags: ["code", "gamenight"],
    img: null
  },
  {
    name: "Butter",
    rarity: "Teal",
    event: "Code",
    year: null,
    tags: ["code", "duped"],
    img: null
  },
  {
    name: "Fireworks",
    rarity: "Teal",
    event: "Code",
    year: null,
    tags: ["code"],
    img: null
  },
  {
    name: "Pearl",
    rarity: "Teal",
    event: "Code",
    year: null,
    tags: ["code", "gamenight"],
    img: null
  },
  {
    name: "Tin",
    rarity: "Teal",
    event: "Code",
    year: null,
    tags: ["code", "gamenight"],
    img: null
  },
  {
    name: "Blastoff",
    rarity: "Teal",
    event: "Launch",
    year: null,
    tags: ["launch"],
    img: null
  },
  {
    name: "Behemoth",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Blizzard",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite", "popular"],
    img: null
  },
  {
    name: "Crescendo",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Demon",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: https://i.ibb.co/FLYCGZYg/IMG-0940-waifu2x-art-scan-noise3-scale-1.png
  },
  {
    name: "Overseer",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Redcliff",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Skeletal",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Telamonster",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Unseen",
    rarity: "Pink",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Bombastic",
    rarity: "Red",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Crimsonwrath",
    rarity: "Red",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Sanctum",
    rarity: "Red",
    event: "Exquisite Case",
    year: null,
    tags: ["case", "exquisite"],
    img: null
  },
  {
    name: "Spider",
    rarity: "Pink",
    event: "Animal Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Unicorn",
    rarity: "Pink",
    event: "Animal Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Bacon",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Salmon",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case", "popular"],
    img: null
  },
  {
    name: "Shark",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Slither",
    rarity: "Red",
    event: "Animal Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Dragon's Forge",
    rarity: "Pink",
    event: "Camouflage Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Glacial",
    rarity: "Pink",
    event: "Camouflage Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Chartreuse Periastron",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Fallen",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Prehistoric",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Shadow",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Violet Energy",
    rarity: "Red",
    event: "Camouflage Case",
    year: null,
    tags: ["case", "energy"],
    img: null
  },
  {
    name: "Laser",
    rarity: "Pink",
    event: "Future Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Orinthian",
    rarity: "Pink",
    event: "Future Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Azure Periastron",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case", "periastron"],
    img: null
  },
  {
    name: "Celestial",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Galactic",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Green Energy",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case", "energy"],
    img: null
  },
  {
    name: "Motherboard",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Omega",
    rarity: "Red",
    event: "Future Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Crystal",
    rarity: "Pink",
    event: "Material Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Adurite",
    rarity: "Red",
    event: "Material Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Bluesteel",
    rarity: "Red",
    event: "Material Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Wooden",
    rarity: "Red",
    event: "Material Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Crystallised",
    rarity: "Pink",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Elven",
    rarity: "Pink",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Molten",
    rarity: "Pink",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Autumnal",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Beach",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Breeze",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Earth",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Ocean",
    rarity: "Red",
    event: "Nature Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Monochrome",
    rarity: "Pink",
    event: "Pattern Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Relic",
    rarity: "Red",
    event: "Pattern Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Sorcus",
    rarity: "Red",
    event: "Pattern Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Archon",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Breaker",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Divine",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Enforcer",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Frosted",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Hunter",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Neon",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Pharaoh",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Skyward",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Steampunk",
    rarity: "Red",
    event: "Refined Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "No Dagger",
    rarity: "Red",
    event: "April Fools",
    year: null,
    tags: ["all case", "popular"],
    img: null
  },
  {
    name: "Whiteheart",
    rarity: "Teal",
    event: "Worthy Individuals",
    year: null,
    tags: ["special", "rare"],
    img: null
  },
  {
    name: "Darkheart",
    rarity: "Teal",
    event: "Worthy Individuals",
    year: null,
    tags: ["special"],
    img: null
  },
  {
    name: "Banana",
    rarity: "Teal",
    event: "Pre-release Tester",
    year: 2021,
    tags: ["special"],
    img: null
  },
  {
    name: "Hammer",
    rarity: "Teal",
    event: "Builder",
    year: null,
    tags: ["special"],
    img: null
  },
  {
    name: "Paintbrush",
    rarity: "Teal",
    event: "Artist",
    year: null,
    tags: ["special"],
    img: null
  },
  {
    name: "Riddling",
    rarity: "Teal",
    event: "Worthy Individuals",
    year: null,
    tags: ["special"],
    img: null
  },
  {
    name: "VIP",
    rarity: "Teal",
    event: "VIP Players",
    year: null,
    tags: ["special", "VIP"],
    img: null
  },
].sort((a, b) => a.name.localeCompare(b.name));

class AppState {
  constructor() {
    this.searchInput = document.getElementById('search-input');
    this.clearButton = document.querySelector('.clear-icon');
    this.suggestionsContainer = document.querySelector('.suggestions-container');
    this.tbody = document.querySelector('#results-table tbody');
    this.tableHeader = document.querySelector('.table-header');
    this.detailsEl = document.getElementById('skin-details');
    this.searchStats = document.getElementById('search-stats');

    this.skins = SKIN_COLLECTION.map(skin => ({ ...skin, favorite: skin.favorite || false }));
    const storedFavorites = localStorage.getItem('favoriteSkins');
    if (storedFavorites) {
      const favNames = JSON.parse(storedFavorites);
      this.skins = this.skins.map(skin => ({ ...skin, favorite: favNames.includes(skin.name) }));
    }
    this.results = [...this.skins];
    this.selectedIndex = 0;
    this.input = '';
    this.sortKey = '';
    this.sortOrder = 1;
    this.favoritesOnly = false;
    this.allTerms = this.loadAllTerms();
    this.suggestions = [];
    this.detailsVisible = true;
    this.suggestionsCache = {};

    // Tag and Case Filters
    this.tagFilters = { include: [], exclude: [] };
    this.caseFilters = { include: [], exclude: [] };
    this.rarityFilters = { include: [], exclude: [] };
    this.yearFilters = { include: [], exclude: [] };
    this.eventFilters = { include: [], exclude: [] };

    // Pagination state
    this.currentPage = 1;
    this.itemsPerPage = 10;

    // Themes configuration
    this.themes = [
      {
        id: 'ocean',
        name: 'Ocean Depths',
        colors: ['#00b4d8', '#90e0ef', '#03045e'],
        isDark: true
      },
      {
        id: 'lavender',
        name: 'Taro Dream',
        colors: ['#b3baff', '#d8deff', '#2c2440'],
        isDark: true
      },
      {
        id: 'mizu',
        name: 'Mizu Blue',
        colors: ['#5fb8ff', '#bae6ff', '#164875'],
        isDark: true
      },
      {
        id: 'cafe',
        name: 'Café Vintage',
        colors: ['#c8976c', '#e6d2b5', '#2a1f17'],
        isDark: true
      },
      {
        id: 'cherry',
        name: 'Cherry Blossom',
        colors: ['#ff5d8f', '#ffa5b9', '#2d1a24'],
        isDark: true
      },
      {
        id: 'electric',
        name: 'Electric Violet',
        colors: ['#b14aed', '#d58bff', '#29153d'],
        isDark: true
      },
      {
        id: 'red-samurai',
        name: 'Red Samurai',
        colors: ['#e53935', '#ff8a80', '#1a1517'],
        isDark: true
      },
      {
        id: 'watermelon',
        name: 'Watermelon Splash',
        colors: ['#ff3860', '#7bde83', '#1e2b1e'],
        isDark: true
      },
      {
        id: 'pastel-dream',
        name: 'Pastel Dream',
        colors: ['#ffafcc', '#ffc8dd', '#463854'],
        isDark: true
      },
      {
        id: 'minty-breeze',
        name: 'Minty Breeze',
        colors: ['#a0e8b7', '#cdf5de', '#2b4136'],
        isDark: true
      },
      {
        id: 'our-theme',
        name: 'Our Theme',
        colors: ['#e4000f', '#ffd700', '#8b0000'],
        isDark: true
      },
      {
        id: 'shreks-swamp',
        name: 'Shrek\'s Swamp',
        colors: ['#a2d149', '#f4d166', '#2f4f4f'],
        isDark: true
      },
      {
        id: 'demon-slayer',
        name: 'Demon Slayer',
        colors: ['#24c6dc', '#a0f6ff', '#202125'],
        isDark: true
      },
      {
        id: 'doge',
        name: 'Doge',
        colors: ['#f2a900', '#ffe27a', '#413728'],
        isDark: true
      },
      {
        id: 'spongebob',
        name: 'Bikini Bottom',
        colors: ['#ffec1a', '#8cdaff', '#0056a2'],
        isDark: false
      },
      {
        id: 'dualshot',
        name: 'Dualshot',
        colors: ['#5d6970', '#c9c9c9', '#1d2226'],
        isDark: true
      },
      {
        id: 'catppuccin',
        name: 'Catppuccin',
        colors: ['#f5c2e7', '#cdd6f4', '#1e1e2e'],
        isDark: true
      },
      {
        id: 'darling',
        name: 'Darling',
        colors: ['#ff6188', '#ffffff', '#2d1f2a'],
        isDark: true
      },
      {
        id: 'honey',
        name: 'Honey',
        colors: ['#e9b637', '#f9e4b7', '#3a3222'],
        isDark: true
      },
      {
        id: 'discord',
        name: 'Discord',
        colors: ['#5865f2', '#ffffff', '#36393f'],
        isDark: true
      },
      {
        id: 'blue-dolphin',
        name: 'Blue Dolphin',
        colors: ['#ff92c2', '#e3f5ff', '#0081cc'],
        isDark: true
      },
      {
        id: 'dark-magic',
        name: 'Dark Magic Girl',
        colors: ['#a9ebc1', '#bde7ff', '#0a3247'],
        isDark: true
      },
      {
        id: 'joker',
        name: 'Joker',
        colors: ['#8c4e9e', '#3ba55c', '#2a1a2f'],
        isDark: true
      },
      {
        id: 'vscode',
        name: 'VSCode Dark',
        colors: ['#007acc', '#d4d4d4', '#1e1e1e'],
        isDark: true
      },
    ].filter(theme => !['blueberry', 'fleuriste', 'creamsicle', 'alduin'].includes(theme.id));

    this.theme = localStorage.getItem('theme') || 'ocean';
    this.applyTheme(this.theme);

    const lastSearch = localStorage.getItem('lastSearchQuery');
    if (lastSearch) {
      this.searchInput.value = lastSearch;
      this.input = lastSearch;
      this.updateSearch(lastSearch);
    }

    this.setupEventListeners();
    this.setupSortEventListeners();
    this.setupKeyboardShortcuts();

    const copyTableBtn = document.getElementById('copy-table');
    if (copyTableBtn) copyTableBtn.addEventListener('click', () => this.copyTable());
    const toggleDetailsBtn = document.getElementById('toggle-details');
    if (toggleDetailsBtn) toggleDetailsBtn.addEventListener('click', () => this.toggleDetailsPanel());

    const favBtn = document.getElementById('toggle-favorites');
    favBtn.addEventListener('click', () => this.toggleFavorites());

    // Filter Modal Event Listeners
    const filterBtn = document.getElementById('filter-btn');
    if(filterBtn) {
      filterBtn.addEventListener('click', () => this.openFilterModal());
    }
    document.getElementById('filter-modal-close').addEventListener('click', () => this.closeFilterModal());
    document.getElementById('cancel-filters').addEventListener('click', () => this.closeFilterModal());
    document.getElementById('filter-modal').addEventListener('click', (e) => {
      if (e.target === document.getElementById('filter-modal')) {
        this.closeFilterModal();
      }
    });
    document.getElementById('clear-all-filters').addEventListener('click', () => this.clearAllFilters());
    
    // Settings Modal Event Listeners
    const settingsBtn = document.getElementById('settings-btn');
    if(settingsBtn) {
      settingsBtn.addEventListener('click', () => this.openSettingsModal());
    }
    document.getElementById('settings-modal-close').addEventListener('click', () => this.closeSettingsModal());
    document.getElementById('settings-modal').addEventListener('click', (e) => {
      if (e.target === document.getElementById('settings-modal')) {
        this.closeSettingsModal();
      }
    });

    this.setupThemeOptions();

    this.render();

    if (!lastSearch) {
      this.searchInput.focus();
    }
  }

  updateLocalFavorites() {
    const favNames = this.skins.filter(skin => skin.favorite).map(skin => skin.name);
    localStorage.setItem('favoriteSkins', JSON.stringify(favNames));
  }

  get displayResults() {
    return this.results.filter(skin => !this.favoritesOnly || skin.favorite);
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
    localStorage.setItem('lastSearchQuery', value);
    this.currentPage = 1;
    const terms = value.toLowerCase().split(' ').filter(t => t.trim() !== '');

    this.results = this.skins.filter(skin => {
      const searchString = `${skin.name} ${skin.rarity} ${skin.event} ${skin.year || ''} ${skin.tags.join(' ')}`.toLowerCase();
      let termMatch = terms.every(term => searchString.includes(term));
      if (!termMatch) return false;
      
      // Tag filters
      if (this.tagFilters.include.length > 0 && !skin.tags.some(tag => this.tagFilters.include.includes(tag))) {
        return false;
      }
      if (this.tagFilters.exclude.length > 0 && skin.tags.some(tag => this.tagFilters.exclude.includes(tag))) {
        return false;
      }
      
      // Case filters
      if (this.caseFilters.include.length > 0 && !this.caseFilters.include.includes(skin.event)) {
        return false;
      }
      if (this.caseFilters.exclude.length > 0 && this.caseFilters.exclude.includes(skin.event)) {
        return false;
      }
      
      // Rarity filters
      if (this.rarityFilters.include.length > 0 && !this.rarityFilters.include.includes(skin.rarity)) {
        return false;
      }
      if (this.rarityFilters.exclude.length > 0 && this.rarityFilters.exclude.includes(skin.rarity)) {
        return false;
      }
      
      // Year filters
      if (this.yearFilters.include.length > 0 && !this.yearFilters.include.includes(skin.year?.toString())) {
        return false;
      }
      if (this.yearFilters.exclude.length > 0 && this.yearFilters.exclude.includes(skin.year?.toString())) {
        return false;
      }
      
      // Event filters (non-case events)
      if (this.eventFilters.include.length > 0 && !this.eventFilters.include.includes(skin.event)) {
        return false;
      }
      if (this.eventFilters.exclude.length > 0 && this.eventFilters.exclude.includes(skin.event)) {
        return false;
      }
      
      return true;
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
      if (this.suggestionsCache[lastTerm]) {
        this.suggestions = this.suggestionsCache[lastTerm];
        return;
      }
      
      this.suggestions = fuzzysort.go(lastTerm, this.allTerms, {
        threshold: -10000,
        limit: 5,
        allowTypo: true,
        cached: true
      }).map(match => ({
        term: match.target,
        highlighted: fuzzysort.highlight(match, '<span class="suggestion-highlight">', '</span>')
      }));
      
      this.suggestionsCache[lastTerm] = this.suggestions;
    }
  }

  acceptSuggestion(term) {
    const terms = this.input.split(' ');
    terms.pop();
    terms.push(term);
    this.input = terms.join(' ') + ' ';

    this.searchInput.value = this.input;
    this.searchInput.focus();
    this.updateSearch(this.input);
  }

  clearSearch() {
    this.searchInput.value = '';
    this.updateSearch('');
    this.searchInput.focus();
  }

  setupEventListeners() {
    const debouncedUpdate = debounce((value) => this.updateSearch(value), 100);
    this.searchInput.addEventListener('input', (e) => debouncedUpdate(e.target.value));

    this.searchInput.addEventListener('keydown', (e) => {
      if (e.key === 'Tab' && this.suggestions.length > 0) {
        e.preventDefault();
        this.acceptSuggestion(this.suggestions[0].term);
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (this.suggestions.length > 0) {
          const items = this.suggestionsContainer.querySelectorAll('.suggestion-item');
          items[0]?.focus();
        } else {
          this.selectedIndex = Math.min(this.selectedIndex + 1, this.displayResults.length - 1);
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

    this.clearButton.addEventListener('click', () => {
      this.clearSearch();
    });

    document.addEventListener('keydown', (e) => {
      if (e.key === '/') {
        e.preventDefault();
        this.searchInput.focus();
      }
      if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
        e.preventDefault();
      }
    });

    document.querySelector('.search-container').addEventListener('click', (e) => {
      if (e.target.closest('.input-wrapper')) {
        this.searchInput.focus();
      }
    });

    document.addEventListener('click', (e) => {
      if (!e.target.closest('.input-wrapper')) {
        this.suggestionsContainer.style.display = 'none';
      }
    });
  }

  setupSortEventListeners() {
    const thElements = document.querySelectorAll('#results-table th[data-sort]');
    thElements.forEach(th => {
      th.addEventListener('click', () => {
        const key = th.dataset.sort;
        if (this.sortKey === key) {
          this.sortOrder = -this.sortOrder;
        } else {
          this.sortKey = key;
          this.sortOrder = 1;
        }
        this.renderTable();
        this.updateSortIndicators();
      });
    });
  }

  updateSortIndicators() {
    const thElements = document.querySelectorAll('#results-table th[data-sort]');
    thElements.forEach(th => {
      if (th.dataset.sort === this.sortKey) {
        th.classList.add('sorted');
        th.setAttribute('data-order', this.sortOrder === 1 ? '↑' : '↓');
      } else {
        th.classList.remove('sorted');
        th.removeAttribute('data-order');
      }
    });
  }

  sortResults() {
    if (!this.sortKey) return;
    this.results.sort((a, b) => {
      let valA = a[this.sortKey];
      let valB = b[this.sortKey];
      if (this.sortKey === 'tags') {
        valA = a.tags.join(', ');
        valB = b.tags.join(', ');
      }
      if (this.sortKey === 'year') {
        valA = a.year || 0;
        valB = b.year || 0;
      }
      if (typeof valA === 'string') valA = valA.toLowerCase();
      if (typeof valB === 'string') valB = valB.toLowerCase();
      if (valA < valB) return -1 * this.sortOrder;
      if (valA > valB) return 1 * this.sortOrder;
      return 0;
    });
  }

  renderStats() {
    const total = this.displayResults.length;
    const start = (this.currentPage - 1) * this.itemsPerPage;
    const end = Math.min(start + this.itemsPerPage, total);
    const totalPages = Math.max(1, Math.ceil(total / this.itemsPerPage));
    this.tableHeader.innerHTML = `
      <div class="stats-info">Showing ${total === 0 ? 0 : (start+1)}-${end} of ${total}${this.favoritesOnly ? " (Favorites)" : ""} | Selected: ${this.selectedIndex + 1}</div>
      <div class="top-pagination">${this.getPaginationHTML('top', totalPages)}</div>
    `;
    this.attachPaginationListeners('top');
  }

  getPaginationHTML(prefix, totalPages) {
    return `
      <button id="first-page-${prefix}" class="pagination-btn" ${this.currentPage === 1 ? 'disabled' : ''} title="First Page"><i class="ri-arrow-left-double-line"></i></button>
      <button id="prev-page-${prefix}" class="pagination-btn" ${this.currentPage === 1 ? 'disabled' : ''} title="Previous Page"><i class="ri-arrow-left-s-line"></i></button>
      <span>Page ${this.currentPage} of ${totalPages}</span>
      <button id="next-page-${prefix}" class="pagination-btn" ${this.currentPage === totalPages ? 'disabled' : ''} title="Next Page"><i class="ri-arrow-right-s-line"></i></button>
      <button id="last-page-${prefix}" class="pagination-btn" ${this.currentPage === totalPages ? 'disabled' : ''} title="Last Page"><i class="ri-arrow-right-double-line"></i></button>
    `;
  }

  attachPaginationListeners(prefix) {
    const firstBtn = document.getElementById(`first-page-${prefix}`);
    const prevBtn = document.getElementById(`prev-page-${prefix}`);
    const nextBtn = document.getElementById(`next-page-${prefix}`);
    const lastBtn = document.getElementById(`last-page-${prefix}`);
    if(firstBtn) {
      firstBtn.addEventListener('click', () => { this.currentPage = 1; this.render(); });
    }
    if(prevBtn) {
      prevBtn.addEventListener('click', () => { if (this.currentPage > 1) { this.currentPage--; this.render(); } });
    }
    if(nextBtn) {
      nextBtn.addEventListener('click', () => {
        const totalPages = Math.max(1, Math.ceil(this.displayResults.length / this.itemsPerPage));
        if (this.currentPage < totalPages) { this.currentPage++; this.render(); }
      });
    }
    if(lastBtn) {
      lastBtn.addEventListener('click', () => {
        this.currentPage = Math.max(1, Math.ceil(this.displayResults.length / this.itemsPerPage));
        this.render();
      });
    }
  }

  renderTable() {
    const total = this.displayResults.length;
    const start = (this.currentPage - 1) * this.itemsPerPage;
    const end = Math.min(start + this.itemsPerPage, total);
    if (total === 0) {
      this.tbody.innerHTML = `<tr><td colspan="6" style="text-align:center;">No skins found</td></tr>`;
      return;
    }

    if (this.sortKey) {
      this.sortResults();
    }

    const pageResults = this.displayResults.slice(start, end);

    const html = pageResults.map((skin, index) => {
      const globalIndex = start + index;
      let eventContent = skin.event;
      if(skin.event.toLowerCase().includes('case')) {
        eventContent = skin.event;
      }
      const tagsHtml = skin.tags.map(tag => `<span class="tag-label">${tag}</span>`).join(' ');

      return `
      <tr class="${globalIndex === this.selectedIndex ? 'selected' : ''}" title="Click to view details"
          onclick="app.selectedIndex = ${globalIndex}; app.render()">
        <td class="td-fav" onclick="event.stopPropagation(); app.toggleFavorite(${globalIndex})">
          ${skin.favorite ? '<i class="ri-star-fill"></i>' : '<i class="ri-star-line"></i>'}
        </td>
        <td>${skin.name}</td>
        <td class="rarity-${skin.rarity}">${skin.rarity}</td>
        <td>${eventContent}</td>
        <td>${skin.year || 'N/A'}</td>
        <td>${tagsHtml}</td>
      </tr>
    `;
    }).join('');

    this.tbody.innerHTML = html;

    if (this.selectedIndex < start || this.selectedIndex >= end) {
      this.selectedIndex = start;
    }

    if (this.selectedIndex >= 0) {
      requestAnimationFrame(() => {
        const selectedRelativeIndex = this.selectedIndex - start;
        const selectedRow = this.tbody.children[selectedRelativeIndex];
        if (selectedRow) {
          selectedRow.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
        }
      });
    }
  }

  renderSuggestion() {
    if (!this.input || this.suggestions.length === 0) {
      this.suggestionsContainer.style.display = 'none';
      return;
    }

    const html = this.suggestions.map(suggestion => `
      <div class="suggestion-item" data-term="${suggestion.term}">
        ${suggestion.highlighted || suggestion.term}
      </div>
    `).join('');

    this.suggestionsContainer.innerHTML = html;
    this.suggestionsContainer.style.display = 'block';
    
    this.suggestionsContainer.querySelectorAll('.suggestion-item').forEach(item => {
      item.addEventListener('click', () => {
        this.acceptSuggestion(item.dataset.term);
      });
    });
  }

  renderDetails() {
    const skin = this.displayResults[this.selectedIndex];
    
    if (!skin) {
      this.detailsEl.innerHTML = '<p>No skin selected</p>';
      return;
    }
    
    const imageContent = skin.img 
      ? `<img src="${skin.img}" alt="${skin.name} preview" class="detail-img">` 
      : `
      <div class="placeholder-image">
        <i class="ri-image-line"></i>
        <span>No preview available</span>
      </div>
    `;
    
    const tagsHtml = skin.tags.map(tag => `<span class="tag-label">${tag}</span>`).join(' ');
    
    const detailRows = `
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
        <span class="detail-label">Value:</span>
        <span>${skin.value || 'N/A'}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">Owners:</span>
        <span>${skin.owners || 'N/A'}</span>
      </div>
      <div class="detail-row">
        <span class="detail-label">Tags:</span>
        <span>${tagsHtml}</span>
      </div>
    `;
    
    if (window.innerWidth < 768) {
      this.detailsEl.innerHTML = `
        <div class="detail-mobile-container">
          <div class="detail-image">
            ${imageContent}
          </div>
          <div class="detail-info">
            <div class="detail-content">
              ${detailRows}
            </div>
            <button id="copy-details" class="qol-btn">
              <i class="ri-clipboard-line"></i> <span>Copy</span>
            </button>
          </div>
        </div>
      `;
    } else {
      this.detailsEl.innerHTML = `
        <div class="detail-image">
          ${imageContent}
        </div>
        <div class="detail-content">
          ${detailRows}
        </div>
        <button id="copy-details" class="qol-btn">
          <i class="ri-clipboard-line"></i> <span>Copy</span>
        </button>
      `;
    }
    
    const copyBtn = document.getElementById('copy-details');
    if(copyBtn) {
      copyBtn.addEventListener('click', () => {
        const detailContentEl = this.detailsEl.querySelector('.detail-content');
        if (!detailContentEl) return;
        const rows = detailContentEl.querySelectorAll('.detail-row');
        let copyText = '';
        rows.forEach(row => {
          const label = row.children[0].textContent.replace(':', '').trim();
          const value = row.children[1].textContent.trim();
          if (value === 'N/A' || value.includes('No preview available')) return;
          copyText += `${label}: ${value}\n`;
        });
        navigator.clipboard.writeText(copyText)
          .then(() => {
            copyBtn.innerHTML = '<i class="ri-clipboard-line"></i> <span>Copied!</span>';
            setTimeout(() => copyBtn.innerHTML = '<i class="ri-clipboard-line"></i> <span>Copy</span>', 2000);
          })
          .catch(err => console.error('Copy failed', err));
      });
    }
    
    if(window.innerWidth < 768) {
      this.detailsEl.scrollIntoView({ behavior: 'smooth' });
    }
  }

  renderPagination() {
    const paginationContainer = document.getElementById('pagination-controls');
    paginationContainer.innerHTML = '';
  }

  renderFilters() {
    const container = document.getElementById('active-filters');
    container.innerHTML = '';
    const createBadge = (type, value, filterType) => {
      const badge = document.createElement('div');
      badge.className = 'active-filter';
      badge.innerHTML = `${type === 'include' ? 'Include: ' : 'Exclude: '}${value} <i class="ri-close-line" data-type="${type}" data-value="${value}" data-filter-type="${filterType}"></i>`;
      badge.dataset.type = type;
      badge.dataset.value = value;
      badge.dataset.filterType = filterType;
      
      const removeFilter = () => {
        const val = badge.dataset.value;
        const filterType = badge.dataset.filterType;
        const type = badge.dataset.type;
        
        if (filterType === 'tag') {
          if (type === 'include') {
            this.tagFilters.include = this.tagFilters.include.filter(item => item !== val);
          } else {
            this.tagFilters.exclude = this.tagFilters.exclude.filter(item => item !== val);
          }
        } else if (filterType === 'case') {
          if (type === 'include') {
            this.caseFilters.include = this.caseFilters.include.filter(item => item !== val);
          } else {
            this.caseFilters.exclude = this.caseFilters.exclude.filter(item => item !== val);
          }
        } else if (filterType === 'event') {
          if (type === 'include') {
            this.eventFilters.include = this.eventFilters.include.filter(item => item !== val);
          } else {
            this.eventFilters.exclude = this.eventFilters.exclude.filter(item => item !== val);
          }
        } else if (filterType === 'rarity') {
          if (type === 'include') {
            this.rarityFilters.include = this.rarityFilters.include.filter(item => item !== val);
          } else {
            this.rarityFilters.exclude = this.rarityFilters.exclude.filter(item => item !== val);
          }
        } else if (filterType === 'year') {
          if (type === 'include') {
            this.yearFilters.include = this.yearFilters.include.filter(item => item !== val);
          } else {
            this.yearFilters.exclude = this.yearFilters.exclude.filter(item => item !== val);
          }
        }
        
        // Apply filters immediately
        this.updateSearch(this.input);
      };
      
      // Make both the badge and the X icon clickable
      badge.addEventListener('click', removeFilter);
      const closeIcon = badge.querySelector('i');
      closeIcon.addEventListener('click', (e) => {
        e.stopPropagation(); // Prevent double triggering
        removeFilter();
      });
      
      return badge;
    }
    
    // Add tag filters
    this.tagFilters.include.forEach(tag => {
      container.appendChild(createBadge('include', tag.charAt(0).toUpperCase() + tag.slice(1), 'tag'));
    });
    this.tagFilters.exclude.forEach(tag => {
      container.appendChild(createBadge('exclude', tag.charAt(0).toUpperCase() + tag.slice(1), 'tag'));
    });
    
    // Add case filters
    this.caseFilters.include.forEach(caseName => {
      container.appendChild(createBadge('include', caseName, 'case'));
    });
    this.caseFilters.exclude.forEach(caseName => {
      container.appendChild(createBadge('exclude', caseName, 'case'));
    });
    
    // Add event filters
    this.eventFilters.include.forEach(eventName => {
      container.appendChild(createBadge('include', eventName, 'event'));
    });
    this.eventFilters.exclude.forEach(eventName => {
      container.appendChild(createBadge('exclude', eventName, 'event'));
    });
    
    // Add rarity filters
    this.rarityFilters.include.forEach(rarity => {
      container.appendChild(createBadge('include', rarity, 'rarity'));
    });
    this.rarityFilters.exclude.forEach(rarity => {
      container.appendChild(createBadge('exclude', rarity, 'rarity'));
    });
    
    // Add year filters
    this.yearFilters.include.forEach(year => {
      container.appendChild(createBadge('include', year, 'year'));
    });
    this.yearFilters.exclude.forEach(year => {
      container.appendChild(createBadge('exclude', year, 'year'));
    });
  }

  render() {
    this.renderTable();
    this.renderSuggestion();
    if (this.detailsVisible) {
      this.renderDetails();
    } else {
      this.detailsEl.innerHTML = '';
    }
    this.renderStats();
    this.renderPagination();
    this.renderFilters();
  }

  toggleFavorite(index) {
    const displayed = this.displayResults;
    const skin = displayed[index];
    if (skin) {
      skin.favorite = !skin.favorite;
      this.updateLocalFavorites();
    }
    this.render();
  }
  
  toggleFavorites() {
    this.favoritesOnly = !this.favoritesOnly;
    this.selectedIndex = 0;
    const favBtn = document.getElementById('toggle-favorites');
    if (this.favoritesOnly) {
      favBtn.classList.add('active');
    } else {
      favBtn.classList.remove('active');
    }
    this.render();
  }

  randomSkin() {
    if (this.displayResults.length === 0) return;
    const randomIndex = Math.floor(Math.random() * this.displayResults.length);
    this.selectedIndex = randomIndex;
    this.render();
    if (this.detailsVisible && window.innerWidth < 768) {
      this.detailsEl.scrollIntoView({ behavior: 'smooth' });
    }
  }

  copyTable() {
    if (this.displayResults.length === 0) return;
    const rowsText = this.displayResults.map(skin => {
      let row = [];
      row.push(skin.name);
      row.push(skin.rarity);
      row.push(skin.event);
      if (skin.year) row.push(skin.year.toString());
      if (skin.tags && skin.tags.length > 0) row.push(skin.tags.join(', '));
      return row.join(' | ');
    }).join('\n');
    navigator.clipboard.writeText(rowsText)
      .then(() => {
         const copyTableBtn = document.getElementById('copy-table');
         if (copyTableBtn) {
           const originalHTML = copyTableBtn.innerHTML;
           copyTableBtn.innerHTML = '<i class="ri-clipboard-line"></i> <span>Copied!</span>';
           setTimeout(() => copyTableBtn.innerHTML = originalHTML, 2000);
         }
      })
      .catch(err => console.error('Copy failed', err));
  }

  toggleDetailsPanel() {
    this.detailsVisible = !this.detailsVisible;
    const toggleDetailsBtn = document.getElementById('toggle-details');
    
    if (this.detailsVisible) {
      this.detailsEl.style.display = 'block';
      this.renderDetails();
      if (toggleDetailsBtn) {
        toggleDetailsBtn.innerHTML = '<i class="ri-eye-line"></i> <span>Details</span>';
      }
    } else {
      this.detailsEl.style.display = 'none';
      if (toggleDetailsBtn) {
        toggleDetailsBtn.innerHTML = '<i class="ri-eye-off-line"></i> <span>Details</span>';
      }
    }
  }

  openFilterModal() {
    const modal = document.getElementById('filter-modal');
    modal.style.display = 'flex';
    
    // Set up the filter tags UI
    const tagContainer = document.getElementById('filter-tags-container');
    tagContainer.innerHTML = '';
    
    // Get unique tags and capitalize first letter
    const allTags = new Set();
    this.skins.forEach(skin => skin.tags.forEach(tag => {
      // Capitalize first letter
      const capitalizedTag = tag.charAt(0).toUpperCase() + tag.slice(1);
      allTags.add(capitalizedTag);
    }));
    
    // Create clickable tag elements
    allTags.forEach(tag => {
      const div = document.createElement('div');
      div.className = 'filter-tag neutral';
      div.innerText = tag;
      div.dataset.tag = tag.toLowerCase(); // Store original lowercase tag for filtering
      div.dataset.state = 'neutral';
      
      div.addEventListener('click', () => {
        if (div.dataset.state === 'neutral') {
          div.dataset.state = 'include';
          div.className = 'filter-tag include';
        } else if (div.dataset.state === 'include') {
          div.dataset.state = 'exclude';
          div.className = 'filter-tag exclude';
        } else {
          div.dataset.state = 'neutral';
          div.className = 'filter-tag neutral';
        }
        
        // Apply filters immediately
        this.applyFilters();
      });
      
      // Pre-select tags based on current filters
      if (this.tagFilters.include.includes(tag.toLowerCase())) {
        div.dataset.state = 'include';
        div.className = 'filter-tag include';
      } else if (this.tagFilters.exclude.includes(tag.toLowerCase())) {
        div.dataset.state = 'exclude';
        div.className = 'filter-tag exclude';
      }
      
      tagContainer.appendChild(div);
    });
    
    // Set up rarity filters UI
    const rarityContainer = document.getElementById('filter-rarities-container');
    rarityContainer.innerHTML = '';
    
    // Get unique rarities
    const allRarities = new Set();
    this.skins.forEach(skin => allRarities.add(skin.rarity));
    
    // Create clickable rarity elements
    allRarities.forEach(rarity => {
      const div = document.createElement('div');
      div.className = 'filter-tag neutral';
      div.innerText = rarity;
      div.dataset.rarity = rarity;
      div.dataset.state = 'neutral';
      
      div.addEventListener('click', () => {
        if (div.dataset.state === 'neutral') {
          div.dataset.state = 'include';
          div.className = 'filter-tag include';
        } else if (div.dataset.state === 'include') {
          div.dataset.state = 'exclude';
          div.className = 'filter-tag exclude';
        } else {
          div.dataset.state = 'neutral';
          div.className = 'filter-tag neutral';
        }
        
        // Apply filters immediately
        this.applyFilters();
      });
      
      // Pre-select rarities based on current filters
      if (this.rarityFilters.include.includes(rarity)) {
        div.dataset.state = 'include';
        div.className = 'filter-tag include';
      } else if (this.rarityFilters.exclude.includes(rarity)) {
        div.dataset.state = 'exclude';
        div.className = 'filter-tag exclude';
      }
      
      rarityContainer.appendChild(div);
    });
    
    // Set up years filters UI
    const yearContainer = document.getElementById('filter-years-container');
    yearContainer.innerHTML = '';
    
    // Get unique years
    const allYears = new Set();
    this.skins.forEach(skin => {
      if (skin.year) allYears.add(skin.year.toString());
    });
    
    // Create clickable year elements
    allYears.forEach(year => {
      const div = document.createElement('div');
      div.className = 'filter-tag neutral';
      div.innerText = year;
      div.dataset.year = year;
      div.dataset.state = 'neutral';
      
      div.addEventListener('click', () => {
        if (div.dataset.state === 'neutral') {
          div.dataset.state = 'include';
          div.className = 'filter-tag include';
        } else if (div.dataset.state === 'include') {
          div.dataset.state = 'exclude';
          div.className = 'filter-tag exclude';
        } else {
          div.dataset.state = 'neutral';
          div.className = 'filter-tag neutral';
        }
        
        // Apply filters immediately
        this.applyFilters();
      });
      
      // Pre-select years based on current filters
      if (this.yearFilters.include.includes(year)) {
        div.dataset.state = 'include';
        div.className = 'filter-tag include';
      } else if (this.yearFilters.exclude.includes(year)) {
        div.dataset.state = 'exclude';
        div.className = 'filter-tag exclude';
      }
      
      yearContainer.appendChild(div);
    });
    
    // Split events from cases
    // Set up case filters UI
    const caseContainer = document.getElementById('filter-cases-container');
    caseContainer.innerHTML = '';
    
    // Set up event filters UI (non-case events)
    const eventContainer = document.getElementById('filter-events-container');
    eventContainer.innerHTML = '';
    
    // Get unique events
    const allEvents = new Set();
    this.skins.forEach(skin => allEvents.add(skin.event));
    
    // Create clickable case/event elements
    allEvents.forEach(eventName => {
      const div = document.createElement('div');
      div.className = 'filter-tag neutral';
      div.innerText = eventName;
      div.dataset.event = eventName;
      div.dataset.state = 'neutral';
      
      div.addEventListener('click', () => {
        if (div.dataset.state === 'neutral') {
          div.dataset.state = 'include';
          div.className = 'filter-tag include';
        } else if (div.dataset.state === 'include') {
          div.dataset.state = 'exclude';
          div.className = 'filter-tag exclude';
        } else {
          div.dataset.state = 'neutral';
          div.className = 'filter-tag neutral';
        }
        
        // Apply filters immediately
        this.applyFilters();
      });
      
      // Decide if it's a case or other event
      const isCase = eventName.toLowerCase().includes('case');
      const container = isCase ? caseContainer : eventContainer;
      
      // Pre-select based on current filters
      if (isCase) {
        if (this.caseFilters.include.includes(eventName)) {
          div.dataset.state = 'include';
          div.className = 'filter-tag include';
        } else if (this.caseFilters.exclude.includes(eventName)) {
          div.dataset.state = 'exclude';
          div.className = 'filter-tag exclude';
        }
      } else {
        if (this.eventFilters.include.includes(eventName)) {
          div.dataset.state = 'include';
          div.className = 'filter-tag include';
        } else if (this.eventFilters.exclude.includes(eventName)) {
          div.dataset.state = 'exclude';
          div.className = 'filter-tag exclude';
        }
      }
      
      container.appendChild(div);
    });
    
    // Show the modal with animation
    setTimeout(() => modal.classList.add('visible'), 10);
  }
  
  applyFilters() {
    const tagDivs = document.querySelectorAll('#filter-tags-container .filter-tag');
    const includeTags = [];
    const excludeTags = [];
    tagDivs.forEach(div => {
      const state = div.dataset.state;
      const tag = div.dataset.tag;
      if(state === 'include') includeTags.push(tag);
      if(state === 'exclude') excludeTags.push(tag);
    });
    this.tagFilters = { include: includeTags, exclude: excludeTags };

    const caseDivs = document.querySelectorAll('#filter-cases-container .filter-tag');
    const includeCases = [];
    const excludeCases = [];
    caseDivs.forEach(div => {
      const state = div.dataset.state;
      const eventName = div.dataset.event;
      if (state === 'include') includeCases.push(eventName);
      if (state === 'exclude') excludeCases.push(eventName);
    });
    this.caseFilters = { include: includeCases, exclude: excludeCases };

    const eventDivs = document.querySelectorAll('#filter-events-container .filter-tag');
    const includeEvents = [];
    const excludeEvents = [];
    eventDivs.forEach(div => {
      const state = div.dataset.state;
      const eventName = div.dataset.event;
      if (state === 'include') includeEvents.push(eventName);
      if (state === 'exclude') excludeEvents.push(eventName);
    });
    this.eventFilters = { include: includeEvents, exclude: excludeEvents };

    const rarityDivs = document.querySelectorAll('#filter-rarities-container .filter-tag');
    const includeRarities = [];
    const excludeRarities = [];
    rarityDivs.forEach(div => {
      const state = div.dataset.state;
      const rarity = div.dataset.rarity;
      if (state === 'include') includeRarities.push(rarity);
      if (state === 'exclude') excludeRarities.push(rarity);
    });
    this.rarityFilters = { include: includeRarities, exclude: excludeRarities };

    const yearDivs = document.querySelectorAll('#filter-years-container .filter-tag');
    const includeYears = [];
    const excludeYears = [];
    yearDivs.forEach(div => {
      const state = div.dataset.state;
      const year = div.dataset.year;
      if (state === 'include') includeYears.push(year);
      if (state === 'exclude') excludeYears.push(year);
    });
    this.yearFilters = { include: includeYears, exclude: excludeYears };

    this.updateSearch(this.input);
  }
  
  clearAllFilters() {
    const allFilterTags = document.querySelectorAll('.filter-tag');
    allFilterTags.forEach(div => {
      div.dataset.state = 'neutral';
      div.className = 'filter-tag neutral';
    });
    
    this.tagFilters = { include: [], exclude: [] };
    this.caseFilters = { include: [], exclude: [] };
    this.eventFilters = { include: [], exclude: [] };
    this.rarityFilters = { include: [], exclude: [] };
    this.yearFilters = { include: [], exclude: [] };
    
    this.updateSearch(this.input);
  }

  closeFilterModal() {
    const modal = document.getElementById('filter-modal');
    modal.classList.remove('visible');
    setTimeout(() => modal.style.display = 'none', 300);
  }

  createThemeToggle() {
    // Remove the theme toggle button entirely
  }
  
  toggleTheme() {
    this.openSettingsModal();
  }

  applyTheme(themeId) {
    const theme = this.themes.find(t => t.id === themeId) || this.themes[0];
    document.documentElement.setAttribute('data-theme', themeId);
    
    document.documentElement.style.setProperty('--accent-color', theme.colors[0]);
    document.documentElement.style.setProperty('--text-color', theme.colors[1]);
    document.documentElement.style.setProperty('--bg-color', theme.colors[2]);
    document.documentElement.style.setProperty('--placeholder-color', theme.colors[1] + 'a0');
    
    document.documentElement.style.setProperty('--card-bg', 
      this.adjustColorBrightness(theme.colors[2], 10));
    document.documentElement.style.setProperty('--hover-bg', 
      this.adjustColorBrightness(theme.colors[2], 20));
    document.documentElement.style.setProperty('--header-bg', 
      this.adjustColorBrightness(theme.colors[2], 15));
    document.documentElement.style.setProperty('--border-color', 
      this.adjustColorBrightness(theme.colors[2], 30));
    document.documentElement.style.setProperty('--button-border', 
      this.adjustColorBrightness(theme.colors[2], 40) + '30');
    
    document.documentElement.style.setProperty('--rarity-pink', 
      this.adjustColorBrightness(theme.colors[0], 20));
    document.documentElement.style.setProperty('--rarity-teal', 
      this.adjustColorBrightness(theme.colors[0], 0));
    document.documentElement.style.setProperty('--rarity-red', 
      this.adjustColorBrightness(theme.colors[0], -20));
    document.documentElement.style.setProperty('--rarity-legendary', 
      this.adjustColorBrightness(theme.colors[0], 40));
    document.documentElement.style.setProperty('--rarity-epic', 
      this.adjustColorBrightness(theme.colors[0], -40));
    
    const themeToggle = document.querySelector('.theme-toggle');
    if (themeToggle) {
      themeToggle.innerHTML = '<i class="ri-settings-3-line"></i>';
      themeToggle.title = 'Open Settings (Alt+S)';
    }
    
    localStorage.setItem('theme', themeId);
    this.theme = themeId;
  }

  setupThemeOptions() {
    const themeContainer = document.getElementById('theme-options');
    if (!themeContainer) return;
    
    themeContainer.innerHTML = '';
    this.themes.forEach(theme => {
      const themeOption = document.createElement('div');
      themeOption.className = 'theme-option';
      themeOption.dataset.themeId = theme.id;
      
      const colorPreviews = theme.colors.map(color => 
        `<span class="color-circle" style="background-color: ${color}"></span>`
      ).join('');
      
      themeOption.innerHTML = `
        <div class="theme-colors">
          ${colorPreviews}
        </div>
        <div class="theme-name" style="color: ${theme.colors[0]}">${theme.name}</div>
      `;
      
      themeOption.addEventListener('click', () => {
        this.applyTheme(theme.id);
        
        document.querySelectorAll('.theme-option').forEach(opt => {
          opt.classList.remove('active');
        });
        themeOption.classList.add('active');
      });
      
      themeContainer.appendChild(themeOption);
    });
  }

  openSettingsModal() {
    const modal = document.getElementById('settings-modal');
    modal.style.display = 'flex';
    setTimeout(() => modal.classList.add('visible'), 10);
  }
  
  closeSettingsModal() {
    const modal = document.getElementById('settings-modal');
    modal.classList.remove('visible');
    setTimeout(() => modal.style.display = 'none', 300);
  }

  adjustColorBrightness(hex, percent) {
    let r = parseInt(hex.substring(1, 3), 16);
    let g = parseInt(hex.substring(3, 5), 16);
    let b = parseInt(hex.substring(5, 7), 16);
    
    r = Math.max(0, Math.min(255, r + percent));
    g = Math.max(0, Math.min(255, g + percent));
    b = Math.max(0, Math.min(255, b + percent));
    
    return `#${((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1)}`;
  }

  setupKeyboardShortcuts() {
    document.addEventListener('keydown', (e) => {
      if (e.altKey && e.key.toLowerCase() === 'f') {
        e.preventDefault();
        this.toggleFavorites();
      }
      if (e.altKey && e.key.toLowerCase() === 'x') {
        e.preventDefault();
        this.clearSearch();
      }
      if (e.altKey && e.key.toLowerCase() === 's') {
        e.preventDefault();
        this.openSettingsModal();
      }
      if (e.altKey && e.key.toLowerCase() === 'z') {
        e.preventDefault();
        document.getElementById('filter-btn').click();
      }
    });
  }
}

document.addEventListener('DOMContentLoaded', () => {
  const app = new AppState();
  window.app = app;

  document.getElementById('help-btn').addEventListener('click', () => {
    const modal = document.getElementById('help-modal');
    modal.style.display = 'flex';
    setTimeout(() => modal.classList.add('visible'), 10);
  });

  document.getElementById('help-modal-close').addEventListener('click', () => {
    const modal = document.getElementById('help-modal');
    modal.classList.remove('visible');
    setTimeout(() => modal.style.display = 'none', 300);
  });

  document.getElementById('help-modal').addEventListener('click', (e) => {
    if (e.target === document.getElementById('help-modal')) {
      const modal = document.getElementById('help-modal');
      modal.classList.remove('visible');
      setTimeout(() => modal.style.display = 'none', 300);
    }
  });

  document.addEventListener('keydown', (e) => {
    if (e.altKey && e.key.toLowerCase() === 'c') {
      e.preventDefault();
      app.copyTable();
    }
    if (e.altKey && e.key.toLowerCase() === 't') {
      e.preventDefault();
      app.toggleDetailsPanel();
    }
    if (e.altKey && e.key.toLowerCase() === 'h') {
      e.preventDefault();
      document.getElementById('help-btn').click();
    }
  });
});
