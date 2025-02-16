class SkinDatabase {
    constructor() {
        this.skins = window.skinsData;
        this.allTerms = this.loadAllTerms();
        this.selectedIndex = 0;
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
        this.updateResults();
        this.updateSuggestions();
    }

    updateResults() {
        const query = this.searchInput.value.toLowerCase();
        const tags = new Set(query.split(/\s+/).filter(t => t));
        
        this.filteredSkins = this.searchSkins(tags);
        this.selectedIndex = Math.min(this.selectedIndex, this.filteredSkins.length - 1);
        this.renderResults();
        this.updateDetails();
    }

    searchSkins(tags) {
        if (tags.size === 0) return this.skins;
        
        return this.skins.filter(skin => 
            Array.from(tags).every(tag =>
                skin.name.toLowerCase().includes(tag) ||
                skin.rarity.toLowerCase() === tag ||
                skin.event.toLowerCase().includes(tag) ||
                skin.tags.some(t => t.toLowerCase() === tag) ||
                (skin.year && skin.year.toString() === tag)
            );
    }

    renderResults() {
        this.resultsList.innerHTML = this.filteredSkins
            .map((skin, index) => `
                <div class="result-item ${index === this.selectedIndex ? 'selected' : ''}" 
                     data-index="${index}">
                    <span class="rarity-${skin.rarity.toLowerCase()}">${skin.name}</span>
                    <span class="rarity-${skin.rarity.toLowerCase()}">${skin.rarity}</span>
                    <span>${skin.event}</span>
                    <span>${skin.year || 'N/A'}</span>
                </div>
            `).join('');
    }

    updateDetails() {
        const skin = this.filteredSkins[this.selectedIndex];
        if (!skin) return;
        
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

    updateSuggestions() {
        // Implement fuzzy suggestions logic here
    }

    handleKeys(e) {
        switch(e.key) {
            case 'ArrowDown':
                this.selectedIndex = Math.min(this.selectedIndex + 1, this.filteredSkins.length - 1);
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
        this.skins.forEach(skin => {
            terms.add(skin.name.toLowerCase());
            skin.name.toLowerCase().split(/\s+/).forEach(word => terms.add(word));
            terms.add(skin.rarity.toLowerCase());
            terms.add(skin.event.toLowerCase());
            skin.tags.forEach(tag => terms.add(tag.toLowerCase()));
            if (skin.year) terms.add(skin.year.toString());
        });
        return terms;
    }
}

// Initialize when ready
document.addEventListener('DOMContentLoaded', () => new SkinDatabase());
