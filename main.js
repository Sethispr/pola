function debounce(func, delay) {
  let timer;
  return function(...args) {
    clearTimeout(timer);
    timer = setTimeout(() => func.apply(this, args), delay);
  }
}

const SKIN_COLLECTION = [
  {
    name: "Azurite",
    rarity: "Pink",
    event: "Easter Event",
    year: 2022,
    tags: ["event"],
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
    name: "Bubbles",
    rarity: "Teal",
    event: "Code Redeemed",
    year: null,
    tags: ["code"],
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
    name: "Crimson Periastron",
    rarity: "Red",
    event: "Valentine Case",
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
    name: "Ghostly",
    rarity: "Pink",
    event: "Birthday Case",
    year: null,
    tags: ["case"],
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
    name: "Cupid",
    rarity: "Pink",
    event: "Valentine Case",
    year: null,
    tags: ["case"],
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
    name: "Midsummer",
    rarity: "Pink",
    event: "Summer Case",
    year: null,
    tags: ["case"],
    img: null
  },
  {
    name: "Rainbow Periastron",
    rarity: "Pink",
    event: "Valentine Case (Exquisite)",
    year: null,
    tags: ["case", "exquisite"],
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
    name: "All Hallow's",
    rarity: "Pink",
    event: "Halloween Case",
    year: null,
    tags: ["case"],
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

    // Tag and Case Filters
    this.tagFilters = { include: [], exclude: [] };
    this.caseFilters = { include: [], exclude: [] };

    // Pagination state
    this.currentPage = 1;
    this.itemsPerPage = 10;

    const lastSearch = localStorage.getItem('lastSearchQuery');
    if (lastSearch) {
      this.searchInput.value = lastSearch;
      this.input = lastSearch;
      this.updateSearch(lastSearch);
    }

    this.setupEventListeners();
    this.setupSortEventListeners();

    const randomBtn = document.getElementById('random-skin');
    if (randomBtn) randomBtn.addEventListener('click', () => this.randomSkin());
    const copyTableBtn = document.getElementById('copy-table');
    if (copyTableBtn) copyTableBtn.addEventListener('click', () => this.copyTable());
    const toggleDetailsBtn = document.getElementById('toggle-details');
    if (toggleDetailsBtn) toggleDetailsBtn.addEventListener('click', () => this.toggleDetailsPanel());

    const favBtn = document.getElementById('toggle-favorites');
    favBtn.addEventListener('click', () => this.toggleFavorites());

    const resetBtn = document.getElementById('reset-favorites');
    // REMOVE RESET BUTTON FUNCTIONALITY
    // resetBtn.addEventListener('click', () => {
    //   if (confirm("Are you sure you want to reset all favorites?")) {
    //     this.skins.forEach(skin => skin.favorite = false);
    //     this.updateLocalFavorites();
    //     this.render();
    //   }
    // });

    // Filter Modal Event Listeners
    const filterBtn = document.getElementById('filter-btn');
    if(filterBtn) {
      filterBtn.addEventListener('click', () => this.openFilterModal());
    }
    document.getElementById('filter-modal-close').addEventListener('click', () => this.closeFilterModal());
    document.getElementById('cancel-filters').addEventListener('click', () => this.closeFilterModal());
    document.getElementById('apply-filters').addEventListener('click', () => {
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
        const caseName = div.dataset.case;
        if (state === 'include') includeCases.push(caseName);
        if (state === 'exclude') excludeCases.push(caseName);
      });
      this.caseFilters = { include: includeCases, exclude: excludeCases };

      this.updateSearch(this.input);
      this.closeFilterModal();
    });

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
      if (this.tagFilters.include.length > 0 && !skin.tags.some(tag => this.tagFilters.include.includes(tag))) {
        return false;
      }
      if (this.tagFilters.exclude.length > 0 && skin.tags.some(tag => this.tagFilters.exclude.includes(tag))) {
        return false;
      }
      if (this.caseFilters.include.length > 0 && !this.caseFilters.include.includes(skin.event)) {
        return false;
      }
      if (this.caseFilters.exclude.length > 0 && this.caseFilters.exclude.includes(skin.event)) {
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
    const debouncedUpdate = debounce((value) => this.updateSearch(value), 250);
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

    document.querySelector('.search-container').addEventListener('click', () => {
      this.searchInput.focus();
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
        <span>${skin.tags.join(', ')}</span>
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
    const createBadge = (type, tag, isCase = false) => {
      const badge = document.createElement('div');
      badge.className = 'active-filter';
      badge.innerHTML = `${type === 'include' ? 'Include: ' : 'Exclude: '}${tag} <i class="ri-close-line" data-type="${type}" data-tag="${tag}" ${isCase ? 'data-case="true"' : ''}></i>`;
      badge.querySelector('i').addEventListener('click', (e) => {
        const t = e.target.dataset.tag;
        const filterType = e.target.dataset.type;
        if(filterType === 'include') {
          if(e.target.hasAttribute('data-case')) {
            this.caseFilters.include = this.caseFilters.include.filter(item => item !== t);
          } else {
            this.tagFilters.include = this.tagFilters.include.filter(item => item !== t);
          }
        } else {
          if(e.target.hasAttribute('data-case')) {
            this.caseFilters.exclude = this.caseFilters.exclude.filter(item => item !== t);
          } else {
            this.tagFilters.exclude = this.tagFilters.exclude.filter(item => item !== t);
          }
        }
        this.updateSearch(this.input);
      });
      return badge;
    }
    this.tagFilters.include.forEach(tag => {
      container.appendChild(createBadge('include', tag));
    });
    this.tagFilters.exclude.forEach(tag => {
      container.appendChild(createBadge('exclude', tag));
    });
    this.caseFilters.include.forEach(caseName => {
      container.appendChild(createBadge('include', caseName, true));
    });
    this.caseFilters.exclude.forEach(caseName => {
      container.appendChild(createBadge('exclude', caseName, true));
    });
  }

  openFilterModal() {
    const modal = document.getElementById('filter-modal');
    modal.style.display = 'flex';
    this.populateFilterModal();
  }

  closeFilterModal() {
    const modal = document.getElementById('filter-modal');
    modal.style.display = 'none';
  }

  populateFilterModal() {
    const tagsSet = new Set();
    this.skins.forEach(skin => {
      skin.tags.forEach(tag => tagsSet.add(tag));
    });
    const tags = Array.from(tagsSet).sort();
    const container = document.getElementById('filter-tags-container');
    container.innerHTML = '';
    tags.forEach(tag => {
      let state = 'neutral';
      if (this.tagFilters.include.includes(tag)) state = 'include';
      else if (this.tagFilters.exclude.includes(tag)) state = 'exclude';
      const tagDiv = document.createElement('div');
      tagDiv.className = `filter-tag ${state}`;
      tagDiv.dataset.tag = tag;
      tagDiv.dataset.state = state;
      tagDiv.textContent = tag;
      tagDiv.addEventListener('click', () => {
        let currentState = tagDiv.dataset.state;
        if (currentState === 'neutral') {
          tagDiv.dataset.state = 'include';
          tagDiv.classList.remove('neutral');
          tagDiv.classList.add('include');
        } else if (currentState === 'include') {
          tagDiv.dataset.state = 'exclude';
          tagDiv.classList.remove('include');
          tagDiv.classList.add('exclude');
        } else {
          tagDiv.dataset.state = 'neutral';
          tagDiv.classList.remove('exclude');
          tagDiv.classList.add('neutral');
        }
      });
      container.appendChild(tagDiv);
    });

    const casesSet = new Set();
    this.skins.forEach(skin => {
      if (skin.event.toLowerCase().includes('case')) {
        casesSet.add(skin.event);
      }
    });
    const cases = Array.from(casesSet).sort();
    const casesContainer = document.getElementById('filter-cases-container');
    casesContainer.innerHTML = '';
    cases.forEach(caseName => {
      let state = 'neutral';
      if (this.caseFilters.include.includes(caseName)) state = 'include';
      else if (this.caseFilters.exclude.includes(caseName)) state = 'exclude';
      const caseDiv = document.createElement('div');
      caseDiv.className = `filter-tag ${state}`;
      caseDiv.dataset.case = caseName;
      caseDiv.dataset.state = state;
      caseDiv.textContent = caseName;
      caseDiv.addEventListener('click', () => {
        let currentState = caseDiv.dataset.state;
        if (currentState === 'neutral') {
          caseDiv.dataset.state = 'include';
          caseDiv.classList.remove('neutral');
          caseDiv.classList.add('include');
        } else if (currentState === 'include') {
          caseDiv.dataset.state = 'exclude';
          caseDiv.classList.remove('include');
          caseDiv.classList.add('exclude');
        } else {
          caseDiv.dataset.state = 'neutral';
          caseDiv.classList.remove('exclude');
          caseDiv.classList.add('neutral');
        }
      });
      casesContainer.appendChild(caseDiv);
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
      .catch(err => console.error('Failed to copy table', err));
  }

  toggleDetailsPanel() {
    this.detailsVisible = !this.detailsVisible;
    if (this.detailsVisible) {
      this.detailsEl.style.display = 'block';
      this.renderDetails();
    } else {
      this.detailsEl.style.display = 'none';
    }
  }
}

const app = new AppState();
window.app = app;

document.addEventListener('scroll', () => {
  const backToTopButton = document.getElementById('back-to-top');
  if (backToTopButton) { 
    if (window.scrollY > 200) {
      backToTopButton.classList.add('show');
    } else {
      backToTopButton.classList.remove('show');
    }
  }
});

const backToTopButtonEl = document.getElementById('back-to-top'); 
if (backToTopButtonEl) { 
  backToTopButtonEl.addEventListener('click', () => {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  });
}

document.getElementById('help-btn').addEventListener('click', () => {
  document.getElementById('help-modal').style.display = 'flex';
});
document.getElementById('help-modal-close').addEventListener('click', () => {
  document.getElementById('help-modal').style.display = 'none';
});
document.getElementById('help-modal').addEventListener('click', (e) => {
  if (e.target === document.getElementById('help-modal')) {
    document.getElementById('help-modal').style.display = 'none';
  }
});

document.addEventListener('keydown', (e) => {
  if (e.altKey && e.key.toLowerCase() === 'r') {
    e.preventDefault();
    app.randomSkin();
  }
  if (e.altKey && e.key.toLowerCase() === 'c') {
    e.preventDefault();
    app.copyTable();
  }
  if (e.altKey && e.key.toLowerCase() === 't') {
    e.preventDefault();
    app.toggleDetailsPanel();
  }
});
