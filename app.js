class SkinDatabase {
    constructor() {
        this.skins = window.skinsData;
        this.allTerms = this.loadAllTerms();
        this.selectedIndex = 0;
        this.input = "";
        this.results = [];
        this.suggestion = null;
        this.init();
    }

    init() {
        this.searchInput = document.getElementById('search');
        this.resultsList = document.getElementById('results');
        this.detailsPanel = document.getElementById('details');
        this.suggestionsDiv = document.getElementById('suggestions');

        this.searchInput.addEventListener('input', () => this.handleSearch());
        document.addEventListener('keydown', (e) => this.handleKeys(e));
        this.resultsList.addEventListener('click', (e) => this.handleRowClick(e));

        this.updateResults();
    }

    handleSearch() {
        this.input = this.searchInput.value;
        this.updateResults();
        this.updateSuggestions();
    }

    updateResults() {
        const query = this.input.toLowerCase();
        const tags = new Set(query.split(/\s+/).filter(t => t);

        this.results = this.searchSkins(tags);
        this.selectedIndex = Math.min(this.selectedIndex, this.results.length - 1);
        this.renderResults();
        this.updateDetails();
    }

    searchSkins(tags) {
        if (tags.size === 0) return this.skins;

        // First, try to find exact name matches
        const exactMatches = this.skins.filter(skin =>
            Array.from(tags).some(tag => skin.name.toLowerCase() === tag)
        );

        if (exactMatches.length > 0) return exactMatches;

        // If no exact matches, enforce AND logic for all tags
        const scoredSkins = this.skins
            .filter(skin =>
                Array.from(tags).every(tag =>
                    skin.name.toLowerCase().includes(tag) ||
                    skin.rarity.toLowerCase() === tag ||
                    skin.event.toLowerCase().includes(tag) ||
                    skin.tags.some(t => t.toLowerCase() === tag) ||
                    (skin.year && skin.year.toString() === tag)
                )
            )
            .map(skin => {
                let score = 0;
                for (const tag of tags) {
                    if (skin.name.toLowerCase().includes(tag)) score += 100;
                    if (skin.rarity.toLowerCase() === tag) score += 80;
                    if (skin.event.toLowerCase().includes(tag)) score += 60;
                    if (skin.tags.some(t => t.toLowerCase() === tag)) score += 40;
                    if (skin.year && skin.year.toString() === tag) score += 20;

                    // Add fuzzy matching score for name
                    const fuzzyScore = this.fuzzyMatch(skin.name.toLowerCase(), tag);
                    if (fuzzyScore !== null) score += fuzzyScore;
                }
                return { skin, score };
            })
            .sort((a, b) => b.score - a.score)
            .map(({ skin }) => skin);

        return scoredSkins;
    }

    fuzzyMatch(text, pattern) {
        // Simple fuzzy matching implementation
        let patternIdx = 0;
        for (let i = 0; i < text.length; i++) {
            if (text[i] === pattern[patternIdx]) {
                patternIdx++;
                if (patternIdx === pattern.length) return pattern.length * 10; // Score based on match length
            }
        }
        return null;
    }

    updateSuggestions() {
        const inputParts = this.input.split(/\s+/);
        const lastPart = inputParts[inputParts.length - 1]?.toLowerCase() || "";
        this.suggestion = null;

        if (lastPart) {
            let bestScore = -Infinity;
            let bestTerm = null;

            for (const term of this.allTerms) {
                const score = this.fuzzyMatch(term, lastPart);
                if (score !== null && score > bestScore) {
                    bestScore = score;
                    bestTerm = term;
                }
            }

            this.suggestion = bestTerm;
            this.renderSuggestions();
        } else {
            this.suggestionsDiv.style.display = "none";
        }
    }

    renderSuggestions() {
        if (this.suggestion) {
            this.suggestionsDiv.innerHTML = `Suggestion: <span class="suggestion">${this.suggestion}</span>`;
            this.suggestionsDiv.style.display = "block";
        } else {
            this.suggestionsDiv.style.display = "none";
        }
    }

    renderResults() {
        this.resultsList.innerHTML = this.results
            .map((skin, index) => `
                <div class="result-item ${index === this.selectedIndex ? 'selected' : ''}" 
                     data-index="${index}">
                    <span class="rarity-${skin.rarity.toLowerCase()}">${skin.name}</span>
                    <span class="rarity-${skin.rarity.toLowerCase()}">${skin.rarity}</span>
                    <span>${skin.event}</span>
                    <span>${skin.year || 'N/A'}</span>
                </div>
            `)
            .join('');
    }

    updateDetails() {
        const skin = this.results[this.selectedIndex];
        if (!skin) {
            this.detailsPanel.innerHTML = "<div>No skin selected</div>";
            return;
        }

        this.detailsPanel.innerHTML = `
            <div><span class="detail-label">Name:</span>${skin.name}</div>
            <div><span class="detail-label">Rarity:</span>
                <span class="rarity-${skin.rarity.toLowerCase()}">${skin.rarity}</span>
            </div>
            <div><span class="detail-label">Event:</span>${skin.event}</div>
            <div><span class="detail-label">Year:</span>${skin.year || 'N/A'}</div>
            <div><span class="detail-label">Tags:</span>${skin.tags.join(', ')}</div>
        `;
    }

    handleKeys(e) {
        switch (e.key) {
            case 'ArrowDown':
                this.selectedIndex = Math.min(this.selectedIndex + 1, this.results.length - 1);
                this.updateResults();
                break;
            case 'ArrowUp':
                this.selectedIndex = Math.max(this.selectedIndex - 1, 0);
                this.updateResults();
                break;
            case 'Escape':
                this.searchInput.value = '';
                this.updateResults();
                break;
            case 'Tab':
                if (this.suggestion) {
                    e.preventDefault();
                    const parts = this.input.split(/\s+/);
                    parts[parts.length - 1] = this.suggestion;
                    this.searchInput.value = parts.join(' ') + ' ';
                    this.handleSearch();
                }
                break;
        }

        const selectedElement = this.resultsList.querySelector(`[data-index="${this.selectedIndex}"]`);
        selectedElement?.scrollIntoView({ block: 'nearest' });
    }

    handleRowClick(e) {
        const row = e.target.closest('.result-item');
        if (row) {
            this.selectedIndex = parseInt(row.dataset.index);
            this.updateResults();
        }
    }

    loadAllTerms() {
        const terms = new Set();
        for (const skin of this.skins) {
            terms.add(skin.name.toLowerCase());
            for (const word of skin.name.toLowerCase().split(/\s+/)) {
                terms.add(word);
            }
            terms.add(skin.rarity.toLowerCase());
            terms.add(skin.event.toLowerCase());
            for (const tag of skin.tags) {
                terms.add(tag.toLowerCase());
            }
            if (skin.year) terms.add(skin.year.toString());
        }
        return terms;
    }
}

// Initialize when ready
document.addEventListener('DOMContentLoaded', () => new SkinDatabase());
